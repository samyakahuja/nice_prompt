use std::path::PathBuf;
use std::fs::create_dir_all;
use anyhow::{anyhow, Result};

pub fn get_app_cache_path() -> Result<PathBuf> {
    let mut path = dirs::cache_dir().ok_or_else(|| anyhow!("failed to find os cache dir."))?;
    path.push(env!("CARGO_PKG_NAME"));
    create_dir_all(&path)?;
    Ok(path)
}
