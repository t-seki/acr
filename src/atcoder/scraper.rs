use scraper::{Html, Selector};

/// Extract sample input/output pairs from a problem page.
/// Targets the English section first, falls back to Japanese if not found.
pub fn extract_sample_cases(html: &str) -> anyhow::Result<Vec<(String, String)>> {
    let document = Html::parse_document(html);

    // Try English section first, then Japanese, then whole task-statement
    let lang_selectors = [
        "#task-statement .lang-en .part",
        "#task-statement .lang-ja .part",
        "#task-statement .part",
    ];

    for lang_sel in &lang_selectors {
        let pairs = extract_samples_from(&document, lang_sel);
        if !pairs.is_empty() {
            return Ok(pairs);
        }
    }

    Ok(vec![])
}

fn extract_samples_from(document: &Html, section_css: &str) -> Vec<(String, String)> {
    let section_selector = Selector::parse(section_css).expect("valid selector");
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
            if h3_text.contains("Sample Input") || h3_text.contains("入力例") {
                inputs.push(text);
            } else if h3_text.contains("Sample Output") || h3_text.contains("出力例") {
                outputs.push(text);
            }
        }
    }

    inputs.into_iter().zip(outputs).collect()
}

/// Extract task list from `/contests/{contest_id}/tasks` page.
/// Returns a list of (alphabet, task_name, task_screen_name).
pub fn extract_task_list(html: &str) -> Vec<(String, String, String)> {
    let document = Html::parse_document(html);
    let tr_selector = Selector::parse("table tbody tr").expect("valid selector");
    let td_selector = Selector::parse("td").expect("valid selector");
    let a_selector = Selector::parse("a").expect("valid selector");

    let mut tasks = Vec::new();

    for row in document.select(&tr_selector) {
        let tds: Vec<_> = row.select(&td_selector).collect();
        if tds.len() < 2 {
            continue;
        }

        let alphabet = tds[0].text().collect::<String>().trim().to_string();
        if alphabet.is_empty() {
            continue;
        }

        if let Some(link) = tds[1].select(&a_selector).next() {
            let task_name = link.text().collect::<String>().trim().to_string();
            if let Some(href) = link.value().attr("href") {
                // href is like "/contests/{contest_id}/tasks/{task_screen_name}"
                if let Some(task_screen_name) = href.rsplit('/').next() {
                    tasks.push((alphabet, task_name, task_screen_name.to_string()));
                }
            }
        }
    }

    tasks
}

/// Extract username from top page HTML (li a[href^="/users/"])
pub fn extract_username(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(r#"li a[href^="/users/"]"#).expect("valid selector");
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
    fn test_extract_sample_cases_ignores_format_description() {
        // Reproduces real AtCoder HTML where "入力" (Input format) section
        // contains <var> tags like N M, which should NOT be treated as samples.
        let html = r#"<html><body>
            <div id="task-statement">
                <span class="lang-ja">
                    <div class="io-style">
                        <div class="part">
                            <section>
                                <h3>入力</h3>
                                <pre><var>N</var> <var>M</var>
</pre>
                            </section>
                        </div>
                        <div class="part">
                            <section>
                                <h3>出力</h3>
                                <pre>Yes or No</pre>
                            </section>
                        </div>
                    </div>
                    <div class="part">
                        <section>
                            <h3>入力例 1</h3>
                            <pre>6 3
</pre>
                        </section>
                    </div>
                    <div class="part">
                        <section>
                            <h3>出力例 1</h3>
                            <pre>Yes
</pre>
                        </section>
                    </div>
                    <div class="part">
                        <section>
                            <h3>入力例 2</h3>
                            <pre>4 3
</pre>
                        </section>
                    </div>
                    <div class="part">
                        <section>
                            <h3>出力例 2</h3>
                            <pre>No
</pre>
                        </section>
                    </div>
                </span>
            </div>
        </body></html>"#;
        let cases = extract_sample_cases(html).unwrap();
        assert_eq!(cases.len(), 2);
        assert_eq!(cases[0].0, "6 3\n");
        assert_eq!(cases[0].1, "Yes\n");
        assert_eq!(cases[1].0, "4 3\n");
        assert_eq!(cases[1].1, "No\n");
    }

    #[test]
    fn test_extract_sample_cases_no_samples() {
        let html = r#"<html><body><div id="task-statement"></div></body></html>"#;
        let cases = extract_sample_cases(html).unwrap();
        assert!(cases.is_empty());
    }

    #[test]
    fn test_extract_task_list_abc() {
        let html = r#"<html><body>
            <table>
                <thead><tr><th>Task</th><th>Name</th></tr></thead>
                <tbody>
                    <tr>
                        <td>A</td>
                        <td><a href="/contests/abc001/tasks/abc001_1">積雪深差</a></td>
                        <td>2 sec</td>
                        <td>64 MiB</td>
                    </tr>
                    <tr>
                        <td>B</td>
                        <td><a href="/contests/abc001/tasks/abc001_2">視程の通報</a></td>
                        <td>2 sec</td>
                        <td>64 MiB</td>
                    </tr>
                </tbody>
            </table>
        </body></html>"#;
        let tasks = extract_task_list(html);
        assert_eq!(tasks.len(), 2);
        assert_eq!(
            tasks[0],
            (
                "A".to_string(),
                "積雪深差".to_string(),
                "abc001_1".to_string()
            )
        );
        assert_eq!(
            tasks[1],
            (
                "B".to_string(),
                "視程の通報".to_string(),
                "abc001_2".to_string()
            )
        );
    }

    #[test]
    fn test_extract_task_list_numeric_labels() {
        let html = r#"<html><body>
            <table>
                <thead><tr><th>Task</th><th>Name</th></tr></thead>
                <tbody>
                    <tr>
                        <td>001</td>
                        <td><a href="/contests/typical90/tasks/typical90_a">Yokan Party（★4）</a></td>
                        <td>2 sec</td>
                        <td>1024 MiB</td>
                    </tr>
                    <tr>
                        <td>002</td>
                        <td><a href="/contests/typical90/tasks/typical90_b">Encyclopedia of Parentheses（★3）</a></td>
                        <td>2 sec</td>
                        <td>1024 MiB</td>
                    </tr>
                </tbody>
            </table>
        </body></html>"#;
        let tasks = extract_task_list(html);
        assert_eq!(tasks.len(), 2);
        assert_eq!(
            tasks[0],
            (
                "001".to_string(),
                "Yokan Party（★4）".to_string(),
                "typical90_a".to_string()
            )
        );
        assert_eq!(
            tasks[1],
            (
                "002".to_string(),
                "Encyclopedia of Parentheses（★3）".to_string(),
                "typical90_b".to_string()
            )
        );
    }
}
