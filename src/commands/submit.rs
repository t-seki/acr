use std::time::Duration;

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

use crate::atcoder::{self, AtCoderClient};
use crate::browser;
use crate::config;
use crate::error;
use crate::runner;
use crate::workspace;

pub async fn execute(
    problem: Option<String>,
    force: bool,
    web: bool,
    language_id: Option<String>,
) -> anyhow::Result<()> {
    let ctx = workspace::require_problem_context(
        workspace::detect_current_context(),
        problem.as_deref(),
    )?;
    let test_cases = workspace::testcase::load(&ctx.problem_dir)?;

    // Run tests first
    if !test_cases.is_empty() {
        let results = runner::tester::run_all(&ctx.problem_dir, &test_cases).await?;
        runner::tester::display_results(&results);

        let passed = results
            .iter()
            .filter(|(_, r)| matches!(r, runner::TestResult::Ac))
            .count();
        if passed < results.len() && !force {
            return Err(error::AcrError::TestFailed {
                passed,
                total: results.len(),
            }
            .into());
        }
    }

    let source = std::fs::read_to_string(ctx.problem_dir.join("src/main.rs"))?;

    if web {
        submit_via_browser(&ctx, &source);
        return Ok(());
    }

    submit_via_api(&ctx, &source, language_id).await
}

fn submit_via_browser(ctx: &workspace::ProblemContext, source: &str) {
    let copied = arboard::Clipboard::new()
        .and_then(|mut cb| cb.set_text(source))
        .is_ok();

    if copied {
        println!("\nSource code copied to clipboard.");
    } else {
        println!("\nCould not copy to clipboard. Please copy src/main.rs manually.");
    }

    let submit_url = format!(
        "{}/contests/{}/submit?taskScreenName={}",
        atcoder::BASE_URL,
        ctx.contest_id,
        ctx.task_screen_name
    );
    println!("Opening submit page...");
    println!("Paste your code and submit: {}", submit_url);

    browser::open(&submit_url);
}

async fn submit_via_api(
    ctx: &workspace::ProblemContext,
    source: &str,
    language_id_override: Option<String>,
) -> anyhow::Result<()> {
    let session = config::session::load()?;
    let client = AtCoderClient::with_session(&session.revel_session)?;

    let language_id = match language_id_override {
        Some(id) => id,
        None => config::global::load()?.language_id,
    };

    println!("\nFetching CSRF token...");
    let csrf = client
        .fetch_csrf_token(&ctx.contest_id, &ctx.task_screen_name)
        .await?;

    println!("Submitting (language_id={})...", language_id);
    let submission_id = client
        .submit_source(
            &ctx.contest_id,
            &ctx.task_screen_name,
            &language_id,
            source,
            &csrf,
        )
        .await?;
    let submission_url = format!(
        "{}/contests/{}/submissions/{}",
        atcoder::BASE_URL,
        ctx.contest_id,
        submission_id
    );
    println!("Submitted: {}", submission_url);

    poll_until_judged(&client, &ctx.contest_id, submission_id).await
}

async fn poll_until_judged(
    client: &AtCoderClient,
    contest_id: &str,
    submission_id: u64,
) -> anyhow::Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner} {msg}")
            .expect("valid template"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("Waiting for judge...");

    // AtCoder needs a brief moment to register the submission before status JSON
    // returns the new entry, so wait a bit before the first poll.
    tokio::time::sleep(Duration::from_millis(500)).await;

    loop {
        let status = client
            .fetch_submission_status(contest_id, submission_id)
            .await?;
        pb.set_message(format!("Judging: {}", status.label));

        if status.finished {
            pb.finish_and_clear();
            print_result(&status.label);
            return Ok(());
        }

        let interval = status.interval_ms.unwrap_or(2000).max(500);
        tokio::time::sleep(Duration::from_millis(interval)).await;
    }
}

fn print_result(label: &str) {
    let colored_label = match label {
        "AC" => label.green().bold(),
        "CE" => label.yellow().bold(),
        _ => label.red().bold(),
    };
    println!("Result: {}", colored_label);
}
