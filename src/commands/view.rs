use anyhow::Context;

use crate::atcoder;
use crate::config;
use crate::workspace;
use crate::workspace::CurrentContext;

pub fn execute(args: Vec<String>) -> anyhow::Result<()> {
    let current = workspace::detect_current_context();
    let url = match current {
        CurrentContext::ProblemDir(ctx) => {
            if !args.is_empty() {
                anyhow::bail!(
                    "Cannot specify arguments from a problem directory. Move to the contest directory."
                );
            }
            ctx.problem_url
        }
        CurrentContext::ContestDir(ctx) => match args.first().map(|s| s.as_str()) {
            Some(p) => {
                workspace::detect_problem_dir_from(&ctx.contest_dir.join(p.to_lowercase()))
                    .with_context(|| format!("Problem '{}' not found", p))?
                    .problem_url
            }
            None => format!(
                "{}/contests/{}/tasks",
                atcoder::BASE_URL,
                ctx.contest_id
            ),
        },
        CurrentContext::Outside => {
            if args.is_empty() {
                anyhow::bail!("Specify a contest ID, or run from a contest directory.");
            }
            let contest_id = &args[0];
            match args.get(1) {
                Some(problem) => format!(
                    "{}/contests/{}/tasks/{}_{}",
                    atcoder::BASE_URL,
                    contest_id,
                    contest_id,
                    problem.to_lowercase()
                ),
                None => format!(
                    "{}/contests/{}/tasks",
                    atcoder::BASE_URL,
                    contest_id
                ),
            }
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
