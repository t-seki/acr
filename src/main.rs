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
            todo!("acrs init")
        }
        Command::Login => {
            print!("Username: ");
            let mut username = String::new();
            std::io::Write::flush(&mut std::io::stdout())?;
            std::io::stdin().read_line(&mut username)?;
            let username = username.trim();

            eprint!("Password: ");
            let password = rpassword::read_password()?;

            let client = AtCoderClient::new()?;
            let revel_session = client.login(username, &password).await?;

            config::session::save(&config::session::SessionConfig { revel_session })?;
            println!("Logged in as {}.", username);
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
                None => println!("Session expired. Run `acrs login` again."),
            }
            Ok(())
        }
        Command::New { contest_id } => {
            let session = config::session::load()?;
            let client = AtCoderClient::with_session(&session.revel_session)?;

            // Check if directory already exists
            let cwd = std::env::current_dir()?;
            if cwd.join(&contest_id).exists() {
                return Err(error::AcrsError::ContestAlreadyExists(contest_id).into());
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

            let mut handles = Vec::new();
            for problem in &contest.problems {
                let client = AtCoderClient::with_session(&session.revel_session)?;
                let contest_id = contest_id.clone();
                let task_screen_name = problem.task_screen_name.clone();
                let problem_dir = workspace_dir.join(problem.alphabet.to_lowercase());
                let pb = pb.clone();
                handles.push(tokio::spawn(async move {
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
                .ok_or_else(|| error::AcrsError::ProblemNotFound(problem.clone()))?;

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
        Command::Test => {
            let ctx = workspace::detect_problem_dir()?;
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
                return Err(error::AcrsError::TestFailed {
                    passed,
                    total: results.len(),
                }
                .into());
            }
            Ok(())
        }
        Command::Submit => {
            todo!("acrs submit")
        }
        Command::Config { key: _, value: _ } => {
            todo!("acrs config")
        }
    }
}
