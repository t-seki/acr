use anyhow::Context;

use crate::atcoder;
use crate::config;
use crate::workspace;
use crate::workspace::CurrentContext;

pub fn execute(problem: Option<String>) -> anyhow::Result<()> {
    let current = workspace::detect_current_context();
    let url = match current {
        CurrentContext::ProblemDir(ctx) => match problem.as_deref() {
            Some(_) => anyhow::bail!(
                "Cannot specify a problem from a problem directory. Move to the contest directory."
            ),
            None => ctx.problem_url,
        },
        CurrentContext::ContestDir(ctx) => match problem.as_deref() {
            Some(p) => workspace::detect_problem_dir_from(&ctx.contest_dir.join(p.to_lowercase()))
                .with_context(|| format!("Problem '{}' not found", p))?
                .problem_url,
            None => format!(
                "{}/contests/{}/tasks",
                atcoder::BASE_URL, ctx.contest_id
            ),
        },
        CurrentContext::Outside => {
            anyhow::bail!("Run this command from a problem or contest directory.")
        }
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
