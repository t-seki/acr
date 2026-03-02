use std::time::Duration;

use anyhow::Context;

use super::AtCoderClient;
use super::BASE_URL;
use super::scraper::{extract_csrf_token, extract_latest_submission_status, extract_rust_language_id};

#[derive(Debug)]
pub struct SubmissionResult {
    pub status: String,
    pub submission_url: String,
}

impl AtCoderClient {
    /// Submit a solution to AtCoder.
    pub async fn submit(
        &self,
        contest_id: &str,
        task_screen_name: &str,
        source_code: &str,
    ) -> anyhow::Result<()> {
        let submit_url = format!("{}/contests/{}/submit", BASE_URL, contest_id);

        // GET submit page to obtain csrf_token and language_id
        let resp = self
            .client
            .get(&submit_url)
            .send()
            .await
            .context("Failed to access submit page")?;
        let html = resp.text().await.context("Failed to read submit page")?;

        let csrf_token = extract_csrf_token(&html)?;
        let language_id = extract_rust_language_id(&html)?;

        // POST submission
        let params = [
            ("csrf_token", csrf_token.as_str()),
            ("data.TaskScreenName", task_screen_name),
            ("data.LanguageId", language_id.as_str()),
            ("sourceCode", source_code),
        ];

        let resp = self
            .client
            .post(&submit_url)
            .form(&params)
            .send()
            .await
            .context("Failed to submit solution")?;

        if !resp.status().is_success() && !resp.status().is_redirection() {
            anyhow::bail!("Submission failed with status: {}", resp.status());
        }

        Ok(())
    }

    /// Poll the submissions/me page until the result is final.
    /// Returns the submission result.
    pub async fn poll_result(&self, contest_id: &str) -> anyhow::Result<SubmissionResult> {
        let submissions_url = format!(
            "{}/contests/{}/submissions/me",
            BASE_URL, contest_id
        );

        let max_attempts = 30;
        for _ in 0..max_attempts {
            tokio::time::sleep(Duration::from_secs(2)).await;

            let resp = self
                .client
                .get(&submissions_url)
                .send()
                .await
                .context("Failed to fetch submissions")?;
            let html = resp.text().await.context("Failed to read submissions page")?;

            if let Some((status, url)) =
                extract_latest_submission_status(&html, contest_id)?
            {
                // Waiting for Judge statuses
                let waiting = ["WJ", "1/", "2/", "3/", "4/", "5/"];
                let is_waiting = waiting.iter().any(|w| status.contains(w));
                if !is_waiting {
                    return Ok(SubmissionResult {
                        status,
                        submission_url: url,
                    });
                }
            }
        }

        anyhow::bail!("Timed out waiting for judge result")
    }
}
