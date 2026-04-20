use std::collections::HashMap;

use anyhow::Context;
use serde::Deserialize;

use super::{AtCoderClient, BASE_URL, scraper};
use crate::error::AcrError;

#[derive(Debug, Deserialize)]
struct StatusResponse {
    /// Polling interval suggested by AtCoder, in milliseconds.
    #[serde(default)]
    interval: Option<u64>,
    #[serde(rename = "Result", default)]
    result: HashMap<String, StatusEntry>,
}

#[derive(Debug, Deserialize)]
struct StatusEntry {
    #[serde(rename = "Html", default)]
    html: String,
}

#[derive(Debug, Clone)]
pub struct SubmissionStatus {
    /// Suggested polling interval (ms). `None` when judging is finished.
    pub interval_ms: Option<u64>,
    pub label: String,
    pub finished: bool,
}

impl AtCoderClient {
    /// Fetch the CSRF token from the submit page for a given problem.
    pub async fn fetch_csrf_token(
        &self,
        contest_id: &str,
        task_screen_name: &str,
    ) -> anyhow::Result<String> {
        let url = format!(
            "{}/contests/{}/submit?taskScreenName={}",
            BASE_URL, contest_id, task_screen_name
        );
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("Failed to fetch submit page: {}", url))?;

        // If we land outside /submit (redirected to /login), the session is invalid.
        let final_url = resp.url().to_string();
        if !final_url.contains("/submit") {
            return Err(AcrError::NotLoggedIn.into());
        }

        let status = resp.status();
        if !status.is_success() {
            return Err(AcrError::SubmissionFailed {
                reason: format!("submit page returned HTTP {}", status),
            }
            .into());
        }

        let html = resp
            .text()
            .await
            .with_context(|| "Failed to read submit page body")?;
        scraper::extract_csrf_token(&html).ok_or_else(|| AcrError::CsrfTokenNotFound.into())
    }

    /// POST a source code submission and return the resulting submission ID.
    pub async fn submit_source(
        &self,
        contest_id: &str,
        task_screen_name: &str,
        language_id: &str,
        source: &str,
        csrf_token: &str,
    ) -> anyhow::Result<u64> {
        let url = format!("{}/contests/{}/submit", BASE_URL, contest_id);
        let form = [
            ("data.TaskScreenName", task_screen_name),
            ("data.LanguageId", language_id),
            ("sourceCode", source),
            ("csrf_token", csrf_token),
        ];

        let resp = self
            .client
            .post(&url)
            .form(&form)
            .send()
            .await
            .with_context(|| format!("Failed to POST submission: {}", url))?;

        let status = resp.status();
        let final_url = resp.url().to_string();

        // After a successful submit, AtCoder redirects to /contests/{id}/submissions/me.
        // If we're still on /submit (or anywhere else), the submission was rejected.
        if !final_url.contains("/submissions/me") {
            return Err(AcrError::SubmissionFailed {
                reason: format!(
                    "AtCoder did not redirect to submissions/me (HTTP {}, landed at {})",
                    status, final_url
                ),
            }
            .into());
        }

        if !status.is_success() {
            return Err(AcrError::SubmissionFailed {
                reason: format!("HTTP {}", status),
            }
            .into());
        }

        let html = resp
            .text()
            .await
            .with_context(|| "Failed to read submissions/me body")?;
        scraper::extract_latest_submission_id(&html).ok_or_else(|| {
            AcrError::SubmissionFailed {
                reason: "could not locate submission ID on submissions/me page".to_string(),
            }
            .into()
        })
    }

    /// Poll the judge status JSON endpoint for a single submission.
    pub async fn fetch_submission_status(
        &self,
        contest_id: &str,
        submission_id: u64,
    ) -> anyhow::Result<SubmissionStatus> {
        let url = format!(
            "{}/contests/{}/submissions/me/status/json?reload=true&sids[]={}",
            BASE_URL, contest_id, submission_id
        );
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("Failed to fetch status JSON: {}", url))?;

        if !resp.status().is_success() {
            return Err(AcrError::SubmissionFailed {
                reason: format!("status JSON returned HTTP {}", resp.status()),
            }
            .into());
        }

        let body: StatusResponse = resp
            .json()
            .await
            .with_context(|| "Failed to parse status JSON")?;

        let entry = body.result.get(&submission_id.to_string());
        let judge = entry
            .map(|e| scraper::extract_judge_status(&e.html))
            .unwrap_or_else(|| scraper::JudgeStatus {
                label: "WJ".to_string(),
                finished: false,
            });

        Ok(SubmissionStatus {
            interval_ms: body.interval,
            label: judge.label,
            finished: judge.finished,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_status_response_judging() {
        let json = r#"{
            "interval": 4500,
            "Result": {
                "12345678": { "Html": "<span class=\"label label-default\">WJ</span>" }
            }
        }"#;
        let resp: StatusResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.interval, Some(4500));
        assert!(resp.result.contains_key("12345678"));
    }

    #[test]
    fn test_deserialize_status_response_finished() {
        // When judging has finished, AtCoder typically omits `interval`.
        let json = r#"{
            "Result": {
                "12345678": { "Html": "<span class=\"label label-success\">AC</span>" }
            }
        }"#;
        let resp: StatusResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.interval, None);
    }
}
