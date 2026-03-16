use anyhow::Context;

use crate::atcoder::AtCoderClient;
use crate::config;
use crate::error;
use crate::workspace;
use crate::workspace::CurrentContext;

async fn retry_with_backoff<T, F, Fut>(max_secs: u64, mut f: F) -> anyhow::Result<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = anyhow::Result<T>>,
{
    let start = std::time::Instant::now();
    let mut attempt = 0u32;
    loop {
        match f().await {
            Ok(v) => return Ok(v),
            Err(e) => {
                if start.elapsed().as_secs() >= max_secs {
                    return Err(e);
                }
                let delay = std::cmp::min(2u64.pow(attempt + 1), 15);
                attempt += 1;
                eprintln!(
                    "  Retrying in {}s... ({}s / {}s)",
                    delay,
                    start.elapsed().as_secs(),
                    max_secs,
                );
                tokio::time::sleep(std::time::Duration::from_secs(delay)).await;
            }
        }
    }
}

pub async fn execute(
    contest_id: String,
    problems: Vec<String>,
    at: Option<String>,
) -> anyhow::Result<()> {
    let current = workspace::detect_current_context();
    match current {
        CurrentContext::ProblemDir(_) | CurrentContext::ContestDir(_) => {
            anyhow::bail!(
                "Cannot create a new contest inside a problem or contest directory."
            );
        }
        CurrentContext::Outside => {}
    }

    // --at: wait until the specified time
    if let Some(ref time_str) = at {
        let target_time = chrono::NaiveTime::parse_from_str(time_str, "%H:%M")
            .or_else(|_| chrono::NaiveTime::parse_from_str(time_str, "%H:%M:%S"))
            .map_err(|_| {
                anyhow::anyhow!(
                    "Invalid time format: '{}'. Use HH:MM (e.g. 21:00)",
                    time_str
                )
            })?;

        let now = chrono::Local::now();
        let today = now.date_naive().and_time(target_time);
        let tomorrow = today + chrono::Duration::days(1);
        let yesterday = today - chrono::Duration::days(1);

        // Pick the nearest occurrence of HH:MM
        let now_naive = now.naive_local();
        let candidates = [yesterday, today, tomorrow];
        let target = candidates
            .iter()
            .min_by_key(|c| ((**c) - now_naive).abs())
            .unwrap();
        let target = target
            .and_local_timezone(chrono::Local)
            .single()
            .context("Failed to resolve target time")?;

        if target > now {
            let wait = target - now;
            println!(
                "Waiting until {} ({:.0}s remaining)... Press Ctrl+C to cancel.",
                target.format("%H:%M"),
                wait.num_seconds(),
            );
            loop {
                let remaining = target - chrono::Local::now();
                if remaining <= chrono::Duration::zero() {
                    break;
                }
                let secs = remaining.num_seconds();
                let mins = secs / 60;
                let secs = secs % 60;
                eprint!("\r\x1b[K  {}m {:02}s remaining...", mins, secs);
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
            eprintln!("\r\x1b[K  Time reached! Starting...");
        }
    }

    let session = config::session::load()?;
    let client = AtCoderClient::with_session(&session.revel_session)?;

    // Check if directory already exists
    let cwd = std::env::current_dir()?;
    if cwd.join(&contest_id).exists() {
        return Err(error::AcrError::ContestAlreadyExists(contest_id).into());
    }

    // Fetch contest info
    println!("Fetching contest info...");
    let contest = if at.is_some() {
        retry_with_backoff(60, || client.fetch_contest(&contest_id)).await?
    } else {
        client.fetch_contest(&contest_id).await?
    };

    // Filter target problems
    let filter: Vec<String> = problems.iter().map(|p| p.to_uppercase()).collect();
    let target_problems: Vec<crate::atcoder::Problem> = contest
        .problems
        .into_iter()
        .filter(|p| filter.is_empty() || filter.contains(&p.alphabet.to_uppercase()))
        .collect();

    if target_problems.is_empty() {
        anyhow::bail!("No matching problems found.");
    }

    // Load template and create workspace
    let template = config::global::load_template()?;
    let workspace_dir = workspace::generator::create_contest_workspace(
        &cwd,
        &contest_id,
        &target_problems,
        &template,
    )?;

    // Fetch sample cases in parallel
    let pb = indicatif::ProgressBar::new(target_problems.len() as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{msg} [{bar:30}] {pos}/{len}")
            .expect("valid template"),
    );
    pb.set_message("Fetching samples");

    let is_at_mode = at.is_some();
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(2));
    let mut handles = Vec::new();
    for problem in &target_problems {
        let client = AtCoderClient::with_session(&session.revel_session)?;
        let contest_id = contest_id.clone();
        let task_screen_name = problem.task_screen_name.clone();
        let problem_dir = workspace_dir.join(problem.alphabet.to_lowercase());
        let pb = pb.clone();
        let semaphore = semaphore.clone();
        let alphabet = problem.alphabet.clone();
        handles.push(tokio::spawn(async move {
            let _permit = semaphore.acquire().await?;
            let cases = if is_at_mode {
                retry_with_backoff(60, || {
                    client.fetch_sample_cases(&contest_id, &task_screen_name)
                })
                .await?
            } else {
                client
                    .fetch_sample_cases(&contest_id, &task_screen_name)
                    .await?
            };
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

    // --at mode: retry if all problems have 0 test cases
    if at.is_some() && warnings.len() == target_problems.len() && !target_problems.is_empty() {
        let max_retry_secs = 60;
        let retry_interval = 5;
        let start = std::time::Instant::now();
        eprintln!(
            "No test cases found yet. Retrying for up to {}s...",
            max_retry_secs
        );
        while start.elapsed().as_secs() < max_retry_secs {
            tokio::time::sleep(std::time::Duration::from_secs(retry_interval)).await;
            let mut any_found = false;
            for problem in &target_problems {
                let client = AtCoderClient::with_session(&session.revel_session)?;
                let cases = client
                    .fetch_sample_cases(&contest_id, &problem.task_screen_name)
                    .await;
                if let Ok(cases) = cases
                    && !cases.is_empty()
                {
                    workspace::testcase::save(
                        &workspace_dir.join(problem.alphabet.to_lowercase()),
                        &cases,
                    )?;
                    any_found = true;
                }
            }
            if any_found {
                // Fetch remaining problems
                for problem in &target_problems {
                    let problem_dir = workspace_dir.join(problem.alphabet.to_lowercase());
                    let existing = workspace::testcase::load(&problem_dir)?;
                    if existing.is_empty() {
                        let client = AtCoderClient::with_session(&session.revel_session)?;
                        if let Ok(cases) = client
                            .fetch_sample_cases(&contest_id, &problem.task_screen_name)
                            .await
                        {
                            workspace::testcase::save(&problem_dir, &cases)?;
                        }
                    }
                }
                warnings.clear();
                // Re-check for warnings
                for problem in &target_problems {
                    let problem_dir = workspace_dir.join(problem.alphabet.to_lowercase());
                    let cases = workspace::testcase::load(&problem_dir)?;
                    if cases.is_empty() {
                        warnings.push(problem.alphabet.clone());
                    }
                }
                eprintln!("Test cases fetched.");
                break;
            }
            let elapsed = start.elapsed().as_secs();
            eprint!(
                "\r\x1b[K  Retrying... ({}s / {}s)",
                elapsed, max_retry_secs
            );
        }
    }

    for alphabet in &warnings {
        eprintln!(
            "Warning: No test cases found for problem {}. Use `acr update -t {}` to retry.",
            alphabet.to_uppercase(),
            alphabet.to_lowercase(),
        );
    }

    // Open first problem in browser
    if let Some(first) = target_problems.first() {
        let browser = config::global::load()
            .map(|c| c.browser)
            .unwrap_or_else(|_| "xdg-open".to_string());
        let _ = std::process::Command::new(&browser)
            .arg(&first.url)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();
    }

    // Open editor
    let editor = config::global::load()
        .map(|c| c.editor)
        .unwrap_or_else(|_| "vim".to_string());
    let mut editor_cmd = std::process::Command::new(&editor);
    editor_cmd.arg(&workspace_dir);
    if let Some(first) = target_problems.first() {
        editor_cmd.arg(
            workspace_dir
                .join(first.alphabet.to_lowercase())
                .join("src/main.rs"),
        );
    }
    let _ = editor_cmd.spawn();

    println!("Created workspace: {}", workspace_dir.display());
    Ok(())
}
