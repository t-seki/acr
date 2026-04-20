use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::Context;
use colored::Colorize;

use super::TestResult;
use crate::atcoder::TestCase;

const TLE_TIMEOUT: Duration = Duration::from_secs(5);

/// Build the problem binary with `cargo build --release`.
/// Returns the path to the compiled binary.
pub fn build(problem_dir: &Path) -> anyhow::Result<PathBuf> {
    let output = std::process::Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(problem_dir)
        .output()
        .context("Failed to run cargo build")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Build failed:\n{}", stderr);
    }

    // Find the binary name from Cargo.toml
    let cargo_toml = std::fs::read_to_string(problem_dir.join("Cargo.toml"))
        .context("Failed to read problem Cargo.toml")?;
    let doc: toml::Value = toml::from_str(&cargo_toml)?;
    let name = doc
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .context("Could not find package name in Cargo.toml")?;

    // Binary path: find the workspace target dir
    // Walk up from problem_dir to find workspace root with target/
    let mut search = problem_dir.to_path_buf();
    loop {
        let target_bin = search.join("target/release").join(name);
        if target_bin.exists() {
            return Ok(target_bin);
        }
        if !search.pop() {
            break;
        }
    }

    anyhow::bail!("Could not find compiled binary for {}", name)
}

/// Run a single test case against the binary.
pub async fn run_test(binary: &Path, test_case: &TestCase) -> TestResult {
    use std::process::Stdio;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::process::Command;

    let mut child = match Command::new(binary)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            return TestResult::Re {
                stderr: e.to_string(),
            };
        }
    };

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(test_case.input.as_bytes()).await;
        drop(stdin);
    }

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let stdout_handle = tokio::spawn(async move {
        let mut buf = Vec::new();
        if let Some(mut out) = stdout {
            let _ = out.read_to_end(&mut buf).await;
        }
        buf
    });
    let stderr_handle = tokio::spawn(async move {
        let mut buf = Vec::new();
        if let Some(mut err) = stderr {
            let _ = err.read_to_end(&mut buf).await;
        }
        buf
    });

    let timed_out = tokio::select! {
        status = child.wait() => {
            match status {
                Ok(s) if !s.success() => {
                    let stderr_buf = stderr_handle.await.unwrap_or_default();
                    return TestResult::Re {
                        stderr: String::from_utf8_lossy(&stderr_buf).to_string(),
                    };
                }
                Ok(_) => false,
                Err(e) => return TestResult::Re { stderr: e.to_string() },
            }
        }
        _ = tokio::time::sleep(TLE_TIMEOUT) => {
            let _ = child.kill().await;
            true
        }
    };

    if timed_out {
        return TestResult::Tle;
    }

    let stdout_buf = stdout_handle.await.unwrap_or_default();
    let actual = String::from_utf8_lossy(&stdout_buf).to_string();
    if actual.trim_end() == test_case.expected.trim_end() {
        TestResult::Ac
    } else {
        TestResult::Wa {
            actual,
            expected: test_case.expected.clone(),
        }
    }
}

/// Run all test cases and return results.
pub async fn run_all(
    problem_dir: &Path,
    test_cases: &[TestCase],
) -> anyhow::Result<Vec<(usize, TestResult)>> {
    println!("{}", "Building...".dimmed());
    let binary = build(problem_dir)?;
    println!("{}", "Running tests...".dimmed());

    let mut results = Vec::new();
    for tc in test_cases {
        let result = run_test(&binary, tc).await;
        results.push((tc.index, result));
    }
    Ok(results)
}

/// Display test results with colored output.
pub fn display_results(results: &[(usize, TestResult)]) {
    for (index, result) in results {
        match result {
            TestResult::Ac => {
                println!("  Test {} ... {}", index, "AC".green().bold());
            }
            TestResult::Wa { actual, expected } => {
                println!("  Test {} ... {}", index, "WA".red().bold());
                println!("    Expected: {}", expected.trim_end());
                println!("    Actual:   {}", actual.trim_end());
            }
            TestResult::Re { stderr } => {
                println!("  Test {} ... {}", index, "RE".yellow().bold());
                if !stderr.is_empty() {
                    println!("    {}", stderr.trim_end());
                }
            }
            TestResult::Tle => {
                println!("  Test {} ... {}", index, "TLE".yellow().bold());
            }
        }
    }

    let total = results.len();
    let passed = results
        .iter()
        .filter(|(_, r)| matches!(r, TestResult::Ac))
        .count();
    let status = if passed == total {
        format!("All tests passed ({}/{})", passed, total).green()
    } else {
        format!("{}/{} passed", passed, total).red()
    };
    println!("\n  {}", status.bold());
}
