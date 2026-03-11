use anyhow::Context;
use serde::Deserialize;

use super::{AtCoderClient, BASE_URL, ContestInfo, Problem, TestCase};
use crate::error::AcrError;

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
    /// Falls back to scraping the tasks page if standings/json is unavailable.
    pub async fn fetch_contest(&self, contest_id: &str) -> anyhow::Result<ContestInfo> {
        // Try standings/json first
        let url = format!("{}/contests/{}/standings/json", BASE_URL, contest_id);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("Failed to fetch contest: {}", contest_id))?;

        if resp.status().is_success() {
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

            return Ok(ContestInfo {
                contest_id: contest_id.to_string(),
                problems,
            });
        }

        // Fallback: scrape /contests/{contest_id}/tasks page
        self.fetch_contest_from_tasks_page(contest_id).await
    }

    /// Fetch contest problem list by scraping the tasks page.
    async fn fetch_contest_from_tasks_page(
        &self,
        contest_id: &str,
    ) -> anyhow::Result<ContestInfo> {
        let url = format!("{}/contests/{}/tasks", BASE_URL, contest_id);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("Failed to fetch contest: {}", contest_id))?;

        if !resp.status().is_success() {
            return Err(AcrError::ContestNotFound(contest_id.to_string()).into());
        }

        let html = resp
            .text()
            .await
            .with_context(|| format!("Failed to read tasks page for: {}", contest_id))?;

        let task_list = super::scraper::extract_task_list(&html);
        if task_list.is_empty() {
            return Err(AcrError::ContestNotFound(contest_id.to_string()).into());
        }

        let problems = task_list
            .into_iter()
            .map(|(alphabet, name, task_screen_name)| Problem {
                alphabet,
                url: format!(
                    "{}/contests/{}/tasks/{}",
                    BASE_URL, contest_id, task_screen_name
                ),
                name,
                task_screen_name,
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

        let max_retries = 3;
        let mut attempts = 0;
        let html = loop {
            let resp = self
                .client
                .get(&url)
                .send()
                .await
                .with_context(|| format!("Failed to fetch problem: {}", task_screen_name))?;
            let status = resp.status();
            if status.is_success() {
                break resp
                    .text()
                    .await
                    .with_context(|| format!("Failed to read problem page: {}", task_screen_name))?;
            }
            if (status == reqwest::StatusCode::TOO_MANY_REQUESTS
                || status.is_server_error())
                && attempts < max_retries
            {
                attempts += 1;
                let delay = std::time::Duration::from_secs(1 << attempts);
                tokio::time::sleep(delay).await;
                continue;
            }
            anyhow::bail!(
                "Failed to fetch problem {} (HTTP {})",
                task_screen_name,
                status
            );
        };

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
