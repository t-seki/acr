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

[package.metadata.acr]
problem_url = "{url}"

[dependencies]
ac-library-rs = "0.2.0"
alga = "0.9.3"
amplify = "4.9.0"
amplify_derive = "4.0.1"
amplify_num = "0.5.3"
argio = "0.2.0"
ascii = "1.1.0"
az = "1.2.1"
bitset-fixed = "0.1.0"
bitvec = "1.0.1"
bstr = "1.12.0"
btreemultimap = "0.1.1"
counter = "0.7.0"
easy-ext = "1.0.2"
either = "1.15.0"
fixedbitset = "0.5.7"
getrandom = "0.3.3"
glidesort = "0.1.2"
hashbag = "0.1.12"
im-rc = "15.1.0"
indexing = "0.4.1"
indexmap = "2.11.0"
itertools = "0.14.0"
itertools-num = "0.1.3"
lazy_static = "1.5.0"
libm = "0.2.15"
maplit = "1.0.2"
memoise = "0.3.2"
multimap = "0.10.1"
multiversion = "0.8.0"
nalgebra = "0.34.0"
ndarray = "0.16.1"
num = "0.4.3"
num-bigint = "0.4.6"
num-complex = "0.4.6"
num-derive = "0.4.2"
num-integer = "0.1.46"
num-iter = "0.1.45"
num-rational = "0.4.2"
num-traits = "0.2.19"
omniswap = "0.1.0"
once_cell = "1.21.3"
ordered-float = "5.0.0"
pathfinding = "4.14.0"
permutohedron = "0.2.4"
petgraph = "0.8.2"
primal = "0.3.3"
proconio = "0.5.0"
rand = "0.9.2"
rand_chacha = "0.9.0"
rand_core = "0.9.3"
rand_distr = "0.5.1"
rand_hc = "0.4.0"
rand_pcg = "0.9.0"
rand_xorshift = "0.4.0"
rand_xoshiro = "0.7.0"
recur-fn = "2.2.0"
regex = "1.11.2"
rpds = "1.1.1"
rustc-hash = "2.1.1"
smallvec = "1.15.1"
static_assertions = "1.1.0"
statrs = "0.18.0"
superslice = "1.0.0"
tap = "1.0.1"
text_io = "0.1.13"
thiserror = "2.0.16"
varisat = "0.2.2"
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
