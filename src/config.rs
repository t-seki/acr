pub mod global;
pub mod session;

use std::path::PathBuf;

use anyhow::Context;

/// Returns the acr config directory: `~/.config/acr/`
pub fn config_dir() -> anyhow::Result<PathBuf> {
    let dir = dirs::config_dir()
        .context("Could not determine config directory")?
        .join("acr");
    Ok(dir)
}
