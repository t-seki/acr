use crate::atcoder;
use crate::config;
use crate::workspace;
use crate::workspace::CurrentContext;

pub fn execute(contest_id: Option<String>) -> anyhow::Result<()> {
    let current = workspace::detect_current_context();
    let contest_id = match current {
        CurrentContext::ProblemDir(ctx) => match contest_id {
            Some(_) => anyhow::bail!("Cannot specify a contest ID from a problem directory."),
            None => ctx.contest_id,
        },
        CurrentContext::ContestDir(ctx) => match contest_id {
            Some(_) => anyhow::bail!("Cannot specify a contest ID from a contest directory."),
            None => ctx.contest_id,
        },
        CurrentContext::Outside => match contest_id {
            Some(id) => {
                workspace::find_contest_dir_by_id(&id)?;
                id
            }
            None => anyhow::bail!("Specify a contest ID or run from a contest directory."),
        },
    };
    let url = format!(
        "{}/contests/{}/submissions/me",
        atcoder::BASE_URL, contest_id
    );
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
