use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const DEFAULT_TEMPLATE: &str = r#"use proconio::input;

fn main() {
    input! {
    }
}
"#;

/// グローバル設定 (~/.config/acrs/config.toml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    #[serde(default = "default_editor")]
    pub editor: String,
    #[serde(default = "default_browser")]
    pub browser: String,
}

fn default_editor() -> String {
    "vim".to_string()
}

fn default_browser() -> String {
    if cfg!(target_os = "macos") {
        "open".to_string()
    } else {
        "xdg-open".to_string()
    }
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            editor: default_editor(),
            browser: default_browser(),
        }
    }
}

/// acrs 設定ディレクトリのパスを返す (~/.config/acrs)
pub fn config_dir() -> Result<PathBuf> {
    let dir = dirs::config_dir()
        .context("ホームディレクトリが見つかりません")?
        .join("acrs");
    Ok(dir)
}

impl GlobalConfig {
    /// 設定ファイルを読み込む。存在しなければデフォルト値を返す。
    pub fn load() -> Result<Self> {
        let path = config_dir()?.join("config.toml");
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("設定ファイルの読み込みに失敗: {}", path.display()))?;
        let config: Self = toml::from_str(&content)
            .with_context(|| format!("設定ファイルのパースに失敗: {}", path.display()))?;
        Ok(config)
    }

    /// 設定ファイルに書き込む。
    pub fn save(&self) -> Result<()> {
        let dir = config_dir()?;
        std::fs::create_dir_all(&dir)
            .with_context(|| format!("設定ディレクトリの作成に失敗: {}", dir.display()))?;
        let path = dir.join("config.toml");
        let content = toml::to_string_pretty(self).context("設定のシリアライズに失敗")?;
        std::fs::write(&path, content)
            .with_context(|| format!("設定ファイルの書き込みに失敗: {}", path.display()))?;
        Ok(())
    }
}

/// テンプレートファイルのパスを返す
pub fn template_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("template.rs"))
}

/// テンプレートを読み込む。存在しなければデフォルトテンプレートを返す。
pub fn load_template() -> Result<String> {
    let path = template_path()?;
    if path.exists() {
        std::fs::read_to_string(&path)
            .with_context(|| format!("テンプレートの読み込みに失敗: {}", path.display()))
    } else {
        Ok(DEFAULT_TEMPLATE.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = GlobalConfig::default();
        assert_eq!(config.editor, "vim");
    }

    #[test]
    fn test_config_roundtrip() {
        let config = GlobalConfig {
            editor: "nvim".to_string(),
            browser: "firefox".to_string(),
        };
        let serialized = toml::to_string_pretty(&config).unwrap();
        let deserialized: GlobalConfig = toml::from_str(&serialized).unwrap();
        assert_eq!(deserialized.editor, "nvim");
        assert_eq!(deserialized.browser, "firefox");
    }

    #[test]
    fn test_default_template() {
        let template = DEFAULT_TEMPLATE;
        assert!(template.contains("proconio"));
        assert!(template.contains("fn main()"));
    }
}
