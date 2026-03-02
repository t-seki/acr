use anyhow::{Context, Result};
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};

const BASE_URL: &str = "https://atcoder.jp";

/// AtCoder HTTP クライアント
#[derive(Debug, Clone)]
pub struct AtCoderClient {
    client: reqwest::Client,
    session_cookie: Option<String>,
}

impl AtCoderClient {
    /// セッションなしでクライアントを作成する。
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .context("HTTPクライアントの作成に失敗")?;
        Ok(Self {
            client,
            session_cookie: None,
        })
    }

    /// セッション付きでクライアントを作成する。
    pub fn with_session(revel_session: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        let cookie_value = format!("REVEL_SESSION={}", revel_session);
        headers.insert(
            COOKIE,
            HeaderValue::from_str(&cookie_value).context("セッションCookieの設定に失敗")?,
        );
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .default_headers(headers)
            .build()
            .context("HTTPクライアントの作成に失敗")?;
        Ok(Self {
            client,
            session_cookie: Some(revel_session.to_string()),
        })
    }

    /// GETリクエストを送信し、レスポンスボディを返す。
    pub async fn get(&self, path: &str) -> Result<String> {
        let url = format!("{}{}", BASE_URL, path);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("GETリクエスト失敗: {}", url))?;
        let status = resp.status();
        let body = resp
            .text()
            .await
            .with_context(|| format!("レスポンスの読み取り失敗: {}", url))?;
        if !status.is_success() {
            anyhow::bail!("HTTPエラー {}: {}", status, url);
        }
        Ok(body)
    }

    /// フォームPOSTリクエストを送信し、レスポンスボディを返す。
    pub async fn post_form(&self, path: &str, params: &[(&str, &str)]) -> Result<String> {
        let url = format!("{}{}", BASE_URL, path);
        let resp = self
            .client
            .post(&url)
            .form(params)
            .send()
            .await
            .with_context(|| format!("POSTリクエスト失敗: {}", url))?;
        let status = resp.status();
        let body = resp
            .text()
            .await
            .with_context(|| format!("レスポンスの読み取り失敗: {}", url))?;
        if !status.is_success() {
            anyhow::bail!("HTTPエラー {}: {}", status, url);
        }
        Ok(body)
    }

    /// GETリクエストを送信し、JSONレスポンスをデシリアライズして返す。
    pub async fn get_json<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", BASE_URL, path);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("GETリクエスト失敗: {}", url))?;
        let status = resp.status();
        if !status.is_success() {
            anyhow::bail!("HTTPエラー {}: {}", status, url);
        }
        let data = resp
            .json::<T>()
            .await
            .with_context(|| format!("JSONパース失敗: {}", url))?;
        Ok(data)
    }

    /// セッションCookieを取得する。
    pub fn session_cookie(&self) -> Option<&str> {
        self.session_cookie.as_deref()
    }
}
