use anyhow::{Context, Result};

use super::client::AtCoderClient;
use crate::types::TestCase;

/// 問題ページからテストケースを取得する。
pub async fn fetch_test_cases(
    client: &AtCoderClient,
    contest_id: &str,
    task_screen_name: &str,
) -> Result<Vec<TestCase>> {
    let path = format!("/contests/{}/tasks/{}", contest_id, task_screen_name);
    let html = client.get(&path).await?;
    parse_test_cases(&html)
}

/// 問題ページHTMLからテストケースをパースする。
pub fn parse_test_cases(html: &str) -> Result<Vec<TestCase>> {
    let document = scraper::Html::parse_document(html);
    let pre_selector = scraper::Selector::parse("#task-statement pre").expect("invalid selector");

    let pres: Vec<String> = document
        .select(&pre_selector)
        .map(|el| el.text().collect::<String>())
        .collect();

    // AtCoderの問題ページでは、サンプル入出力が交互に並ぶ:
    // 入力例1, 出力例1, 入力例2, 出力例2, ...
    // <h3> タグで「入力例」「出力例」を識別するのがより正確
    let h3_selector = scraper::Selector::parse("#task-statement h3").expect("invalid selector");
    let section_selector =
        scraper::Selector::parse("#task-statement .part").expect("invalid selector");

    let mut inputs = Vec::new();
    let mut outputs = Vec::new();

    for section in document.select(&section_selector) {
        let h3 = section.select(&h3_selector).next();
        let pre = section
            .select(&scraper::Selector::parse("pre").expect("invalid selector"))
            .next();

        if let (Some(h3_el), Some(pre_el)) = (h3, pre) {
            let heading = h3_el.text().collect::<String>();
            let content = pre_el.text().collect::<String>();

            if heading.contains("入力例") || heading.contains("Sample Input") {
                inputs.push(content);
            } else if heading.contains("出力例") || heading.contains("Sample Output") {
                outputs.push(content);
            }
        }
    }

    // .part セクションが見つからない場合、<pre> タグの交互パターンにフォールバック
    if inputs.is_empty() && outputs.is_empty() && pres.len() >= 2 {
        for i in (0..pres.len() - 1).step_by(2) {
            inputs.push(pres[i].clone());
            outputs.push(pres[i + 1].clone());
        }
    }

    let test_cases: Vec<TestCase> = inputs
        .into_iter()
        .zip(outputs.into_iter())
        .enumerate()
        .map(|(i, (input, expected))| TestCase {
            index: i + 1,
            input,
            expected,
        })
        .collect();

    Ok(test_cases)
}

/// 提出ページから Rust の language_id を取得する。
pub async fn fetch_language_id(client: &AtCoderClient, contest_id: &str) -> Result<String> {
    let path = format!("/contests/{}/submit", contest_id);
    let html = client.get(&path).await?;
    parse_language_id(&html)
}

/// 提出ページHTMLから Rust の language_id をパースする。
pub fn parse_language_id(html: &str) -> Result<String> {
    let document = scraper::Html::parse_document(html);
    let selector = scraper::Selector::parse("select[name=\"data.LanguageId\"] option")
        .expect("invalid selector");

    for option in document.select(&selector) {
        let text = option.text().collect::<String>();
        if text.contains("Rust") {
            let value = option
                .value()
                .attr("value")
                .context("language option に value 属性がありません")?;
            return Ok(value.to_string());
        }
    }

    anyhow::bail!("Rust の言語IDが見つかりません。提出ページを確認してください。")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_test_cases_with_sections() {
        let html = r#"
        <div id="task-statement">
            <div class="part">
                <h3>入力例 1</h3>
                <pre>3
1 2 3</pre>
            </div>
            <div class="part">
                <h3>出力例 1</h3>
                <pre>6</pre>
            </div>
            <div class="part">
                <h3>入力例 2</h3>
                <pre>5
10 20 30 40 50</pre>
            </div>
            <div class="part">
                <h3>出力例 2</h3>
                <pre>150</pre>
            </div>
        </div>
        "#;

        let cases = parse_test_cases(html).unwrap();
        assert_eq!(cases.len(), 2);
        assert_eq!(cases[0].index, 1);
        assert_eq!(cases[0].input, "3\n1 2 3");
        assert_eq!(cases[0].expected, "6");
        assert_eq!(cases[1].index, 2);
        assert_eq!(cases[1].input, "5\n10 20 30 40 50");
        assert_eq!(cases[1].expected, "150");
    }

    #[test]
    fn test_parse_test_cases_english() {
        let html = r#"
        <div id="task-statement">
            <div class="part">
                <h3>Sample Input 1</h3>
                <pre>1 2</pre>
            </div>
            <div class="part">
                <h3>Sample Output 1</h3>
                <pre>3</pre>
            </div>
        </div>
        "#;

        let cases = parse_test_cases(html).unwrap();
        assert_eq!(cases.len(), 1);
        assert_eq!(cases[0].input, "1 2");
        assert_eq!(cases[0].expected, "3");
    }

    #[test]
    fn test_parse_language_id() {
        let html = r#"
        <select name="data.LanguageId">
            <option value="5001">C++ 23 (gcc 12.2)</option>
            <option value="5054">Rust (rustc 1.70.0)</option>
            <option value="5018">Python (CPython 3.11.4)</option>
        </select>
        "#;

        let id = parse_language_id(html).unwrap();
        assert_eq!(id, "5054");
    }

    #[test]
    fn test_parse_language_id_not_found() {
        let html = r#"
        <select name="data.LanguageId">
            <option value="5001">C++ 23 (gcc 12.2)</option>
        </select>
        "#;

        assert!(parse_language_id(html).is_err());
    }
}
