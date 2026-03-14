use crate::error;
use crate::runner;
use crate::workspace;

pub async fn execute(problem: Option<String>) -> anyhow::Result<()> {
    let ctx = workspace::require_problem_context(
        workspace::detect_current_context(),
        problem.as_deref(),
    )?;
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
