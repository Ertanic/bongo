use anyhow::Context;
use std::path::PathBuf;

pub fn get_current_folder() -> anyhow::Result<PathBuf> {
    Ok(std::env::current_exe()?
        .parent()
        .context("Unable to get a parent of current exe")?
        .to_path_buf())
}
