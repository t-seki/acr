use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::config::load_template;
use crate::error::AcrsError;
use crate::types::Problem;

/// コンテストワークスペースを作成する。
pub fn create_contest_workspace(
    base_dir: &Path,
    contest_id: &str,
    problems: &[Problem],
) -> Result<PathBuf> {
    let contest_dir = base_dir.join(contest_id);
    if contest_dir.exists() {
        return Err(AcrsError::ContestAlreadyExists(contest_id.to_string()).into());
    }

    std::fs::create_dir_all(&contest_dir).with_context(|| {
        format!(
            "コンテストディレクトリの作成に失敗: {}",
            contest_dir.display()
        )
    })?;

    // ワークスペース Cargo.toml を生成
    let member_strs: Vec<String> = problems
        .iter()
        .map(|p| p.alphabet.to_lowercase())
        .collect();
    let members_toml: Vec<String> = member_strs.iter().map(|s| format!("\"{}\"", s)).collect();

    let workspace_toml = format!(
        "[workspace]\nmembers = [{}]\nresolver = \"2\"\n",
        members_toml.join(", ")
    );
    std::fs::write(contest_dir.join("Cargo.toml"), &workspace_toml).with_context(|| {
        format!(
            "ワークスペース Cargo.toml の書き込みに失敗: {}",
            contest_dir.display()
        )
    })?;

    // 各問題ディレクトリを作成
    let template = load_template()?;
    for problem in problems {
        create_problem_dir(&contest_dir, contest_id, problem, &template)?;
    }

    Ok(contest_dir)
}

/// 問題ディレクトリを作成する。
pub fn create_problem_dir(
    contest_dir: &Path,
    contest_id: &str,
    problem: &Problem,
    template: &str,
) -> Result<PathBuf> {
    let dir_name = problem.alphabet.to_lowercase();
    let problem_dir = contest_dir.join(&dir_name);

    std::fs::create_dir_all(problem_dir.join("src"))
        .with_context(|| format!("問題ディレクトリの作成に失敗: {}", problem_dir.display()))?;
    std::fs::create_dir_all(problem_dir.join("tests"))
        .with_context(|| format!("テストディレクトリの作成に失敗: {}", problem_dir.display()))?;

    // 問題 Cargo.toml
    let package_name = format!("{}-{}", contest_id, dir_name);
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[package.metadata.acrs]
problem_url = "{}"

[dependencies]
proconio = "0.4.5"
ac-library-rs = "0.1.1"
"#,
        package_name, problem.url
    );
    std::fs::write(problem_dir.join("Cargo.toml"), &cargo_toml)
        .with_context(|| format!("Cargo.toml の書き込みに失敗: {}", problem_dir.display()))?;

    // src/main.rs
    std::fs::write(problem_dir.join("src/main.rs"), template)
        .with_context(|| format!("main.rs の書き込みに失敗: {}", problem_dir.display()))?;

    Ok(problem_dir)
}

/// ワークスペースの Cargo.toml にメンバーを追加する。
pub fn add_member_to_workspace(contest_dir: &Path, member: &str) -> Result<()> {
    let cargo_toml_path = contest_dir.join("Cargo.toml");
    let content = std::fs::read_to_string(&cargo_toml_path)
        .with_context(|| format!("Cargo.toml の読み込みに失敗: {}", cargo_toml_path.display()))?;

    let mut doc: toml::Table = toml::from_str(&content)
        .with_context(|| format!("Cargo.toml のパースに失敗: {}", cargo_toml_path.display()))?;

    if let Some(workspace) = doc.get_mut("workspace") {
        if let Some(members) = workspace.get_mut("members") {
            if let Some(arr) = members.as_array_mut() {
                if !arr.iter().any(|v| v.as_str() == Some(member)) {
                    arr.push(toml::Value::String(member.to_string()));
                }
            }
        }
    }

    let new_content = toml::to_string_pretty(&doc).context("Cargo.toml のシリアライズに失敗")?;
    std::fs::write(&cargo_toml_path, new_content)
        .with_context(|| format!("Cargo.toml の書き込みに失敗: {}", cargo_toml_path.display()))?;

    Ok(())
}

