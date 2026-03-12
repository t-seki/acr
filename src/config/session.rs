use std::path::{Path, PathBuf};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::error::AcrError;

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionConfig {
    pub revel_session: String,
}

// --- Internal functions (path-parameterized for testability) ---

fn load_from(path: &Path) -> anyhow::Result<SessionConfig> {
    let content = std::fs::read_to_string(path).map_err(|_| AcrError::NotLoggedIn)?;
    let session: SessionConfig = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse session: {}", path.display()))?;
    Ok(session)
}

fn save_to(path: &Path, session: &SessionConfig) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }
    let content =
        serde_json::to_string_pretty(session).with_context(|| "Failed to serialize session")?;
    std::fs::write(path, content)
        .with_context(|| format!("Failed to write session: {}", path.display()))?;
    Ok(())
}

fn delete_at(path: &Path) -> anyhow::Result<()> {
    if path.exists() {
        std::fs::remove_file(path)
            .with_context(|| format!("Failed to delete session: {}", path.display()))?;
    }
    Ok(())
}

// --- Public API (uses default paths) ---

fn session_path() -> anyhow::Result<PathBuf> {
    Ok(crate::config::config_dir()?.join("session.json"))
}

pub fn load() -> anyhow::Result<SessionConfig> {
    load_from(&session_path()?)
}

pub fn save(session: &SessionConfig) -> anyhow::Result<()> {
    save_to(&session_path()?, session)
}

pub fn delete() -> anyhow::Result<()> {
    delete_at(&session_path()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_load_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("session.json");

        let session = SessionConfig {
            revel_session: "test_session_value".to_string(),
        };
        save_to(&path, &session).unwrap();
        let loaded = load_from(&path).unwrap();

        assert_eq!(loaded.revel_session, "test_session_value");
    }

    #[test]
    fn test_load_missing_file_returns_not_logged_in() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nonexistent.json");

        let err = load_from(&path).unwrap_err();
        assert!(err.downcast_ref::<AcrError>().is_some());
    }

    #[test]
    fn test_delete_existing_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("session.json");

        std::fs::write(&path, "{}").unwrap();
        assert!(path.exists());

        delete_at(&path).unwrap();
        assert!(!path.exists());
    }

    #[test]
    fn test_delete_nonexistent_file_is_ok() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nonexistent.json");

        delete_at(&path).unwrap();
    }

}
