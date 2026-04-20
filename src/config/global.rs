use std::path::{Path, PathBuf};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::error::AcrError;

const DEFAULT_TEMPLATE: &str = r#"#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet, VecDeque};
use proconio::input;

fn main() {
    input! {
    }
}
"#;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct GlobalConfig {
    pub editor: String,
    pub browser: String,
    /// AtCoder language ID used by `acr submit`. Defaults to current Rust ID.
    pub language_id: String,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            editor: "vim".to_string(),
            browser: "xdg-open".to_string(),
            language_id: "5054".to_string(),
        }
    }
}

// --- Internal functions (path-parameterized for testability) ---

fn load_from(path: &Path) -> anyhow::Result<GlobalConfig> {
    let content = std::fs::read_to_string(path).map_err(|_| AcrError::ConfigNotFound)?;
    let config: GlobalConfig = toml::from_str(&content)
        .with_context(|| format!("Failed to parse config: {}", path.display()))?;
    Ok(config)
}

fn save_to(path: &Path, config: &GlobalConfig) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }
    let content =
        toml::to_string(config).with_context(|| "Failed to serialize config to TOML")?;
    std::fs::write(path, content)
        .with_context(|| format!("Failed to write config: {}", path.display()))?;
    Ok(())
}

fn load_template_from(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|_| DEFAULT_TEMPLATE.to_string())
}

// --- Public API (uses default paths) ---

fn config_path() -> anyhow::Result<PathBuf> {
    Ok(crate::config::config_dir()?.join("config.toml"))
}

pub fn template_path() -> anyhow::Result<PathBuf> {
    Ok(crate::config::config_dir()?.join("template.rs"))
}

pub fn load() -> anyhow::Result<GlobalConfig> {
    load_from(&config_path()?)
}

pub fn save(config: &GlobalConfig) -> anyhow::Result<()> {
    save_to(&config_path()?, config)
}

pub fn load_template() -> anyhow::Result<String> {
    Ok(load_template_from(&template_path()?))
}

pub fn default_template() -> &'static str {
    DEFAULT_TEMPLATE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = GlobalConfig::default();
        assert_eq!(config.editor, "vim");
        assert_eq!(config.browser, "xdg-open");
        assert_eq!(config.language_id, "5054");
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");

        let config = GlobalConfig {
            editor: "nvim".to_string(),
            browser: "firefox".to_string(),
            language_id: "9999".to_string(),
        };
        save_to(&path, &config).unwrap();
        let loaded = load_from(&path).unwrap();

        assert_eq!(loaded.editor, "nvim");
        assert_eq!(loaded.browser, "firefox");
        assert_eq!(loaded.language_id, "9999");
    }

    #[test]
    fn test_load_legacy_config_without_language_id() {
        // Old config files predate the language_id field; loading should fall
        // back to the default rather than erroring.
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        std::fs::write(&path, "editor = \"nvim\"\nbrowser = \"firefox\"\n").unwrap();

        let loaded = load_from(&path).unwrap();
        assert_eq!(loaded.editor, "nvim");
        assert_eq!(loaded.browser, "firefox");
        assert_eq!(loaded.language_id, "5054");
    }

    #[test]
    fn test_load_missing_file_returns_config_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nonexistent.toml");

        let err = load_from(&path).unwrap_err();
        assert!(err.downcast_ref::<AcrError>().is_some());
    }

    #[test]
    fn test_load_template_returns_default_when_missing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("template.rs");

        let template = load_template_from(&path);
        assert_eq!(template, DEFAULT_TEMPLATE);
    }

    #[test]
    fn test_load_template_from_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("template.rs");

        let custom = "fn main() {}";
        std::fs::write(&path, custom).unwrap();

        let template = load_template_from(&path);
        assert_eq!(template, custom);
    }
}
