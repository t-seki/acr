use std::path::Path;

use anyhow::{Context, Result};

use crate::types::TestCase;

/// テストケースをファイルに保存する。
pub fn save_test_cases(tests_dir: &Path, test_cases: &[TestCase]) -> Result<()> {
    std::fs::create_dir_all(tests_dir)
        .with_context(|| format!("テストディレクトリの作成に失敗: {}", tests_dir.display()))?;

    for tc in test_cases {
        let input_path = tests_dir.join(format!("{}.in", tc.index));
        let output_path = tests_dir.join(format!("{}.out", tc.index));

        std::fs::write(&input_path, &tc.input)
            .with_context(|| format!("入力ファイルの書き込みに失敗: {}", input_path.display()))?;
        std::fs::write(&output_path, &tc.expected).with_context(|| {
            format!("出力ファイルの書き込みに失敗: {}", output_path.display())
        })?;
    }

    Ok(())
}

/// テストケースをファイルから読み込む。
pub fn load_test_cases(tests_dir: &Path) -> Result<Vec<TestCase>> {
    if !tests_dir.exists() {
        return Ok(Vec::new());
    }

    let mut test_cases = Vec::new();
    let mut index = 1;

    loop {
        let input_path = tests_dir.join(format!("{}.in", index));
        let output_path = tests_dir.join(format!("{}.out", index));

        if !input_path.exists() || !output_path.exists() {
            break;
        }

        let input = std::fs::read_to_string(&input_path)
            .with_context(|| format!("入力ファイルの読み込みに失敗: {}", input_path.display()))?;
        let expected = std::fs::read_to_string(&output_path).with_context(|| {
            format!("出力ファイルの読み込みに失敗: {}", output_path.display())
        })?;

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
    fn test_save_and_load_test_cases() {
        let tmp = tempfile::tempdir().unwrap();
        let tests_dir = tmp.path().join("tests");

        let cases = vec![
            TestCase {
                index: 1,
                input: "3\n1 2 3\n".to_string(),
                expected: "6\n".to_string(),
            },
            TestCase {
                index: 2,
                input: "5\n10 20 30 40 50\n".to_string(),
                expected: "150\n".to_string(),
            },
        ];

        save_test_cases(&tests_dir, &cases).unwrap();

        assert!(tests_dir.join("1.in").exists());
        assert!(tests_dir.join("1.out").exists());
        assert!(tests_dir.join("2.in").exists());
        assert!(tests_dir.join("2.out").exists());

        let loaded = load_test_cases(&tests_dir).unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].input, "3\n1 2 3\n");
        assert_eq!(loaded[0].expected, "6\n");
        assert_eq!(loaded[1].input, "5\n10 20 30 40 50\n");
        assert_eq!(loaded[1].expected, "150\n");
    }

    #[test]
    fn test_load_test_cases_empty_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let cases = load_test_cases(&tmp.path().join("nonexistent")).unwrap();
        assert!(cases.is_empty());
    }
}
