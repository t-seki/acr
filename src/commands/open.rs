use std::path::PathBuf;

use anyhow::Context;

use crate::commands::workspace_launcher;
use crate::workspace;
use crate::workspace::{ContestContext, CurrentContext};

/// Resolved target for `acr open`.
#[derive(Debug, PartialEq)]
struct OpenTarget {
    workspace_dir: PathBuf,
    problem_url: Option<String>,
    problem_main_rs: Option<PathBuf>,
}

fn resolve_target(current: CurrentContext, args: &[String]) -> anyhow::Result<OpenTarget> {
    match current {
        CurrentContext::ProblemDir(ctx) => {
            if !args.is_empty() {
                anyhow::bail!(
                    "Cannot specify arguments from a problem directory. Move to the contest directory."
                );
            }
            let workspace_dir = ctx
                .problem_dir
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| ctx.problem_dir.clone());
            let main_rs = ctx.problem_dir.join("src/main.rs");
            Ok(OpenTarget {
                workspace_dir,
                problem_url: Some(ctx.problem_url),
                problem_main_rs: Some(main_rs),
            })
        }
        CurrentContext::ContestDir(ctx) => resolve_from_contest(ctx, args),
        CurrentContext::Outside => {
            let contest_id = args.first().ok_or_else(|| {
                anyhow::anyhow!("Specify a contest ID, or run from a contest or problem directory.")
            })?;
            let contest_ctx = workspace::find_contest_dir_by_id(contest_id)?;
            resolve_from_contest(contest_ctx, &args[1..])
        }
    }
}

fn resolve_from_contest(ctx: ContestContext, args: &[String]) -> anyhow::Result<OpenTarget> {
    let problem = match args.first() {
        Some(name) => {
            workspace::detect_problem_dir_from(&ctx.contest_dir.join(name.to_lowercase()))
                .with_context(|| format!("Problem '{}' not found", name))?
        }
        None => workspace::list_contest_problems(&ctx.contest_dir)?
            .into_iter()
            .next()
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "No problems found in {}. Run `acr add` first.",
                    ctx.contest_dir.display()
                )
            })?,
    };
    let main_rs = problem.problem_dir.join("src/main.rs");
    Ok(OpenTarget {
        workspace_dir: ctx.contest_dir,
        problem_url: Some(problem.problem_url),
        problem_main_rs: Some(main_rs),
    })
}

pub fn execute(args: Vec<String>) -> anyhow::Result<()> {
    let current = workspace::detect_current_context();
    let target = resolve_target(current, &args)?;
    workspace_launcher::launch_workspace(
        &target.workspace_dir,
        target.problem_url.as_deref(),
        target.problem_main_rs.as_deref(),
    );
    if let Some(url) = target.problem_url.as_deref() {
        println!("Opening {}", url);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::{ContestContext, ProblemContext};

    fn make_problem_context(contest_id: &str, problem: &str, dir: PathBuf) -> ProblemContext {
        ProblemContext {
            contest_id: contest_id.to_string(),
            problem_alphabet: problem.to_string(),
            task_screen_name: format!("{}_{}", contest_id, problem),
            problem_dir: dir,
            problem_url: format!(
                "https://atcoder.jp/contests/{}/tasks/{}_{}",
                contest_id, contest_id, problem
            ),
        }
    }

    fn write_problem(contest_dir: &std::path::Path, contest_id: &str, alphabet: &str) {
        let pdir = contest_dir.join(alphabet);
        std::fs::create_dir_all(pdir.join("src")).unwrap();
        std::fs::write(
            pdir.join("Cargo.toml"),
            format!(
                r#"[package]
name = "{contest_id}-{alphabet}"
version = "0.1.0"
edition = "2021"

[package.metadata.acr]
problem_url = "https://atcoder.jp/contests/{contest_id}/tasks/{contest_id}_{alphabet}"
"#
            ),
        )
        .unwrap();
    }

    #[test]
    fn test_resolve_target_problem_dir_no_args() {
        let dir = tempfile::tempdir().unwrap();
        let contest = dir.path().join("abc001");
        let pdir = contest.join("a");
        std::fs::create_dir_all(&pdir).unwrap();
        let ctx = CurrentContext::ProblemDir(make_problem_context("abc001", "a", pdir.clone()));

        let target = resolve_target(ctx, &[]).unwrap();
        assert_eq!(target.workspace_dir, contest);
        assert_eq!(
            target.problem_url.as_deref(),
            Some("https://atcoder.jp/contests/abc001/tasks/abc001_a")
        );
        assert_eq!(target.problem_main_rs, Some(pdir.join("src/main.rs")));
    }

    #[test]
    fn test_resolve_target_problem_dir_with_args_error() {
        let ctx = CurrentContext::ProblemDir(make_problem_context(
            "abc001",
            "a",
            PathBuf::from("/tmp/abc001/a"),
        ));
        assert!(resolve_target(ctx, &["b".to_string()]).is_err());
    }

    #[test]
    fn test_resolve_target_contest_dir_no_args() {
        let dir = tempfile::tempdir().unwrap();
        let contest = dir.path().join("abc001");
        std::fs::create_dir_all(&contest).unwrap();
        write_problem(&contest, "abc001", "a");
        write_problem(&contest, "abc001", "b");

        let ctx = CurrentContext::ContestDir(ContestContext {
            contest_id: "abc001".to_string(),
            contest_dir: contest.clone(),
        });
        let target = resolve_target(ctx, &[]).unwrap();
        assert_eq!(target.workspace_dir, contest);
        assert_eq!(
            target.problem_url.as_deref(),
            Some("https://atcoder.jp/contests/abc001/tasks/abc001_a")
        );
        assert_eq!(
            target.problem_main_rs,
            Some(contest.join("a").join("src/main.rs"))
        );
    }

    #[test]
    fn test_resolve_target_contest_dir_with_problem() {
        let dir = tempfile::tempdir().unwrap();
        let contest = dir.path().join("abc001");
        std::fs::create_dir_all(&contest).unwrap();
        write_problem(&contest, "abc001", "a");
        write_problem(&contest, "abc001", "b");

        let ctx = CurrentContext::ContestDir(ContestContext {
            contest_id: "abc001".to_string(),
            contest_dir: contest.clone(),
        });
        let target = resolve_target(ctx, &["b".to_string()]).unwrap();
        assert_eq!(target.workspace_dir, contest);
        assert_eq!(
            target.problem_url.as_deref(),
            Some("https://atcoder.jp/contests/abc001/tasks/abc001_b")
        );
        assert_eq!(
            target.problem_main_rs,
            Some(contest.join("b").join("src/main.rs"))
        );
    }

    #[test]
    fn test_resolve_target_contest_dir_missing_problem_error() {
        let dir = tempfile::tempdir().unwrap();
        let contest = dir.path().join("abc001");
        std::fs::create_dir_all(&contest).unwrap();

        let ctx = CurrentContext::ContestDir(ContestContext {
            contest_id: "abc001".to_string(),
            contest_dir: contest,
        });
        assert!(resolve_target(ctx, &["z".to_string()]).is_err());
    }

    #[test]
    fn test_resolve_target_outside_no_args_error() {
        assert!(resolve_target(CurrentContext::Outside, &[]).is_err());
    }
}
