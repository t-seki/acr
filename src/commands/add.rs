use crate::atcoder::AtCoderClient;
use crate::config;
use crate::error;
use crate::workspace;

pub async fn execute(problems: Vec<String>) -> anyhow::Result<()> {
    let contest_ctx = workspace::detect_contest_dir()?;
    let contest_dir = contest_ctx.contest_dir;
    let contest_id = contest_ctx.contest_id;
    let session = config::session::load()?;
    let client = AtCoderClient::with_session(&session.revel_session)?;

    let contest = client.fetch_contest(&contest_id).await?;

    // Determine target problems
    let targets: Vec<crate::atcoder::Problem> = if problems.is_empty() {
        // Add all missing problems
        contest
            .problems
            .into_iter()
            .filter(|p| !contest_dir.join(p.alphabet.to_lowercase()).exists())
            .collect()
    } else {
        // Add specified problems
        let mut result = Vec::new();
        for name in &problems {
            let p = contest
                .problems
                .iter()
                .find(|p| p.alphabet.to_uppercase() == name.to_uppercase())
                .ok_or_else(|| error::AcrError::ProblemNotFound(name.clone()))?
                .clone();
            result.push(p);
        }
        result
    };

    if targets.is_empty() {
        println!("All problems are already set up.");
        return Ok(());
    }

    let template = config::global::load_template()?;

    // Add problems and fetch sample cases in parallel
    let pb = indicatif::ProgressBar::new(targets.len() as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{msg} [{bar:30}] {pos}/{len}")
            .expect("valid template"),
    );
    pb.set_message("Fetching samples");

    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(2));
    let mut handles = Vec::new();
    for problem in &targets {
        workspace::generator::add_problem_to_workspace(
            &contest_dir,
            &contest_id,
            problem,
            &template,
        )?;

        let client = AtCoderClient::with_session(&session.revel_session)?;
        let contest_id = contest_id.clone();
        let task_screen_name = problem.task_screen_name.clone();
        let problem_dir = contest_dir.join(problem.alphabet.to_lowercase());
        let pb = pb.clone();
        let semaphore = semaphore.clone();
        let alphabet = problem.alphabet.clone();
        handles.push(tokio::spawn(async move {
            let _permit = semaphore.acquire().await?;
            let cases = client
                .fetch_sample_cases(&contest_id, &task_screen_name)
                .await?;
            let count = cases.len();
            workspace::testcase::save(&problem_dir, &cases)?;
            pb.inc(1);
            Ok::<(String, usize), anyhow::Error>((alphabet, count))
        }));
    }
    let mut warnings = Vec::new();
    for handle in handles {
        let (alphabet, count) = handle.await??;
        if count == 0 {
            warnings.push(alphabet);
        }
    }
    pb.finish_with_message("Done");
    for alphabet in &warnings {
        eprintln!(
            "Warning: No test cases found for problem {}. Use `acr update -t {}` to retry.",
            alphabet.to_uppercase(),
            alphabet.to_lowercase(),
        );
    }

    for problem in &targets {
        println!("Added problem {}.", problem.alphabet.to_uppercase());
    }
    Ok(())
}
