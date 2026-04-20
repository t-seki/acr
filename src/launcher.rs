/// Parse a shell-style command spec into `(program, args)`.
///
/// Used for `editor` and `browser` config values so users can append flags
/// (e.g. `"code --new-window"`) and quote paths containing spaces
/// (e.g. `'"/mnt/c/Program Files/Google/Chrome/Application/chrome.exe" --new-window'`).
///
/// Returns `None` when the input has no tokens.
pub fn parse_command_spec(spec: &str) -> Option<(String, Vec<String>)> {
    let mut parts = shlex::split(spec)?.into_iter();
    let program = parts.next()?;
    Some((program, parts.collect()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_command() {
        let (prog, args) = parse_command_spec("xdg-open").unwrap();
        assert_eq!(prog, "xdg-open");
        assert!(args.is_empty());
    }

    #[test]
    fn test_parse_with_flag() {
        let (prog, args) = parse_command_spec("google-chrome --new-window").unwrap();
        assert_eq!(prog, "google-chrome");
        assert_eq!(args, vec!["--new-window".to_string()]);
    }

    #[test]
    fn test_parse_with_quoted_path() {
        let spec = r#""/mnt/c/Program Files/Google/Chrome/Application/chrome.exe" --new-window"#;
        let (prog, args) = parse_command_spec(spec).unwrap();
        assert_eq!(prog, "/mnt/c/Program Files/Google/Chrome/Application/chrome.exe");
        assert_eq!(args, vec!["--new-window".to_string()]);
    }

    #[test]
    fn test_parse_multiple_flags() {
        let (prog, args) = parse_command_spec("firefox --new-window --private").unwrap();
        assert_eq!(prog, "firefox");
        assert_eq!(args, vec!["--new-window".to_string(), "--private".to_string()]);
    }

    #[test]
    fn test_parse_empty_returns_none() {
        assert!(parse_command_spec("").is_none());
        assert!(parse_command_spec("   ").is_none());
    }
}
