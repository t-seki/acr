use scraper::{Html, Selector};

use crate::error::AcrsError;

/// Extract CSRF token from HTML (input[name="csrf_token"])
pub fn extract_csrf_token(html: &str) -> anyhow::Result<String> {
    let document = Html::parse_document(html);
    let selector =
        Selector::parse(r#"input[name="csrf_token"]"#).expect("valid selector");
    document
        .select(&selector)
        .next()
        .and_then(|el| el.value().attr("value"))
        .map(|s| s.to_string())
        .ok_or_else(|| AcrsError::ScrapingFailed("csrf_token not found".to_string()).into())
}

/// Extract username from top page HTML (li a[href^="/users/"])
pub fn extract_username(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let selector =
        Selector::parse(r#"li a[href^="/users/"]"#).expect("valid selector");
    document
        .select(&selector)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_csrf_token() {
        let html = r#"<html><body>
            <form>
                <input type="hidden" name="csrf_token" value="abc123+def/456==" />
            </form>
        </body></html>"#;
        let token = extract_csrf_token(html).unwrap();
        assert_eq!(token, "abc123+def/456==");
    }

    #[test]
    fn test_extract_csrf_token_missing() {
        let html = "<html><body></body></html>";
        assert!(extract_csrf_token(html).is_err());
    }

    #[test]
    fn test_extract_username_logged_in() {
        let html = r#"<html><body>
            <ul>
                <li><a href="/users/testuser">testuser</a></li>
            </ul>
        </body></html>"#;
        assert_eq!(extract_username(html), Some("testuser".to_string()));
    }

    #[test]
    fn test_extract_username_not_logged_in() {
        let html = r#"<html><body>
            <ul>
                <li><a href="/login">Login</a></li>
            </ul>
        </body></html>"#;
        assert_eq!(extract_username(html), None);
    }
}
