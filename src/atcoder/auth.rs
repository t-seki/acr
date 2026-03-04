use anyhow::Context;

use super::AtCoderClient;
use super::scraper::extract_username;

const TOP_URL: &str = "https://atcoder.jp/";

impl AtCoderClient {
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
