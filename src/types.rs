use serde::Deserialize;

/// コンテスト情報（問題一覧付き）
#[derive(Debug, Clone)]
pub struct ContestInfo {
    pub contest_id: String,
    pub problems: Vec<Problem>,
}

/// 問題情報
#[derive(Debug, Clone)]
pub struct Problem {
    /// "A", "B", ...
    pub alphabet: String,
    /// 問題名
    pub name: String,
    /// "abc001_a" 形式
    pub task_screen_name: String,
    /// 問題ページURL
    pub url: String,
}

/// サンプルテストケース
#[derive(Debug, Clone)]
pub struct TestCase {
    pub index: usize,
    pub input: String,
    pub expected: String,
}

/// テスト実行結果
#[derive(Debug)]
pub enum TestResult {
    Ac,
    Wa { actual: String, expected: String },
    Re { stderr: String },
    Tle,
}

/// standings/json の TaskInfo エントリ
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TaskInfo {
    pub assignment: String,
    pub task_name: String,
    pub task_screen_name: String,
}

/// standings/json レスポンス
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StandingsResponse {
    pub task_info: Vec<TaskInfo>,
}
