pub mod auth;
pub mod contest;
pub mod scraper;

use std::sync::Arc;

use anyhow::Context;
use reqwest::cookie::Jar;

pub const BASE_URL: &str = "https://atcoder.jp";

pub struct AtCoderClient {
    client: reqwest::Client,
}

#[derive(Debug, Clone)]
pub struct ContestInfo {
    pub contest_id: String,
    pub problems: Vec<Problem>,
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub alphabet: String,
    pub name: String,
    pub task_screen_name: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct TestCase {
    pub index: usize,
    pub input: String,
    pub expected: String,
}

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
        let jar = Jar::default();
        let cookie = format!("REVEL_SESSION={}", revel_session);
        let url = BASE_URL.parse().context("Failed to parse base URL")?;
        jar.add_cookie_str(&cookie, &url);

        let client = reqwest::Client::builder()
            .cookie_provider(Arc::new(jar))
            .build()
            .context("Failed to create HTTP client")?;
        Ok(Self { client })
    }
}
