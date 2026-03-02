use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::atcoder::Problem;

/// Create a full contest workspace directory structure.
pub fn create_contest_workspace(
    base_dir: &Path,
    contest_id: &str,
    problems: &[Problem],
    template: &str,
) -> anyhow::Result<PathBuf> {
    let workspace_dir = base_dir.join(contest_id);
    std::fs::create_dir_all(&workspace_dir)
        .with_context(|| format!("Failed to create workspace: {}", workspace_dir.display()))?;

    // Generate workspace Cargo.toml
    let members: Vec<String> = problems
        .iter()
        .map(|p| format!("\"{}\"", p.alphabet.to_lowercase()))
        .collect();
    let workspace_toml = format!(
        "[workspace]\nmembers = [{}]\nresolver = \"2\"\n",
        members.join(", ")
    );
    std::fs::write(workspace_dir.join("Cargo.toml"), workspace_toml)
        .context("Failed to write workspace Cargo.toml")?;

    // Create each problem directory
    for problem in problems {
        create_problem_dir(&workspace_dir, contest_id, problem, template)?;
    }

    Ok(workspace_dir)
}

/// Add a single problem to an existing workspace.
pub fn add_problem_to_workspace(
    workspace_dir: &Path,
    contest_id: &str,
    problem: &Problem,
    template: &str,
) -> anyhow::Result<()> {
    create_problem_dir(workspace_dir, contest_id, problem, template)?;

    // Update workspace Cargo.toml members
    let cargo_toml_path = workspace_dir.join("Cargo.toml");
    let content = std::fs::read_to_string(&cargo_toml_path)
        .context("Failed to read workspace Cargo.toml")?;
    let mut doc: toml::Value = toml::from_str(&content)
        .context("Failed to parse workspace Cargo.toml")?;

    let new_member = problem.alphabet.to_lowercase();
    if let Some(members) = doc
        .get_mut("workspace")
        .and_then(|w| w.get_mut("members"))
        .and_then(|m| m.as_array_mut())
    {
        if !members.iter().any(|m| m.as_str() == Some(&new_member)) {
            members.push(toml::Value::String(new_member));
        }
    }

    let updated = toml::to_string(&doc).context("Failed to serialize workspace Cargo.toml")?;
    std::fs::write(&cargo_toml_path, updated)
        .context("Failed to write workspace Cargo.toml")?;

    Ok(())
}

fn create_problem_dir(
    workspace_dir: &Path,
    contest_id: &str,
    problem: &Problem,
    template: &str,
) -> anyhow::Result<()> {
    let alphabet = problem.alphabet.to_lowercase();
    let problem_dir = workspace_dir.join(&alphabet);
    let src_dir = problem_dir.join("src");
    let tests_dir = problem_dir.join("tests");

    std::fs::create_dir_all(&src_dir)
        .with_context(|| format!("Failed to create src dir: {}", src_dir.display()))?;
    std::fs::create_dir_all(&tests_dir)
        .with_context(|| format!("Failed to create tests dir: {}", tests_dir.display()))?;

    // Generate problem Cargo.toml
    let cargo_toml = format!(
        r#"[package]
name = "{contest_id}-{alphabet}"
version = "0.1.0"
edition = "2021"

[package.metadata.acrs]
problem_url = "{url}"

[dependencies]
proconio = "0.4.5"
ac-library-rs = "0.1.1"
"#,
        contest_id = contest_id,
        alphabet = alphabet,
        url = problem.url,
    );
    std::fs::write(problem_dir.join("Cargo.toml"), cargo_toml)
        .context("Failed to write problem Cargo.toml")?;

    // Write template to src/main.rs
    std::fs::write(src_dir.join("main.rs"), template)
        .context("Failed to write src/main.rs")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_problems() -> Vec<Problem> {
        vec![
            Problem {
                alphabet: "A".to_string(),
                name: "Problem A".to_string(),
                task_screen_name: "abc001_a".to_string(),
                url: "https://atcoder.jp/contests/abc001/tasks/abc001_a".to_string(),
            },
            Problem {
                alphabet: "B".to_string(),
                name: "Problem B".to_string(),
                task_screen_name: "abc001_b".to_string(),
                url: "https://atcoder.jp/contests/abc001/tasks/abc001_b".to_string(),
            },
        ]
    }

    #[test]
    fn test_create_contest_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let problems = sample_problems();
        let ws = create_contest_workspace(dir.path(), "abc001", &problems, "fn main() {}").unwrap();

        // Check workspace Cargo.toml
        let content = std::fs::read_to_string(ws.join("Cargo.toml")).unwrap();
        assert!(content.contains("[workspace]"));
        assert!(content.contains("\"a\""));
        assert!(content.contains("\"b\""));

        // Check problem directories
        assert!(ws.join("a/src/main.rs").exists());
        assert!(ws.join("a/Cargo.toml").exists());
        assert!(ws.join("a/tests").exists());
        assert!(ws.join("b/src/main.rs").exists());

        // Check problem Cargo.toml content
        let problem_toml = std::fs::read_to_string(ws.join("a/Cargo.toml")).unwrap();
        assert!(problem_toml.contains("name = \"abc001-a\""));
        assert!(problem_toml.contains("problem_url"));

        // Check template
        let main_rs = std::fs::read_to_string(ws.join("a/src/main.rs")).unwrap();
        assert_eq!(main_rs, "fn main() {}");
    }

    #[test]
    fn test_add_problem_to_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let problems = sample_problems();
        let ws = create_contest_workspace(dir.path(), "abc001", &problems, "fn main() {}").unwrap();

        let new_problem = Problem {
            alphabet: "C".to_string(),
            name: "Problem C".to_string(),
            task_screen_name: "abc001_c".to_string(),
            url: "https://atcoder.jp/contests/abc001/tasks/abc001_c".to_string(),
        };
        add_problem_to_workspace(&ws, "abc001", &new_problem, "fn main() {}").unwrap();

        // Check new problem dir exists
        assert!(ws.join("c/src/main.rs").exists());

        // Check workspace members updated
        let content = std::fs::read_to_string(ws.join("Cargo.toml")).unwrap();
        assert!(content.contains("c"));
    }
}
