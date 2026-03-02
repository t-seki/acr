use anyhow::{Context, Result};

use super::client::AtCoderClient;
use crate::config::Session;

/// HTMLからCSRFトークンを抽出する。
pub fn extract_csrf_token(html: &str) -> Result<String> {
    let document = scraper::Html::parse_document(html);
    let selector =
        scraper::Selector::parse("input[name=\"csrf_token\"]").expect("invalid selector");
    let element = document
        .select(&selector)
        .next()
        .context("CSRFトークンが見つかりません")?;
    let token = element
        .value()
        .attr("value")
        .context("CSRFトークンのvalue属性がありません")?;
    Ok(token.to_string())
}

/// AtCoderにログインし、セッションを保存する。
pub async fn login(username: &str, password: &str) -> Result<()> {
    let client = AtCoderClient::new()?;

    // ログインページからCSRFトークンを取得
    let login_page = client.get("/login").await?;
    let csrf_token = extract_csrf_token(&login_page)?;

    // ログインPOST
    let params = [
        ("username", username),
        ("password", password),
        ("csrf_token", &csrf_token),
    ];
    let result_html = client.post_form("/login", &params).await?;

    // ログイン成功判定: ユーザーリンクの存在を確認
    let document = scraper::Html::parse_document(&result_html);
    let selector = scraper::Selector::parse("li a[href^=\"/users/\"]").expect("invalid selector");
    if document.select(&selector).next().is_none() {
        anyhow::bail!("ログインに失敗しました。ユーザー名またはパスワードが正しくありません。");
    }

    // REVEL_SESSIONを取得して保存
    // cookie_storeからセッションCookieを取得
    // reqwestのcookie_storeでは直接Cookieを取得できないため、
    // レスポンスHTMLからログイン成功を確認し、再度GETしてCookie取得を試みる
    // 代替: ログインPOSTのSet-Cookieヘッダーを解析する
    //
    // NOTE: reqwest の cookie_store=true ではリダイレクト後に Set-Cookie が処理される。
    // ここでは post_form が成功した時点で cookie jar にセッションが格納されている。
    // ただし post_form は text() を返すだけなので、Cookieは直接取得できない。
    //
    // 別アプローチ: cookie_store を使わず、レスポンスのヘッダーから直接取得する。
    // AtCoderClient を拡張して raw response を返す方法を採用する。
    let revel_session = login_and_get_session(username, password, &csrf_token).await?;

    let session = Session { revel_session };
    session.save()?;
    Ok(())
}

/// ログインして REVEL_SESSION Cookie を取得する（低レベル実装）
async fn login_and_get_session(
    username: &str,
    password: &str,
    csrf_token: &str,
) -> Result<String> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .context("HTTPクライアントの作成に失敗")?;

    let params = [
        ("username", username),
        ("password", password),
        ("csrf_token", csrf_token),
    ];

    let resp = client
        .post("https://atcoder.jp/login")
        .form(&params)
        .send()
        .await
        .context("ログインリクエスト失敗")?;

    // Set-CookieヘッダーからREVEL_SESSIONを取得
    for cookie_str in resp.headers().get_all("set-cookie") {
        let cookie_val = cookie_str
            .to_str()
            .context("Cookie値のパース失敗")?;
        if let Some(value) = extract_revel_session(cookie_val) {
            return Ok(value);
        }
    }

    anyhow::bail!("REVEL_SESSIONの取得に失敗しました")
}

/// Set-Cookie ヘッダー文字列から REVEL_SESSION の値を抽出する。
fn extract_revel_session(set_cookie: &str) -> Option<String> {
    for part in set_cookie.split(';') {
        let trimmed = part.trim();
        if let Some(value) = trimmed.strip_prefix("REVEL_SESSION=") {
            return Some(value.to_string());
        }
    }
    None
}

/// セッションが有効か確認する。
pub async fn check_session() -> Result<bool> {
    if !Session::exists()? {
        return Ok(false);
    }
    let session = Session::load()?;
    let client = AtCoderClient::with_session(&session.revel_session)?;
    let html = client.get("/").await?;
    let document = scraper::Html::parse_document(&html);
    let selector = scraper::Selector::parse("li a[href^=\"/users/\"]").expect("invalid selector");
    Ok(document.select(&selector).next().is_some())
}

/// ログアウトする（セッションファイルを削除）。
pub fn logout() -> Result<()> {
    Session::delete()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_csrf_token() {
        let html = r#"
        <html>
        <body>
            <form>
                <input type="hidden" name="csrf_token" value="abc123+def456==" />
            </form>
        </body>
        </html>
        "#;
        let token = extract_csrf_token(html).unwrap();
        assert_eq!(token, "abc123+def456==");
    }

    #[test]
    fn test_extract_csrf_token_not_found() {
        let html = "<html><body></body></html>";
        assert!(extract_csrf_token(html).is_err());
    }

    #[test]
    fn test_extract_revel_session() {
        let cookie = "REVEL_SESSION=abc123; Path=/; HttpOnly";
        assert_eq!(
            extract_revel_session(cookie),
            Some("abc123".to_string())
        );
    }

    #[test]
    fn test_extract_revel_session_not_found() {
        let cookie = "OTHER_COOKIE=value; Path=/";
        assert_eq!(extract_revel_session(cookie), None);
    }
}
