pub mod auth;
pub mod contest;
pub mod scraper;
pub mod submit;

use anyhow::Context;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};

pub struct AtCoderClient {
    client: reqwest::Client,
}

const BASE_URL: &str = "https://atcoder.jp";

impl AtCoderClient {
    /// Create a new client without session (for login)
    pub fn new() -> anyhow::Result<Self> {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .context("Failed to create HTTP client")?;
        Ok(Self { client })
    }

    /// Create a client with an existing REVEL_SESSION cookie
    pub fn with_session(revel_session: &str) -> anyhow::Result<Self> {
        let mut headers = HeaderMap::new();
        let cookie_value = format!("REVEL_SESSION={}", revel_session);
        headers.insert(
            COOKIE,
            HeaderValue::from_str(&cookie_value).context("Invalid session value")?,
        );
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .default_headers(headers)
            .build()
            .context("Failed to create HTTP client")?;
        Ok(Self { client })
    }
}
