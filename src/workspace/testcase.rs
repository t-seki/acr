use std::path::Path;

use anyhow::Context;

use crate::atcoder::TestCase;

/// Save test cases to the problem's tests/ directory.
/// Creates files: tests/1.in, tests/1.out, tests/2.in, tests/2.out, ...
pub fn save(problem_dir: &Path, test_cases: &[TestCase]) -> anyhow::Result<()> {
    let tests_dir = problem_dir.join("tests");
    std::fs::create_dir_all(&tests_dir)
        .with_context(|| format!("Failed to create tests dir: {}", tests_dir.display()))?;

    // Remove existing test case files to avoid stale data
    if let Ok(entries) = std::fs::read_dir(&tests_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|e| e.to_str())
                && (ext == "in" || ext == "out")
            {
                let _ = std::fs::remove_file(&path);
            }
        }
    }

    for tc in test_cases {
        std::fs::write(tests_dir.join(format!("{}.in", tc.index)), &tc.input)
            .with_context(|| format!("Failed to write test input {}", tc.index))?;
        std::fs::write(tests_dir.join(format!("{}.out", tc.index)), &tc.expected)
            .with_context(|| format!("Failed to write test output {}", tc.index))?;
    }
    Ok(())
}

/// Load test cases from the problem's tests/ directory.
/// Reads pairs of {n}.in and {n}.out files.
pub fn load(problem_dir: &Path) -> anyhow::Result<Vec<TestCase>> {
    let tests_dir = problem_dir.join("tests");
    if !tests_dir.exists() {
        return Ok(Vec::new());
    }

    let mut test_cases = Vec::new();
    let mut index = 1;
    loop {
        let in_path = tests_dir.join(format!("{}.in", index));
        let out_path = tests_dir.join(format!("{}.out", index));

        if !in_path.exists() || !out_path.exists() {
            break;
        }

        let input = std::fs::read_to_string(&in_path)
            .with_context(|| format!("Failed to read {}", in_path.display()))?;
        let expected = std::fs::read_to_string(&out_path)
            .with_context(|| format!("Failed to read {}", out_path.display()))?;

        test_cases.push(TestCase {
            index,
            input,
            expected,
        });
        index += 1;
    }
    Ok(test_cases)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_load_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let cases = vec![
            TestCase {
                index: 1,
                input: "3 5\n".to_string(),
                expected: "8\n".to_string(),
            },
            TestCase {
                index: 2,
                input: "10 20\n".to_string(),
                expected: "30\n".to_string(),
            },
        ];

        save(dir.path(), &cases).unwrap();

        // Verify files exist
        assert!(dir.path().join("tests/1.in").exists());
        assert!(dir.path().join("tests/1.out").exists());
        assert!(dir.path().join("tests/2.in").exists());
        assert!(dir.path().join("tests/2.out").exists());

        // Load and verify
        let loaded = load(dir.path()).unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].input, "3 5\n");
        assert_eq!(loaded[0].expected, "8\n");
        assert_eq!(loaded[1].input, "10 20\n");
        assert_eq!(loaded[1].expected, "30\n");
    }

    #[test]
    fn test_load_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let cases = load(dir.path()).unwrap();
        assert!(cases.is_empty());
    }
}
