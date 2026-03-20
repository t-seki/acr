use crate::atcoder;
use crate::browser;
use crate::error;
use crate::runner;
use crate::workspace;

pub async fn execute(problem: Option<String>, force: bool) -> anyhow::Result<()> {
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

    browser::open(&submit_url);

    Ok(())
}
