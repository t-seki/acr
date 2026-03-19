use crate::atcoder;
use crate::config;
use crate::workspace;
use crate::workspace::CurrentContext;

fn resolve_contest_id(
    current: CurrentContext,
    contest_id: Option<String>,
) -> anyhow::Result<String> {
    match current {
        CurrentContext::ProblemDir(ctx) => match contest_id {
            Some(_) => anyhow::bail!("Cannot specify a contest ID from a problem directory."),
            None => Ok(ctx.contest_id),
        },
        CurrentContext::ContestDir(ctx) => match contest_id {
            Some(_) => anyhow::bail!("Cannot specify a contest ID from a contest directory."),
            None => Ok(ctx.contest_id),
        },
        CurrentContext::Outside => match contest_id {
            Some(id) => {
                workspace::find_contest_dir_by_id(&id)?;
                Ok(id)
            }
            None => anyhow::bail!("Specify a contest ID or run from a contest directory."),
        },
    }
}

fn build_url(contest_id: &str) -> String {
    format!(
        "{}/contests/{}/submissions/me",
        atcoder::BASE_URL,
        contest_id
    )
}

pub fn execute(contest_id: Option<String>) -> anyhow::Result<()> {
    let current = workspace::detect_current_context();
    let contest_id = resolve_contest_id(current, contest_id)?;
    let url = build_url(&contest_id);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::{ContestContext, ProblemContext};
    use std::path::PathBuf;

    fn make_problem_context(contest_id: &str) -> ProblemContext {
        ProblemContext {
            contest_id: contest_id.to_string(),
            problem_alphabet: "a".to_string(),
            task_screen_name: format!("{}_a", contest_id),
            problem_dir: PathBuf::from(format!("/tmp/{}/a", contest_id)),
            problem_url: format!(
                "https://atcoder.jp/contests/{}/tasks/{}_a",
                contest_id, contest_id
            ),
        }
    }

    fn make_contest_context(contest_id: &str) -> ContestContext {
        ContestContext {
            contest_id: contest_id.to_string(),
            contest_dir: PathBuf::from(format!("/tmp/{}", contest_id)),
        }
    }

    #[test]
    fn test_resolve_contest_id_problem_dir() {
        let ctx = CurrentContext::ProblemDir(make_problem_context("abc001"));
        let id = resolve_contest_id(ctx, None).unwrap();
        assert_eq!(id, "abc001");
    }

    #[test]
    fn test_resolve_contest_id_problem_dir_with_id_error() {
        let ctx = CurrentContext::ProblemDir(make_problem_context("abc001"));
        assert!(resolve_contest_id(ctx, Some("abc002".to_string())).is_err());
    }

    #[test]
    fn test_resolve_contest_id_contest_dir() {
        let ctx = CurrentContext::ContestDir(make_contest_context("abc001"));
        let id = resolve_contest_id(ctx, None).unwrap();
        assert_eq!(id, "abc001");
    }

    #[test]
    fn test_resolve_contest_id_contest_dir_with_id_error() {
        let ctx = CurrentContext::ContestDir(make_contest_context("abc001"));
        assert!(resolve_contest_id(ctx, Some("abc002".to_string())).is_err());
    }

    #[test]
    fn test_resolve_contest_id_outside_no_id_error() {
        let ctx = CurrentContext::Outside;
        assert!(resolve_contest_id(ctx, None).is_err());
    }

    #[test]
    fn test_build_url() {
        assert_eq!(
            build_url("abc001"),
            "https://atcoder.jp/contests/abc001/submissions/me"
        );
    }
}
