pub mod generator;
pub mod testcase;

use std::path::PathBuf;

use anyhow::Context;

/// Context for the current problem directory.
#[derive(Debug)]
pub struct ProblemContext {
    pub contest_id: String,
    pub problem_alphabet: String,
    pub task_screen_name: String,
    pub problem_dir: PathBuf,
    pub problem_url: String,
}

/// Context for a contest workspace directory.
#[derive(Debug)]
pub struct ContestContext {
    pub contest_id: String,
    pub contest_dir: PathBuf,
}

/// The detected context based on the current working directory.
pub enum CurrentContext {
    ProblemDir(ProblemContext),
    ContestDir(ContestContext),
    Outside,
}

/// Detect the current context from the working directory.
pub fn detect_current_context() -> CurrentContext {
    if let Ok(ctx) = detect_problem_dir() {
        CurrentContext::ProblemDir(ctx)
    } else if let Ok(ctx) = detect_contest_dir() {
        CurrentContext::ContestDir(ctx)
    } else {
        CurrentContext::Outside
    }
}

/// Resolve a problem context based on the current context and an optional problem identifier.
/// Used by test and submit commands.
pub fn require_problem_context(
    current: CurrentContext,
    problem: Option<&str>,
) -> anyhow::Result<ProblemContext> {
    match current {
        CurrentContext::ProblemDir(ctx) => match problem {
            Some(_) => anyhow::bail!(
                "Cannot specify a problem from a problem directory. Move to the contest directory."
            ),
            None => Ok(ctx),
        },
        CurrentContext::ContestDir(ctx) => match problem {
            Some(p) => detect_problem_dir_from(&ctx.contest_dir.join(p.to_lowercase()))
                .with_context(|| format!("Problem '{}' not found", p)),
            None => anyhow::bail!("Specify a problem, or run from a problem directory."),
        },
        CurrentContext::Outside => {
            anyhow::bail!("Run this command from a problem or contest directory.")
        }
    }
}

/// Detect the problem context from the current working directory.
/// Reads `[package.metadata.acr]` from the Cargo.toml in cwd.
pub fn detect_problem_dir() -> anyhow::Result<ProblemContext> {
    let cwd = std::env::current_dir().context("Failed to get current directory")?;
    detect_problem_dir_from(&cwd)
}

/// Detect the problem context from a given directory.
pub fn detect_problem_dir_from(dir: &std::path::Path) -> anyhow::Result<ProblemContext> {
    let cargo_toml_path = dir.join("Cargo.toml");
    let content = std::fs::read_to_string(&cargo_toml_path)
        .with_context(|| format!("No Cargo.toml found in {}", dir.display()))?;
    let doc: toml::Value =
        toml::from_str(&content).context("Failed to parse Cargo.toml")?;

    let problem_url = doc
        .get("package")
        .and_then(|p| p.get("metadata"))
        .and_then(|m| m.get("acr"))
        .and_then(|a| a.get("problem_url"))
        .and_then(|u| u.as_str())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Not a problem directory: [package.metadata.acr] not found in {}",
                cargo_toml_path.display()
            )
        })?
        .to_string();

    // Parse URL: https://atcoder.jp/contests/{contest_id}/tasks/{task_screen_name}
    let parts: Vec<&str> = problem_url.split('/').collect();
    // Expected: ["https:", "", "atcoder.jp", "contests", "{contest_id}", "tasks", "{task_screen_name}"]
    let contest_id = parts
        .get(4)
        .ok_or_else(|| anyhow::anyhow!("Invalid problem_url: {}", problem_url))?
        .to_string();
    let task_screen_name = parts
        .get(6)
        .ok_or_else(|| anyhow::anyhow!("Invalid problem_url: {}", problem_url))?
        .to_string();

    let problem_alphabet = dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();

    Ok(ProblemContext {
        contest_id,
        problem_alphabet,
        task_screen_name,
        problem_dir: dir.to_path_buf(),
        problem_url,
    })
}

/// List all problem contexts in a contest directory, sorted by alphabet.
pub fn list_contest_problems(contest_dir: &std::path::Path) -> anyhow::Result<Vec<ProblemContext>> {
    let mut problems = Vec::new();
    for entry in std::fs::read_dir(contest_dir)
        .with_context(|| format!("Failed to read directory: {}", contest_dir.display()))?
    {
        let path = entry?.path();
        if path.is_dir()
            && let Ok(ctx) = detect_problem_dir_from(&path)
        {
            problems.push(ctx);
        }
    }
    problems.sort_by(|a, b| a.problem_alphabet.cmp(&b.problem_alphabet));
    Ok(problems)
}

