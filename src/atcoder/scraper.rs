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

/// Extract sample input/output pairs from a problem page.
/// Looks for <pre> elements inside #task-statement, pairing consecutive input/output.
pub fn extract_sample_cases(html: &str) -> anyhow::Result<Vec<(String, String)>> {
    let document = Html::parse_document(html);
    let section_selector = Selector::parse("#task-statement .part").expect("valid selector");
    let pre_selector = Selector::parse("pre").expect("valid selector");
    let h3_selector = Selector::parse("h3").expect("valid selector");

    let mut inputs = Vec::new();
    let mut outputs = Vec::new();

    for section in document.select(&section_selector) {
        let h3_text = section
            .select(&h3_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();

        if let Some(pre) = section.select(&pre_selector).next() {
            let text = pre.text().collect::<String>();
            if h3_text.contains("Input") || h3_text.contains("入力") {
                inputs.push(text);
            } else if h3_text.contains("Output") || h3_text.contains("出力") {
                outputs.push(text);
            }
        }
    }

    let pairs: Vec<(String, String)> = inputs
        .into_iter()
        .zip(outputs)
        .collect();

    if pairs.is_empty() {
        return Err(
            AcrsError::ScrapingFailed("No sample cases found".to_string()).into(),
        );
    }

    Ok(pairs)
}

/// Extract Rust language_id from the submit page HTML.
/// Looks for <option> containing "Rust" in the language select.
pub fn extract_rust_language_id(html: &str) -> anyhow::Result<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(r#"select[name="data.LanguageId"] option"#)
        .expect("valid selector");

    let mut rust_options: Vec<(String, String)> = document
        .select(&selector)
        .filter_map(|el| {
            let text = el.text().collect::<String>();
            if text.contains("Rust") {
                el.value().attr("value").map(|v| (v.to_string(), text))
            } else {
                None
            }
        })
        .collect();

    // Sort by text descending to pick the latest version
    rust_options.sort_by(|a, b| b.1.cmp(&a.1));

    rust_options
        .first()
        .map(|(id, _)| id.clone())
        .ok_or_else(|| AcrsError::ScrapingFailed("Rust language not found in submit page".to_string()).into())
}

/// Extract the latest submission status from the submissions/me page.
/// Returns (status, submission_url).
pub fn extract_latest_submission_status(
    html: &str,
    contest_id: &str,
) -> anyhow::Result<Option<(String, String)>> {
    let document = Html::parse_document(html);
    let row_selector = Selector::parse("table tbody tr").expect("valid selector");
    let td_selector = Selector::parse("td").expect("valid selector");
    let a_selector = Selector::parse("a").expect("valid selector");

    if let Some(row) = document.select(&row_selector).next() {
        let tds: Vec<_> = row.select(&td_selector).collect();
        // The status is typically in the 7th column (index 6)
        if let Some(status_td) = tds.get(6) {
            let status = status_td.text().collect::<String>().trim().to_string();

            // Extract submission URL from the detail link
            let submission_url = tds
                .last()
                .and_then(|td| td.select(&a_selector).next())
                .and_then(|a| a.value().attr("href"))
                .map(|href| {
                    if href.starts_with("http") {
                        href.to_string()
                    } else {
                        format!("https://atcoder.jp{}", href)
                    }
                })
                .unwrap_or_else(|| {
                    format!(
                        "https://atcoder.jp/contests/{}/submissions/me",
                        contest_id
                    )
                });

            return Ok(Some((status, submission_url)));
        }
    }

    Ok(None)
}

/// Extract username from top page HTML (li a[href^="/users/"])
pub fn extract_username(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let selector =
        Selector::parse(r#"li a[href^="/users/"]"#).expect("valid selector");
    document
        .select(&selector)
        .next()
        .and_then(|el| el.value().attr("href"))
        .and_then(|href| href.strip_prefix("/users/"))
        .map(|s| s.to_string())
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
                <li><a href="/users/testuser">My Profile</a></li>
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

    #[test]
    fn test_extract_sample_cases() {
        let html = r#"<html><body>
            <div id="task-statement">
                <span class="lang-en">
                    <div class="part">
                        <h3>Sample Input 1</h3>
                        <pre>3 5
</pre>
                    </div>
                    <div class="part">
                        <h3>Sample Output 1</h3>
                        <pre>8
</pre>
                    </div>
                    <div class="part">
                        <h3>Sample Input 2</h3>
                        <pre>10 20
</pre>
                    </div>
                    <div class="part">
                        <h3>Sample Output 2</h3>
                        <pre>30
</pre>
                    </div>
                </span>
            </div>
        </body></html>"#;
        let cases = extract_sample_cases(html).unwrap();
        assert_eq!(cases.len(), 2);
        assert_eq!(cases[0].0, "3 5\n");
        assert_eq!(cases[0].1, "8\n");
        assert_eq!(cases[1].0, "10 20\n");
        assert_eq!(cases[1].1, "30\n");
    }

    #[test]
    fn test_extract_sample_cases_no_samples() {
        let html = r#"<html><body><div id="task-statement"></div></body></html>"#;
        assert!(extract_sample_cases(html).is_err());
    }

    #[test]
    fn test_extract_rust_language_id() {
        let html = r#"<html><body>
            <select name="data.LanguageId">
                <option value="4050">Rust (1.42.0)</option>
                <option value="5054">Rust (rustc 1.70.0)</option>
                <option value="5055">Rust (rustc 1.86.0)</option>
                <option value="4003">C++ 17 (gcc 12.2)</option>
            </select>
        </body></html>"#;
        let id = extract_rust_language_id(html).unwrap();
        assert_eq!(id, "5055"); // Latest version should be picked
    }

    #[test]
    fn test_extract_rust_language_id_not_found() {
        let html = r#"<html><body>
            <select name="data.LanguageId">
                <option value="4003">C++ 17 (gcc 12.2)</option>
            </select>
        </body></html>"#;
        assert!(extract_rust_language_id(html).is_err());
    }

    #[test]
    fn test_extract_latest_submission_status() {
        let html = r#"<html><body>
            <table>
                <tbody>
                    <tr>
                        <td>2024-01-01</td>
                        <td>abc001_a</td>
                        <td>user</td>
                        <td>Rust</td>
                        <td>100</td>
                        <td>10 Byte</td>
                        <td><span class="label label-success">AC</span></td>
                        <td><a href="/contests/abc001/submissions/12345">Detail</a></td>
                    </tr>
                </tbody>
            </table>
        </body></html>"#;
        let result = extract_latest_submission_status(html, "abc001").unwrap();
        assert!(result.is_some());
        let (status, url) = result.unwrap();
        assert_eq!(status, "AC");
        assert!(url.contains("/contests/abc001/submissions/12345"));
    }
}
