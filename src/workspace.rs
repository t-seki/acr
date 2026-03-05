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

/// Detect the contest workspace directory from the current working directory.
/// Checks cwd and its parent for a workspace Cargo.toml with [workspace].
pub fn detect_contest_dir() -> anyhow::Result<(PathBuf, String)> {
    let cwd = std::env::current_dir().context("Failed to get current directory")?;
    detect_contest_dir_from(&cwd)
}

/// Detect the contest workspace directory from a given directory.
pub fn detect_contest_dir_from(dir: &std::path::Path) -> anyhow::Result<(PathBuf, String)> {
    for candidate in [dir, dir.parent().unwrap_or(dir)] {
        let cargo_toml = candidate.join("Cargo.toml");
        if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
            if let Ok(doc) = toml::from_str::<toml::Value>(&content) {
                if doc.get("workspace").is_some() {
                    let contest_id = candidate
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string();
                    return Ok((candidate.to_path_buf(), contest_id));
                }
            }
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

        let (path, id) = detect_contest_dir_from(&ws).unwrap();
        assert_eq!(path, ws);
        assert_eq!(id, "abc001");
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

        let (path, id) = detect_contest_dir_from(&problem).unwrap();
        assert_eq!(path, ws);
        assert_eq!(id, "abc001");
    }
}
