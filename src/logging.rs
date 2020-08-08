use simplelog::{WriteLogger, LevelFilter, Config};
use anyhow::Result;
use std::fs::File;

use crate::util::get_app_cache_path;

pub fn setup_logging() -> Result<()> {
    let mut path = get_app_cache_path()?;
    path.push(format!("{}.log", env!("CARGO_PKG_NAME")));

    let _ = WriteLogger::init(
        LevelFilter::max(),
        Config::default(),
        File::create(path)?,
    );

    Ok(())
}
