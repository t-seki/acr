mod atcoder;
mod cli;
mod config;
mod error;
mod runner;
mod workspace;

use atcoder::AtCoderClient;
use clap::Parser;
use cli::{Cli, Command};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init => {
            let config_dir = config::config_dir()?;
            std::fs::create_dir_all(&config_dir)?;

            // config.toml
            let config_path = config_dir.join("config.toml");
            if config_path.exists() {
                println!("config.toml already exists, skipping.");
            } else {
                print!("Editor [vim]: ");
                std::io::Write::flush(&mut std::io::stdout())?;
                let mut editor = String::new();
                std::io::stdin().read_line(&mut editor)?;
                let editor = editor.trim();
                let editor = if editor.is_empty() {
                    "vim".to_string()
                } else {
                    editor.to_string()
                };

                let cfg = config::global::GlobalConfig {
                    editor,
                    browser: "xdg-open".to_string(),
                };
                config::global::save(&cfg)?;
                println!("Created config.toml");
            }

            // template.rs
            let template_path = config::global::template_path()?;
            if template_path.exists() {
                println!("template.rs already exists, skipping.");
            } else {
                std::fs::write(&template_path, config::global::default_template())?;
                println!("Created template.rs");
            }

            // .cargo/config.toml (shared target directory)
            let cargo_config_dir = std::env::current_dir()?.join(".cargo");
            let cargo_config_path = cargo_config_dir.join("config.toml");
            if cargo_config_path.exists() {
                println!(".cargo/config.toml already exists, skipping.");
            } else {
                std::fs::create_dir_all(&cargo_config_dir)?;
                std::fs::write(&cargo_config_path, "[build]\ntarget-dir = \"target\"\n")?;
                println!("Created .cargo/config.toml");
            }

            // .gitignore
            let gitignore_path = std::env::current_dir()?.join(".gitignore");
            if gitignore_path.exists() {
                println!(".gitignore already exists, skipping.");
            } else {
                std::fs::write(&gitignore_path, "/target\n")?;
                println!("Created .gitignore");
            }

            println!("Initialization complete!");
            Ok(())
        }
        Command::Login => {
            // Open AtCoder login page in browser
            let login_url = "https://atcoder.jp/login";
            let browser = config::global::load()
                .map(|c| c.browser)
                .unwrap_or_else(|_| "xdg-open".to_string());
            let _ = std::process::Command::new(&browser)
                .arg(login_url)
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn();

            println!("Opening AtCoder login page in your browser...");
            println!();
            println!("After logging in, please copy the REVEL_SESSION cookie value:");
            println!("  1. Open DevTools (F12)");
            println!("  2. Go to Application tab > Cookies > https://atcoder.jp");
            println!("  3. Find REVEL_SESSION and copy its value");
            println!();

            // Read REVEL_SESSION from stdin
            print!("REVEL_SESSION: ");
            std::io::Write::flush(&mut std::io::stdout())?;
            let mut revel_session = String::new();
            std::io::stdin().read_line(&mut revel_session)?;
            let revel_session = revel_session.trim().to_string();

            if revel_session.is_empty() {
                anyhow::bail!("REVEL_SESSION cannot be empty.");
            }

            // Validate session
            println!("Validating session...");
            let client = AtCoderClient::with_session(&revel_session)?;
            match client.check_session().await? {
                Some(username) => {
                    config::session::save(&config::session::SessionConfig { revel_session })?;
                    println!("Logged in as {}.", username);
                }
                None => {
                    anyhow::bail!(
                        "Invalid or expired session. Please make sure you are logged in to AtCoder and copied the correct REVEL_SESSION value."
                    );
                }
            }
            Ok(())
        }
        Command::Logout => {
            config::session::delete()?;
            println!("Logged out.");
            Ok(())
        }
        Command::Session => {
            let session = config::session::load()?;
            let client = AtCoderClient::with_session(&session.revel_session)?;
            match client.check_session().await? {
                Some(username) => println!("Logged in as {}.", username),
                None => println!("Session expired. Run `acr login` again."),
            }
            Ok(())
        }
        Command::New { contest_id } => {
            let session = config::session::load()?;
            let client = AtCoderClient::with_session(&session.revel_session)?;

            // Check if directory already exists
            let cwd = std::env::current_dir()?;
            if cwd.join(&contest_id).exists() {
                return Err(error::AcrError::ContestAlreadyExists(contest_id).into());
            }

            // Fetch contest info
            println!("Fetching contest info...");
            let contest = client.fetch_contest(&contest_id).await?;

            // Load template and create workspace
            let template = config::global::load_template()?;
            let workspace_dir =
                workspace::generator::create_contest_workspace(&cwd, &contest_id, &contest.problems, &template)?;

            // Fetch sample cases in parallel
            let pb = indicatif::ProgressBar::new(contest.problems.len() as u64);
            pb.set_style(
                indicatif::ProgressStyle::default_bar()
                    .template("{msg} [{bar:30}] {pos}/{len}")
                    .expect("valid template"),
            );
            pb.set_message("Fetching samples");

            let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(2));
            let mut handles = Vec::new();
            for problem in &contest.problems {
                let client = AtCoderClient::with_session(&session.revel_session)?;
                let contest_id = contest_id.clone();
                let task_screen_name = problem.task_screen_name.clone();
                let problem_dir = workspace_dir.join(problem.alphabet.to_lowercase());
                let pb = pb.clone();
                let semaphore = semaphore.clone();
                handles.push(tokio::spawn(async move {
                    let _permit = semaphore.acquire().await?;
                    let cases = client
                        .fetch_sample_cases(&contest_id, &task_screen_name)
                        .await?;
                    workspace::testcase::save(&problem_dir, &cases)?;
                    pb.inc(1);
                    Ok::<(), anyhow::Error>(())
                }));
            }
            for handle in handles {
                handle.await??;
            }
            pb.finish_with_message("Done");

            // Open editor
            let editor = config::global::load()
                .map(|c| c.editor)
                .unwrap_or_else(|_| "vim".to_string());
            let _ = std::process::Command::new(&editor)
                .arg(&workspace_dir)
                .spawn();

            println!("Created workspace: {}", workspace_dir.display());
            Ok(())
        }
        Command::Add { problem } => {
            let (contest_dir, contest_id) = workspace::detect_contest_dir()?;
            let session = config::session::load()?;
            let client = AtCoderClient::with_session(&session.revel_session)?;

            let contest = client.fetch_contest(&contest_id).await?;
            let target = contest
                .problems
                .iter()
                .find(|p| p.alphabet.to_lowercase() == problem.to_lowercase())
                .ok_or_else(|| error::AcrError::ProblemNotFound(problem.clone()))?;

            let template = config::global::load_template()?;
            workspace::generator::add_problem_to_workspace(
                &contest_dir,
                &contest_id,
                target,
                &template,
            )?;

            let cases = client
                .fetch_sample_cases(&contest_id, &target.task_screen_name)
                .await?;
            workspace::testcase::save(&contest_dir.join(problem.to_lowercase()), &cases)?;

            println!("Added problem {}.", problem.to_uppercase());
            Ok(())
        }
        Command::Fetch { problem } => {
            let ctx = workspace::resolve_problem_context(problem.as_deref())?;
            let session = config::session::load()?;
            let client = AtCoderClient::with_session(&session.revel_session)?;

            println!("Fetching test cases for problem {}...", ctx.problem_alphabet.to_uppercase());
            let cases = client
                .fetch_sample_cases(&ctx.contest_id, &ctx.task_screen_name)
                .await?;
            workspace::testcase::save(&ctx.problem_dir, &cases)?;
            println!("Saved {} test case(s).", cases.len());
            Ok(())
        }
        Command::View { problem } => {
            let url = if let Some(ref problem) = problem {
                let ctx = workspace::resolve_problem_context(Some(problem))?;
                ctx.problem_url
            } else if let Ok(ctx) = workspace::detect_problem_dir() {
                ctx.problem_url
            } else {
                let (_, contest_id) = workspace::detect_contest_dir()?;
                format!("{}/contests/{}/tasks", atcoder::BASE_URL, contest_id)
            };
            let browser = config::global::load()
                .map(|c| c.browser)
                .unwrap_or_else(|_| "xdg-open".to_string());
            let _ = std::process::Command::new(&browser)
                .arg(&url)
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn();
            println!("{}", url);
            Ok(())
        }
        Command::Test { problem } => {
            let ctx = workspace::resolve_problem_context(problem.as_deref())?;
            let test_cases = workspace::testcase::load(&ctx.problem_dir)?;

            if test_cases.is_empty() {
                println!("No test cases found.");
                return Ok(());
            }

            let results = runner::tester::run_all(&ctx.problem_dir, &test_cases).await?;
            runner::tester::display_results(&results);

            let passed = results
                .iter()
                .filter(|(_, r)| matches!(r, runner::TestResult::Ac))
                .count();
            if passed < results.len() {
                return Err(error::AcrError::TestFailed {
                    passed,
                    total: results.len(),
                }
                .into());
            }
            Ok(())
        }
        Command::Submit { problem } => {
            let ctx = workspace::resolve_problem_context(problem.as_deref())?;
            let test_cases = workspace::testcase::load(&ctx.problem_dir)?;

            // Run tests first
            if !test_cases.is_empty() {
                let results = runner::tester::run_all(&ctx.problem_dir, &test_cases).await?;
                runner::tester::display_results(&results);

                let passed = results
                    .iter()
                    .filter(|(_, r)| matches!(r, runner::TestResult::Ac))
                    .count();
                if passed < results.len() {
                    return Err(error::AcrError::TestFailed {
                        passed,
                        total: results.len(),
                    }
                    .into());
                }
            }

            // Read source code and copy to clipboard
            let source = std::fs::read_to_string(ctx.problem_dir.join("src/main.rs"))?;

            let copied = arboard::Clipboard::new()
                .and_then(|mut cb| cb.set_text(&source))
                .is_ok();

            if copied {
                println!("\nSource code copied to clipboard.");
            } else {
                println!("\nCould not copy to clipboard. Please copy src/main.rs manually.");
            }

            // Open submit page in browser
            let submit_url = format!(
                "{}/contests/{}/submit?taskScreenName={}",
                atcoder::BASE_URL, ctx.contest_id, ctx.task_screen_name
            );
            println!("Opening submit page...");
            println!("Paste your code and submit: {}", submit_url);

            let browser = config::global::load()
                .map(|c| c.browser)
                .unwrap_or_else(|_| "xdg-open".to_string());
            let _ = std::process::Command::new(&browser)
                .arg(&submit_url)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn();

            Ok(())
        }
        Command::Config { key, value } => {
            match (key, value) {
                (None, None) => {
                    let cfg = config::global::load()?;
                    println!("editor = {}", cfg.editor);
                    println!("browser = {}", cfg.browser);
                    Ok(())
                }
                (Some(key), None) => {
                    let cfg = config::global::load()?;
                    match key.as_str() {
                        "editor" => println!("{}", cfg.editor),
                        "browser" => println!("{}", cfg.browser),
                        _ => eprintln!("Unknown config key: {}", key),
                    }
                    Ok(())
                }
                (Some(key), Some(value)) => {
                    let mut cfg = config::global::load()?;
                    match key.as_str() {
                        "editor" => cfg.editor = value,
                        "browser" => cfg.browser = value,
                        _ => anyhow::bail!("Unknown config key: {}", key),
                    }
                    config::global::save(&cfg)?;
                    println!("Updated {}.", key);
                    Ok(())
                }
                (None, Some(_)) => unreachable!(),
            }
        }
    }
}
