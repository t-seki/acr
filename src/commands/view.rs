use anyhow::Context;

use crate::atcoder;
use crate::config;
use crate::workspace;
use crate::workspace::CurrentContext;

fn resolve_url(current: CurrentContext, args: &[String]) -> anyhow::Result<String> {
    match current {
        CurrentContext::ProblemDir(ctx) => {
            if !args.is_empty() {
                anyhow::bail!(
                    "Cannot specify arguments from a problem directory. Move to the contest directory."
                );
            }
            Ok(ctx.problem_url)
        }
        CurrentContext::ContestDir(ctx) => match args.first().map(|s| s.as_str()) {
            Some(p) => {
                let problem_ctx =
                    workspace::detect_problem_dir_from(&ctx.contest_dir.join(p.to_lowercase()))
                        .with_context(|| format!("Problem '{}' not found", p))?;
                Ok(problem_ctx.problem_url)
            }
            None => Ok(format!(
                "{}/contests/{}/tasks",
                atcoder::BASE_URL,
                ctx.contest_id
            )),
        },
        CurrentContext::Outside => {
            if args.is_empty() {
                anyhow::bail!("Specify a contest ID, or run from a contest directory.");
            }
            let contest_id = &args[0];
            match args.get(1) {
                Some(problem) => Ok(format!(
                    "{}/contests/{}/tasks/{}_{}",
                    atcoder::BASE_URL,
                    contest_id,
                    contest_id,
                    problem.to_lowercase()
                )),
                None => Ok(format!(
                    "{}/contests/{}/tasks",
                    atcoder::BASE_URL,
                    contest_id
                )),
            }
        }
    }
}

pub fn execute(args: Vec<String>) -> anyhow::Result<()> {
    let current = workspace::detect_current_context();
    let url = resolve_url(current, &args)?;
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

    fn make_problem_context(contest_id: &str, problem: &str) -> ProblemContext {
        ProblemContext {
            contest_id: contest_id.to_string(),
            problem_alphabet: problem.to_string(),
            task_screen_name: format!("{}_{}", contest_id, problem),
            problem_dir: PathBuf::from(format!("/tmp/{}/{}", contest_id, problem)),
            problem_url: format!(
                "https://atcoder.jp/contests/{}/tasks/{}_{}",
                contest_id, contest_id, problem
            ),
        }
    }

    #[test]
    fn test_resolve_url_problem_dir_no_args() {
        let ctx = CurrentContext::ProblemDir(make_problem_context("abc001", "a"));
        let url = resolve_url(ctx, &[]).unwrap();
        assert_eq!(url, "https://atcoder.jp/contests/abc001/tasks/abc001_a");
    }

    #[test]
    fn test_resolve_url_problem_dir_with_args_error() {
        let ctx = CurrentContext::ProblemDir(make_problem_context("abc001", "a"));
        assert!(resolve_url(ctx, &["b".to_string()]).is_err());
    }

    #[test]
    fn test_resolve_url_contest_dir_no_args() {
        let dir = tempfile::tempdir().unwrap();
        let ws = dir.path().join("abc001");
        std::fs::create_dir_all(&ws).unwrap();
        std::fs::write(
            ws.join("Cargo.toml"),
            "[workspace]\nmembers = [\"a\"]\nresolver = \"2\"\n",
        )
        .unwrap();
        let ctx = CurrentContext::ContestDir(ContestContext {
            contest_id: "abc001".to_string(),
            contest_dir: ws,
        });
        let url = resolve_url(ctx, &[]).unwrap();
        assert_eq!(url, "https://atcoder.jp/contests/abc001/tasks");
    }

    #[test]
    fn test_resolve_url_contest_dir_with_problem() {
        let dir = tempfile::tempdir().unwrap();
        let ws = dir.path().join("abc001");
        let problem_dir = ws.join("a");
        std::fs::create_dir_all(&problem_dir).unwrap();
        std::fs::write(
            ws.join("Cargo.toml"),
            "[workspace]\nmembers = [\"a\"]\nresolver = \"2\"\n",
        )
        .unwrap();
        std::fs::write(
            problem_dir.join("Cargo.toml"),
            r#"[package]
name = "abc001-a"
version = "0.1.0"
edition = "2021"

[package.metadata.acr]
problem_url = "https://atcoder.jp/contests/abc001/tasks/abc001_a"
"#,
        )
        .unwrap();
        let ctx = CurrentContext::ContestDir(ContestContext {
            contest_id: "abc001".to_string(),
            contest_dir: ws,
        });
        let url = resolve_url(ctx, &["a".to_string()]).unwrap();
        assert_eq!(url, "https://atcoder.jp/contests/abc001/tasks/abc001_a");
    }

    #[test]
    fn test_resolve_url_outside_contest_only() {
        let ctx = CurrentContext::Outside;
        let url = resolve_url(ctx, &["abc001".to_string()]).unwrap();
        assert_eq!(url, "https://atcoder.jp/contests/abc001/tasks");
    }

    #[test]
    fn test_resolve_url_outside_contest_and_problem() {
        let ctx = CurrentContext::Outside;
        let url = resolve_url(ctx, &["abc001".to_string(), "a".to_string()]).unwrap();
        assert_eq!(url, "https://atcoder.jp/contests/abc001/tasks/abc001_a");
    }

    #[test]
    fn test_resolve_url_outside_no_args_error() {
        let ctx = CurrentContext::Outside;
        assert!(resolve_url(ctx, &[]).is_err());
    }
}