/// Find a contest directory by contest ID, searching from the current working directory.
pub fn find_contest_dir_by_id(contest_id: &str) -> anyhow::Result<ContestContext> {
    let cwd = std::env::current_dir().context("Failed to get current directory")?;
    find_contest_dir_by_id_from(&cwd, contest_id)
}

/// Find a contest directory by contest ID, searching from a given base directory.
pub fn find_contest_dir_by_id_from(
    base_dir: &std::path::Path,
    contest_id: &str,
) -> anyhow::Result<ContestContext> {
    let candidate = base_dir.join(contest_id);
    if !candidate.exists() {
        anyhow::bail!("Contest '{}' not found", contest_id);
    }
    let cargo_toml = candidate.join("Cargo.toml");
    let content = std::fs::read_to_string(&cargo_toml)
        .with_context(|| format!("No Cargo.toml found in {}", candidate.display()))?;
    let doc: toml::Value = toml::from_str(&content).context("Failed to parse Cargo.toml")?;
    if doc.get("workspace").is_none() {
        anyhow::bail!(
            "{} is not a contest workspace (no [workspace] in Cargo.toml)",
            candidate.display()
        );
    }
    Ok(ContestContext {
        contest_id: contest_id.to_string(),
        contest_dir: candidate,
    })
}

/// Detect the contest workspace directory from the current working directory.
/// Checks cwd and its parent for a workspace Cargo.toml with [workspace].
pub fn detect_contest_dir() -> anyhow::Result<ContestContext> {
    let cwd = std::env::current_dir().context("Failed to get current directory")?;
    detect_contest_dir_from(&cwd)
}

