use anyhow::Context;

use crate::atcoder::AtCoderClient;
use crate::config;
use crate::workspace;
use crate::workspace::CurrentContext;

pub async fn execute(args: Vec<String>, tests: bool, code: bool, deps: bool) -> anyhow::Result<()> {
    // Default to --tests if no flags given
    let do_tests = tests || (!code && !deps);
    let do_code = code;
    let do_deps = deps;

    // Resolve update targets based on current context
    let current = workspace::detect_current_context();
    let contexts: Vec<workspace::ProblemContext> = match current {
        CurrentContext::ProblemDir(ctx) => {
            if !args.is_empty() {
                anyhow::bail!(
                    "Cannot specify arguments from a problem directory. Move to the contest directory."
                );
            }
            vec![ctx]
        }
        CurrentContext::ContestDir(ctx) => {
            if args.is_empty() {
                workspace::list_contest_problems(&ctx.contest_dir)?
            } else {
                args.iter()
                    .map(|p| {
                        workspace::detect_problem_dir_from(&ctx.contest_dir.join(p.to_lowercase()))
                            .with_context(|| format!("Problem '{}' not found", p))
                    })
                    .collect::<anyhow::Result<Vec<_>>>()?
            }
        }
        CurrentContext::Outside => {
            if args.is_empty() {
                anyhow::bail!("Specify a contest ID, or run from a contest directory.");
            }
            let contest_id = &args[0];
            let ctx = workspace::find_contest_dir_by_id(contest_id)?;
            if args.len() == 1 {
                workspace::list_contest_problems(&ctx.contest_dir)?
            } else {
                args[1..]
                    .iter()
                    .map(|p| {
                        workspace::detect_problem_dir_from(&ctx.contest_dir.join(p.to_lowercase()))
                            .with_context(|| format!("Problem '{}' not found", p))
                    })
                    .collect::<anyhow::Result<Vec<_>>>()?
            }
        }
    };

    // --tests: re-fetch sample cases
    if do_tests {
        let session = config::session::load()?;
        if contexts.len() == 1 {
            let ctx = &contexts[0];
            let client = AtCoderClient::with_session(&session.revel_session)?;
            println!(
                "Fetching test cases for problem {}...",
                ctx.problem_alphabet.to_uppercase()
            );
            let cases = client
                .fetch_sample_cases(&ctx.contest_id, &ctx.task_screen_name)
                .await?;
            workspace::testcase::save(&ctx.problem_dir, &cases)?;
            if cases.is_empty() {
                eprintln!(
                    "Warning: No test cases found for problem {}.",
                    ctx.problem_alphabet.to_uppercase()
                );
            } else {
                println!("Saved {} test case(s).", cases.len());
            }
        } else {
            let pb = indicatif::ProgressBar::new(contexts.len() as u64);
            pb.set_style(
                indicatif::ProgressStyle::default_bar()
                    .template("{msg} [{bar:30}] {pos}/{len}")
                    .expect("valid template"),
            );
            pb.set_message("Fetching samples");

            let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(2));
            let mut handles = Vec::new();
            for ctx in &contexts {
                let client = AtCoderClient::with_session(&session.revel_session)?;
                let contest_id = ctx.contest_id.clone();
                let task_screen_name = ctx.task_screen_name.clone();
                let problem_dir = ctx.problem_dir.clone();
                let alphabet = ctx.problem_alphabet.clone();
                let pb = pb.clone();
                let semaphore = semaphore.clone();
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
            let mut results = Vec::new();
            for handle in handles {
                results.push(handle.await??);
            }
            pb.finish_and_clear();
            for (alphabet, count) in &results {
                if *count == 0 {
                    eprintln!(
                        "Warning: No test cases found for problem {}. Use `acr update -t {}` to retry.",
                        alphabet.to_uppercase(),
                        alphabet.to_lowercase(),
                    );
                } else {
                    println!(
                        "Fetched {} test case(s) for problem {}.",
                        count,
                        alphabet.to_uppercase()
                    );
                }
            }
        }
    }

    // --code: regenerate src/main.rs from template
    if do_code {
        let template = config::global::load_template()?;
        for ctx in &contexts {
            let main_rs = ctx.problem_dir.join("src/main.rs");
            std::fs::write(&main_rs, &template)
                .with_context(|| format!("Failed to write {}", main_rs.display()))?;
            println!(
                "Regenerated src/main.rs for problem {}.",
                ctx.problem_alphabet.to_uppercase()
            );
        }
    }

    // --deps: update Cargo.toml dependencies
    if do_deps {
        for ctx in &contexts {
            let cargo_toml_path = ctx.problem_dir.join("Cargo.toml");
            let content = std::fs::read_to_string(&cargo_toml_path)
                .with_context(|| format!("Failed to read {}", cargo_toml_path.display()))?;
            let doc: toml::Value =
                toml::from_str(&content).context("Failed to parse Cargo.toml")?;
            let name = doc
                .get("package")
                .and_then(|p| p.get("name"))
                .and_then(|n| n.as_str())
                .unwrap_or("");
            // name = "{contest_id}-{alphabet}"
            let (contest_id, alphabet) = name.split_once('-').unwrap_or(("", name));
            let new_content =
                workspace::generator::problem_cargo_toml(contest_id, alphabet, &ctx.problem_url);
            std::fs::write(&cargo_toml_path, new_content)
                .with_context(|| format!("Failed to write {}", cargo_toml_path.display()))?;
            println!(
                "Updated dependencies for problem {}.",
                ctx.problem_alphabet.to_uppercase()
            );
        }
    }

    Ok(())
}
