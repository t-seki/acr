use anyhow::Context;
use serde::Deserialize;

use super::{AtCoderClient, BASE_URL, ContestInfo, Problem, TestCase};
use crate::error::AcrsError;

#[derive(Deserialize)]
struct StandingsResponse {
    #[serde(rename = "TaskInfo")]
    task_info: Vec<TaskInfo>,
}

#[derive(Deserialize)]
struct TaskInfo {
    #[serde(rename = "Assignment")]
    assignment: String,
    #[serde(rename = "TaskName")]
    task_name: String,
    #[serde(rename = "TaskScreenName")]
    task_screen_name: String,
}

impl AtCoderClient {
    /// Fetch contest problem list from standings/json API.
    pub async fn fetch_contest(&self, contest_id: &str) -> anyhow::Result<ContestInfo> {
        let url = format!("{}/contests/{}/standings/json", BASE_URL, contest_id);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("Failed to fetch contest: {}", contest_id))?;

        if !resp.status().is_success() {
            return Err(AcrsError::ContestNotFound(contest_id.to_string()).into());
        }

        let standings: StandingsResponse = resp
            .json()
            .await
            .with_context(|| format!("Failed to parse standings for: {}", contest_id))?;

        let problems = standings
            .task_info
            .into_iter()
            .map(|t| Problem {
                alphabet: t.assignment.clone(),
                url: format!(
                    "{}/contests/{}/tasks/{}",
                    BASE_URL, contest_id, t.task_screen_name
                ),
                name: t.task_name,
                task_screen_name: t.task_screen_name,
            })
            .collect();

        Ok(ContestInfo {
            contest_id: contest_id.to_string(),
            problems,
        })
    }

    /// Fetch sample test cases from a problem page.
    pub async fn fetch_sample_cases(
        &self,
        contest_id: &str,
        task_screen_name: &str,
    ) -> anyhow::Result<Vec<TestCase>> {
        let url = format!(
            "{}/contests/{}/tasks/{}",
            BASE_URL, contest_id, task_screen_name
        );
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("Failed to fetch problem: {}", task_screen_name))?;
        let html = resp
            .text()
            .await
            .with_context(|| format!("Failed to read problem page: {}", task_screen_name))?;

        let pairs = super::scraper::extract_sample_cases(&html)?;
        let test_cases = pairs
            .into_iter()
            .enumerate()
            .map(|(i, (input, expected))| TestCase {
                index: i + 1,
                input,
                expected,
            })
            .collect();
        Ok(test_cases)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_standings_response() {
        let json = r#"{
            "TaskInfo": [
                {
                    "Assignment": "A",
                    "TaskName": "Problem A",
                    "TaskScreenName": "abc001_a"
                },
                {
                    "Assignment": "B",
                    "TaskName": "Problem B",
                    "TaskScreenName": "abc001_b"
                }
            ]
        }"#;
        let resp: StandingsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.task_info.len(), 2);
        assert_eq!(resp.task_info[0].assignment, "A");
        assert_eq!(resp.task_info[0].task_screen_name, "abc001_a");
        assert_eq!(resp.task_info[1].task_name, "Problem B");
    }
}
