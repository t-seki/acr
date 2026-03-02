use anyhow::Context;

use super::AtCoderClient;
use super::scraper::{extract_csrf_token, extract_username};
use crate::error::AcrsError;

const LOGIN_URL: &str = "https://atcoder.jp/login";
const TOP_URL: &str = "https://atcoder.jp/";

impl AtCoderClient {
    /// Login to AtCoder and return the REVEL_SESSION cookie value.
    pub async fn login(&self, username: &str, password: &str) -> anyhow::Result<String> {
        // GET /login to obtain csrf_token
        let resp = self
            .client
            .get(LOGIN_URL)
            .send()
            .await
            .context("Failed to access login page")?;
        let html = resp.text().await.context("Failed to read login page")?;
        let csrf_token = extract_csrf_token(&html)?;

        // POST /login with credentials
        let params = [
            ("username", username),
            ("password", password),
            ("csrf_token", &csrf_token),
        ];
        let resp = self
            .client
            .post(LOGIN_URL)
            .form(&params)
            .send()
            .await
            .context("Failed to submit login form")?;

        // Extract REVEL_SESSION from response cookies
        let revel_session = resp
            .cookies()
            .find(|c| c.name() == "REVEL_SESSION")
            .map(|c| c.value().to_string())
            .ok_or_else(|| {
                AcrsError::ScrapingFailed("Login failed: REVEL_SESSION cookie not found. Check your username and password.".to_string())
            })?;

        Ok(revel_session)
    }

    /// Check if the current session is valid. Returns the username if logged in.
    pub async fn check_session(&self) -> anyhow::Result<Option<String>> {
        let resp = self
            .client
            .get(TOP_URL)
            .send()
            .await
            .context("Failed to access AtCoder")?;
        let html = resp.text().await.context("Failed to read page")?;
        Ok(extract_username(&html))
    }
}
