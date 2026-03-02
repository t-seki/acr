pub mod global;
pub mod session;

use std::path::PathBuf;

use anyhow::Context;

/// Returns the acrs config directory: `~/.config/acrs/`
pub fn config_dir() -> anyhow::Result<PathBuf> {
    let dir = dirs::config_dir()
        .context("Could not determine config directory")?
        .join("acrs");
    Ok(dir)
}
