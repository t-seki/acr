use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use super::global::config_dir;

/// セッション情報 (~/.config/acrs/session.json)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub revel_session: String,
}

impl Session {
    /// セッションファイルを読み込む。
    pub fn load() -> Result<Self> {
        let path = config_dir()?.join("session.json");
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("セッションファイルの読み込みに失敗: {}", path.display()))?;
        let session: Self = serde_json::from_str(&content)
            .with_context(|| format!("セッションファイルのパースに失敗: {}", path.display()))?;
        Ok(session)
    }

    /// セッションファイルに書き込む。
    pub fn save(&self) -> Result<()> {
        let dir = config_dir()?;
        std::fs::create_dir_all(&dir)
            .with_context(|| format!("設定ディレクトリの作成に失敗: {}", dir.display()))?;
        let path = dir.join("session.json");
        let content =
            serde_json::to_string_pretty(self).context("セッションのシリアライズに失敗")?;
        std::fs::write(&path, content)
            .with_context(|| format!("セッションファイルの書き込みに失敗: {}", path.display()))?;
        Ok(())
    }

    /// セッションファイルを削除する。
    pub fn delete() -> Result<()> {
        let path = config_dir()?.join("session.json");
        if path.exists() {
            std::fs::remove_file(&path).with_context(|| {
                format!("セッションファイルの削除に失敗: {}", path.display())
            })?;
        }
        Ok(())
    }

    /// セッションファイルが存在するか確認する。
    pub fn exists() -> Result<bool> {
        let path = config_dir()?.join("session.json");
        Ok(path.exists())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_roundtrip() {
        let session = Session {
            revel_session: "test_token_123".to_string(),
        };
        let json = serde_json::to_string_pretty(&session).unwrap();
        let deserialized: Session = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.revel_session, "test_token_123");
    }
}
