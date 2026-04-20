use crate::config;
use crate::launcher::parse_command_spec;

/// Open a URL in the user's configured browser.
///
/// The configured `browser` string is split shell-style, so users can include
/// flags or quoted paths, e.g. `"chrome --new-window"` or
/// `'"/mnt/c/Program Files/Google/Chrome/Application/chrome.exe" --new-window'`.
pub fn open(url: &str) {
    let browser_spec = config::global::load()
        .map(|c| c.browser)
        .unwrap_or_else(|_| "xdg-open".to_string());

    let (program, mut args) = parse_command_spec(&browser_spec)
        .unwrap_or_else(|| ("xdg-open".to_string(), Vec::new()));
    args.push(url.to_string());

    let _ = std::process::Command::new(&program)
        .args(&args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
}