/// カレントディレクトリから問題コンテキストを検出する。
/// 返り値: (contest_id, problem_alphabet, problem_dir)
pub fn detect_problem_context(current_dir: &Path) -> Result<(String, String, PathBuf)> {
    // 問題ディレクトリの Cargo.toml から metadata.acrs.problem_url を読む
    let cargo_toml_path = current_dir.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        anyhow::bail!(
            "Cargo.toml が見つかりません。問題ディレクトリから実行してください。"
        );
    }

    let content = std::fs::read_to_string(&cargo_toml_path)
        .with_context(|| format!("Cargo.toml の読み込みに失敗: {}", cargo_toml_path.display()))?;
    let doc: toml::Table = toml::from_str(&content)
        .with_context(|| format!("Cargo.toml のパースに失敗: {}", cargo_toml_path.display()))?;

    // package.metadata.acrs.problem_url を取得
    let problem_url = doc
        .get("package")
        .and_then(|p| p.get("metadata"))
        .and_then(|m| m.get("acrs"))
        .and_then(|a| a.get("problem_url"))
        .and_then(|u| u.as_str())
        .context("package.metadata.acrs.problem_url が見つかりません")?;

    // URL から contest_id と problem を抽出
    // 例: https://atcoder.jp/contests/abc001/tasks/abc001_a
    let parts: Vec<&str> = problem_url.split('/').collect();
    let contest_id = parts
        .iter()
        .position(|&p| p == "contests")
        .and_then(|i| parts.get(i + 1))
        .context("URLからcontest_idを抽出できません")?
        .to_string();

    let problem_alphabet = current_dir
        .file_name()
        .context("ディレクトリ名の取得に失敗")?
        .to_string_lossy()
        .to_string();

    Ok((contest_id, problem_alphabet, current_dir.to_path_buf()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Problem;

    #[test]
    fn test_create_contest_workspace() {
        let tmp = tempfile::tempdir().unwrap();
        let problems = vec![
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
        ];

        let contest_dir = create_contest_workspace(tmp.path(), "abc001", &problems).unwrap();

        assert!(contest_dir.join("Cargo.toml").exists());
        assert!(contest_dir.join("a/Cargo.toml").exists());
        assert!(contest_dir.join("a/src/main.rs").exists());
        assert!(contest_dir.join("a/tests").exists());
        assert!(contest_dir.join("b/Cargo.toml").exists());
        assert!(contest_dir.join("b/src/main.rs").exists());

        // ワークスペース Cargo.toml の内容確認
        let ws_toml = std::fs::read_to_string(contest_dir.join("Cargo.toml")).unwrap();
        assert!(ws_toml.contains("\"a\""));
        assert!(ws_toml.contains("\"b\""));
    }

    #[test]
    fn test_create_contest_workspace_already_exists() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir(tmp.path().join("abc001")).unwrap();
        let problems = vec![];

        let result = create_contest_workspace(tmp.path(), "abc001", &problems);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_member_to_workspace() {
        let tmp = tempfile::tempdir().unwrap();
        let toml_content = "[workspace]\nmembers = [\"a\", \"b\"]\nresolver = \"2\"\n";
        std::fs::write(tmp.path().join("Cargo.toml"), toml_content).unwrap();

        add_member_to_workspace(tmp.path(), "c").unwrap();

        let content = std::fs::read_to_string(tmp.path().join("Cargo.toml")).unwrap();
        assert!(content.contains("\"c\""));
    }

    #[test]
    fn test_detect_problem_context() {
        let tmp = tempfile::tempdir().unwrap();
        let problem_dir = tmp.path().join("a");
        std::fs::create_dir_all(&problem_dir).unwrap();

        let cargo_toml = r#"
[package]
name = "abc001-a"
version = "0.1.0"
edition = "2021"

[package.metadata.acrs]
problem_url = "https://atcoder.jp/contests/abc001/tasks/abc001_a"
"#;
        std::fs::write(problem_dir.join("Cargo.toml"), cargo_toml).unwrap();

        let (contest_id, alphabet, dir) = detect_problem_context(&problem_dir).unwrap();
        assert_eq!(contest_id, "abc001");
        assert_eq!(alphabet, "a");
        assert_eq!(dir, problem_dir);
    }
}
