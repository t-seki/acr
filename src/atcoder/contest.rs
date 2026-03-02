use anyhow::Result;

use super::client::AtCoderClient;
use crate::types::{ContestInfo, Problem, StandingsResponse};

/// standings/json から問題一覧を取得する。
pub async fn fetch_problems(client: &AtCoderClient, contest_id: &str) -> Result<ContestInfo> {
    let path = format!("/contests/{}/standings/json", contest_id);
    let standings: StandingsResponse = client.get_json(&path).await?;

    let problems = standings
        .task_info
        .into_iter()
        .map(|task| Problem {
            alphabet: task.assignment.clone(),
            name: task.task_name,
            url: format!(
                "https://atcoder.jp/contests/{}/tasks/{}",
                contest_id, task.task_screen_name
            ),
            task_screen_name: task.task_screen_name,
        })
        .collect();

    Ok(ContestInfo {
        contest_id: contest_id.to_string(),
        problems,
    })
}

#[cfg(test)]
mod tests {
    use crate::types::StandingsResponse;

    #[test]
    fn test_parse_standings_json() {
        let json = r#"{
            "TaskInfo": [
                {
                    "Assignment": "A",
                    "TaskName": "Which is Smaller?",
                    "TaskScreenName": "abc001_a"
                },
                {
                    "Assignment": "B",
                    "TaskName": "Vision Test",
                    "TaskScreenName": "abc001_b"
                }
            ]
        }"#;

        let standings: StandingsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(standings.task_info.len(), 2);
        assert_eq!(standings.task_info[0].assignment, "A");
        assert_eq!(standings.task_info[0].task_screen_name, "abc001_a");
        assert_eq!(standings.task_info[1].assignment, "B");
    }
}