/// Detect the contest workspace directory from a given directory.
pub fn detect_contest_dir_from(dir: &std::path::Path) -> anyhow::Result<ContestContext> {
    for candidate in [dir, dir.parent().unwrap_or(dir)] {
        let cargo_toml = candidate.join("Cargo.toml");
        if let Ok(content) = std::fs::read_to_string(&cargo_toml)
            && let Ok(doc) = toml::from_str::<toml::Value>(&content)
            && doc.get("workspace").is_some()
        {
            let contest_id = candidate
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
            return Ok(ContestContext {
                contest_id,
                contest_dir: candidate.to_path_buf(),
            });
        }
    }
    Err(anyhow::anyhow!(
        "Not in a contest workspace directory. Run this command from a contest or problem directory."
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_problem_dir() {
        let dir = tempfile::tempdir().unwrap();
        let problem_dir = dir.path().join("abc001").join("a");
        std::fs::create_dir_all(&problem_dir).unwrap();
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

        let ctx = detect_problem_dir_from(&problem_dir).unwrap();
        assert_eq!(ctx.contest_id, "abc001");
        assert_eq!(ctx.task_screen_name, "abc001_a");
        assert_eq!(ctx.problem_alphabet, "a");
        assert_eq!(
            ctx.problem_url,
            "https://atcoder.jp/contests/abc001/tasks/abc001_a"
        );
    }

    #[test]
    fn test_detect_problem_dir_not_a_problem() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("Cargo.toml"),
            "[package]\nname = \"test\"\n",
        )
        .unwrap();
        assert!(detect_problem_dir_from(dir.path()).is_err());
    }

    #[test]
    fn test_detect_contest_dir_from_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let ws = dir.path().join("abc001");
        std::fs::create_dir_all(&ws).unwrap();
        std::fs::write(
            ws.join("Cargo.toml"),
            "[workspace]\nmembers = [\"a\"]\nresolver = \"2\"\n",
        )
        .unwrap();

        let ctx = detect_contest_dir_from(&ws).unwrap();
        assert_eq!(ctx.contest_dir, ws);
        assert_eq!(ctx.contest_id, "abc001");
    }

    /// Create a contest workspace with problem directories for testing.
    fn create_test_workspace(
        base: &std::path::Path,
        contest_id: &str,
        problems: &[&str],
    ) -> PathBuf {
        let ws = base.join(contest_id);
        std::fs::create_dir_all(&ws).unwrap();
        let members: Vec<String> = problems.iter().map(|p| format!("\"{}\"", p)).collect();
        std::fs::write(
            ws.join("Cargo.toml"),
            format!(
                "[workspace]\nmembers = [{}]\nresolver = \"2\"\n",
                members.join(", ")
            ),
        )
        .unwrap();
        for p in problems {
            let problem_dir = ws.join(p);
            std::fs::create_dir_all(&problem_dir).unwrap();
            std::fs::write(
                problem_dir.join("Cargo.toml"),
                format!(
                    r#"[package]
name = "{contest_id}-{p}"
version = "0.1.0"
edition = "2021"

[package.metadata.acr]
problem_url = "https://atcoder.jp/contests/{contest_id}/tasks/{contest_id}_{p}"
"#,
                ),
            )
            .unwrap();
        }
        ws
    }

    #[test]
    fn test_detect_contest_dir_from_problem_dir() {
        let dir = tempfile::tempdir().unwrap();
        let ws = dir.path().join("abc001");
        let problem = ws.join("a");
        std::fs::create_dir_all(&problem).unwrap();
        std::fs::write(
            ws.join("Cargo.toml"),
            "[workspace]\nmembers = [\"a\"]\nresolver = \"2\"\n",
        )
        .unwrap();

        let ctx = detect_contest_dir_from(&problem).unwrap();
        assert_eq!(ctx.contest_dir, ws);
        assert_eq!(ctx.contest_id, "abc001");
    }

    // find_contest_dir_by_id_from tests

    #[test]
    fn test_find_contest_dir_by_id_from_valid() {
        let dir = tempfile::tempdir().unwrap();
        let ws = create_test_workspace(dir.path(), "abc001", &["a"]);
        let ctx = find_contest_dir_by_id_from(dir.path(), "abc001").unwrap();
        assert_eq!(ctx.contest_id, "abc001");
        assert_eq!(ctx.contest_dir, ws);
    }

    #[test]
    fn test_find_contest_dir_by_id_from_not_found() {
        let dir = tempfile::tempdir().unwrap();
        assert!(find_contest_dir_by_id_from(dir.path(), "abc999").is_err());
    }

    #[test]
    fn test_find_contest_dir_by_id_from_not_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let not_ws = dir.path().join("abc001");
        std::fs::create_dir_all(&not_ws).unwrap();
        std::fs::write(
            not_ws.join("Cargo.toml"),
            "[package]\nname = \"test\"\n",
        )
        .unwrap();
        assert!(find_contest_dir_by_id_from(dir.path(), "abc001").is_err());
    }

    // require_problem_context tests

    #[test]
    fn test_require_problem_context_problem_dir_none() {
        let dir = tempfile::tempdir().unwrap();
        let ws = create_test_workspace(dir.path(), "abc001", &["a"]);
        let problem_ctx = detect_problem_dir_from(&ws.join("a")).unwrap();
        let ctx = CurrentContext::ProblemDir(problem_ctx);
        let result = require_problem_context(ctx, None).unwrap();
        assert_eq!(result.contest_id, "abc001");
        assert_eq!(result.problem_alphabet, "a");
    }

    #[test]
    fn test_require_problem_context_problem_dir_some_error() {
        let dir = tempfile::tempdir().unwrap();
        let ws = create_test_workspace(dir.path(), "abc001", &["a"]);
        let problem_ctx = detect_problem_dir_from(&ws.join("a")).unwrap();
        let ctx = CurrentContext::ProblemDir(problem_ctx);
        assert!(require_problem_context(ctx, Some("b")).is_err());
    }

    #[test]
    fn test_require_problem_context_contest_dir_some() {
        let dir = tempfile::tempdir().unwrap();
        let ws = create_test_workspace(dir.path(), "abc001", &["a", "b"]);
        let contest_ctx = detect_contest_dir_from(&ws).unwrap();
        let ctx = CurrentContext::ContestDir(contest_ctx);
        let result = require_problem_context(ctx, Some("a")).unwrap();
        assert_eq!(result.contest_id, "abc001");
        assert_eq!(result.problem_alphabet, "a");
    }

    #[test]
    fn test_require_problem_context_contest_dir_none_error() {
        let dir = tempfile::tempdir().unwrap();
        let ws = create_test_workspace(dir.path(), "abc001", &["a"]);
        let contest_ctx = detect_contest_dir_from(&ws).unwrap();
        let ctx = CurrentContext::ContestDir(contest_ctx);
        assert!(require_problem_context(ctx, None).is_err());
    }

    #[test]
    fn test_require_problem_context_outside_error() {
        assert!(require_problem_context(CurrentContext::Outside, None).is_err());
        assert!(require_problem_context(CurrentContext::Outside, Some("a")).is_err());
    }

    // list_contest_problems tests

    #[test]
    fn test_list_contest_problems_sorted() {
        let dir = tempfile::tempdir().unwrap();
        let ws = create_test_workspace(dir.path(), "abc001", &["c", "a", "b"]);
        let problems = list_contest_problems(&ws).unwrap();
        let alphabets: Vec<&str> = problems.iter().map(|p| p.problem_alphabet.as_str()).collect();
        assert_eq!(alphabets, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_list_contest_problems_empty() {
        let dir = tempfile::tempdir().unwrap();
        let ws = dir.path().join("abc001");
        std::fs::create_dir_all(&ws).unwrap();
        std::fs::write(
            ws.join("Cargo.toml"),
            "[workspace]\nmembers = []\nresolver = \"2\"\n",
        )
        .unwrap();
        let problems = list_contest_problems(&ws).unwrap();
        assert!(problems.is_empty());
    }
}
