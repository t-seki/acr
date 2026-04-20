use crate::config;

/// Open a URL in the user's configured browser.
///
/// The configured `browser` string is split shell-style, so users can include
/// flags or quoted paths, e.g. `"chrome --new-window"` or
/// `'"/mnt/c/Program Files/Google/Chrome/Application/chrome.exe" --new-window'`.
pub fn open(url: &str) {
    let browser_spec = config::global::load()
        .map(|c| c.browser)
        .unwrap_or_else(|_| "xdg-open".to_string());

    let (program, mut args) = parse_browser_command(&browser_spec)
        .unwrap_or_else(|| ("xdg-open".to_string(), Vec::new()));
    args.push(url.to_string());

    let _ = std::process::Command::new(&program)
        .args(&args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
}

fn parse_browser_command(spec: &str) -> Option<(String, Vec<String>)> {
    let mut parts = shlex::split(spec)?.into_iter();
    let program = parts.next()?;
    Some((program, parts.collect()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_command() {
        let (prog, args) = parse_browser_command("xdg-open").unwrap();
        assert_eq!(prog, "xdg-open");
        assert!(args.is_empty());
    }

    #[test]
    fn test_parse_with_flag() {
        let (prog, args) = parse_browser_command("google-chrome --new-window").unwrap();
        assert_eq!(prog, "google-chrome");
        assert_eq!(args, vec!["--new-window".to_string()]);
    }

    #[test]
    fn test_parse_with_quoted_path() {
        let spec = r#""/mnt/c/Program Files/Google/Chrome/Application/chrome.exe" --new-window"#;
        let (prog, args) = parse_browser_command(spec).unwrap();
        assert_eq!(prog, "/mnt/c/Program Files/Google/Chrome/Application/chrome.exe");
        assert_eq!(args, vec!["--new-window".to_string()]);
    }

    #[test]
    fn test_parse_multiple_flags() {
        let (prog, args) = parse_browser_command("firefox --new-window --private").unwrap();
        assert_eq!(prog, "firefox");
        assert_eq!(args, vec!["--new-window".to_string(), "--private".to_string()]);
    }

    #[test]
    fn test_parse_empty_returns_none() {
        assert!(parse_browser_command("").is_none());
        assert!(parse_browser_command("   ").is_none());
    }
}
