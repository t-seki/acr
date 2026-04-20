use crate::config;
use crate::config::global::default_browser_command;
use crate::launcher::parse_command_spec;

/// Open a URL in the user's configured browser.
///
/// The configured `browser` string is split shell-style, so users can include
/// flags or quoted paths, e.g. `"chrome --new-window"` or
/// `"/mnt/c/Program Files/Google/Chrome/Application/chrome.exe" --new-window`.
///
/// Falls back to the platform-appropriate default (`open` on macOS,
/// `explorer` on Windows, `xdg-open` otherwise) if the config cannot be
/// loaded or the browser spec cannot be parsed (e.g. an unterminated quote).
/// In the parse-failure case a warning is written to stderr so the broken
/// config value is not silently ignored.
pub fn open(url: &str) {
    let browser_spec = config::global::load()
        .map(|c| c.browser)
        .unwrap_or_else(|_| default_browser_command().to_string());

    let (program, mut args) = parse_command_spec(&browser_spec).unwrap_or_else(|| {
        if !browser_spec.trim().is_empty() {
            eprintln!(
                "acr: could not parse browser config '{}', falling back to {}",
                browser_spec,
                default_browser_command()
            );
        }
        (default_browser_command().to_string(), Vec::new())
    });
    args.push(url.to_string());

    let _ = std::process::Command::new(&program)
        .args(&args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
}
