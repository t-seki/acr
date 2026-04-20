use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::cli::TemplateAction;
use crate::config;

pub async fn execute(action: TemplateAction) -> anyhow::Result<()> {
    match action {
        TemplateAction::Add { source } => add(&source).await,
        TemplateAction::Show => show(),
        TemplateAction::Reset => reset(),
    }
}

async fn add(source: &str) -> anyhow::Result<()> {
    let contents = fetch(source).await?;
    warn_if_not_rust(&contents);
    let path = config::global::template_path()?;
    backup_and_write_to(&path, &contents)?;
    println!("Saved template to {}", path.display());
    Ok(())
}

fn show() -> anyhow::Result<()> {
    print!("{}", config::global::load_template()?);
    Ok(())
}

fn reset() -> anyhow::Result<()> {
    let path = config::global::template_path()?;
    backup_and_write_to(&path, config::global::default_template())?;
    println!("Restored default template at {}", path.display());
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum Source {
    Url(String),
    Path(PathBuf),
}

async fn fetch(source: &str) -> anyhow::Result<String> {
    match normalize_source(source) {
        Source::Url(url) => {
            let resp = reqwest::get(&url)
                .await
                .with_context(|| format!("Failed to fetch {}", url))?
                .error_for_status()
                .with_context(|| format!("Non-success response from {}", url))?;
            resp.text()
                .await
                .with_context(|| format!("Failed to read body of {}", url))
        }
        Source::Path(path) => std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display())),
    }
}

/// Normalise a user-supplied source string into either a raw-content URL or a
/// local filesystem path. GitHub `blob/` URLs and Gist URLs are rewritten to
/// their raw-content equivalents; everything else is passed through as-is.
fn normalize_source(input: &str) -> Source {
    let trimmed = input.trim();
    if let Some(rest) = trimmed.strip_prefix("https://github.com/") {
        // https://github.com/<owner>/<repo>/blob/<ref>/<path>
        let parts: Vec<&str> = rest.splitn(5, '/').collect();
        if parts.len() == 5 && parts[2] == "blob" {
            return Source::Url(format!(
                "https://raw.githubusercontent.com/{}/{}/{}/{}",
                parts[0], parts[1], parts[3], parts[4]
            ));
        }
        return Source::Url(trimmed.to_string());
    }
    if let Some(rest) = trimmed.strip_prefix("https://gist.github.com/") {
        // Gist "pretty" URL: https://gist.github.com/<user>/<id>
        // Already-raw URL:    https://gist.github.com/<user>/<id>/raw/...
        if !rest.contains("/raw") {
            return Source::Url(format!("https://gist.github.com/{}/raw", rest));
        }
        return Source::Url(trimmed.to_string());
    }
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        return Source::Url(trimmed.to_string());
    }
    Source::Path(PathBuf::from(trimmed))
}

fn warn_if_not_rust(contents: &str) {
    if contents.contains("fn ") || contents.contains("use ") {
        return;
    }
    eprintln!(
        "acr: warning: fetched template doesn't look like Rust code (no `fn ` or `use ` found). Saving anyway."
    );
}

fn backup_and_write_to(path: &Path, contents: &str) -> anyhow::Result<()> {
    if path.exists() {
        let backup = path.with_extension("rs.bak");
        std::fs::rename(path, &backup)
            .with_context(|| format!("Failed to back up to {}", backup.display()))?;
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }
    std::fs::write(path, contents)
        .with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_github_blob_url() {
        let input = "https://github.com/owner/repo/blob/main/templates/atcoder.rs";
        assert_eq!(
            normalize_source(input),
            Source::Url(
                "https://raw.githubusercontent.com/owner/repo/main/templates/atcoder.rs"
                    .to_string()
            )
        );
    }

    #[test]
    fn test_normalize_gist_pretty_url() {
        let input = "https://gist.github.com/someone/abcdef1234";
        assert_eq!(
            normalize_source(input),
            Source::Url("https://gist.github.com/someone/abcdef1234/raw".to_string())
        );
    }

    #[test]
    fn test_normalize_gist_raw_url_passthrough() {
        let input = "https://gist.github.com/someone/abcdef1234/raw/rev/file.rs";
        assert_eq!(normalize_source(input), Source::Url(input.to_string()));
    }

    #[test]
    fn test_normalize_raw_githubusercontent_passthrough() {
        let input = "https://raw.githubusercontent.com/owner/repo/main/x.rs";
        assert_eq!(normalize_source(input), Source::Url(input.to_string()));
    }

    #[test]
    fn test_normalize_plain_http_url_passthrough() {
        let input = "https://example.com/template.rs";
        assert_eq!(normalize_source(input), Source::Url(input.to_string()));
    }

    #[test]
    fn test_normalize_local_relative_path() {
        assert_eq!(
            normalize_source("./my_template.rs"),
            Source::Path(PathBuf::from("./my_template.rs"))
        );
    }

    #[test]
    fn test_normalize_local_absolute_path() {
        assert_eq!(
            normalize_source("/tmp/t.rs"),
            Source::Path(PathBuf::from("/tmp/t.rs"))
        );
    }

    #[test]
    fn test_normalize_bare_filename_is_path() {
        assert_eq!(
            normalize_source("template.rs"),
            Source::Path(PathBuf::from("template.rs"))
        );
    }

    #[test]
    fn test_warn_if_not_rust_accepts_rust_code() {
        // No panic / stderr for valid code
        warn_if_not_rust("fn main() {}");
        warn_if_not_rust("use proconio::input;\n");
    }

    #[test]
    fn test_backup_and_write_new_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("template.rs");
        backup_and_write_to(&path, "fn main() {}").unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "fn main() {}");
        assert!(!path.with_extension("rs.bak").exists());
    }

    #[test]
    fn test_backup_and_write_replaces_existing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("template.rs");
        std::fs::write(&path, "OLD").unwrap();

        backup_and_write_to(&path, "NEW").unwrap();

        let backup = path.with_extension("rs.bak");
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "NEW");
        assert_eq!(std::fs::read_to_string(&backup).unwrap(), "OLD");
    }

    #[test]
    fn test_backup_and_write_creates_parent_dir() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("template.rs");
        backup_and_write_to(&path, "contents").unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "contents");
    }
}
