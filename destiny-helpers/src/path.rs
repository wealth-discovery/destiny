use anyhow::{anyhow, Result};
use homedir::my_home;
use std::path::PathBuf;

pub fn home_dir() -> Result<PathBuf> {
    my_home()?.ok_or(anyhow!("user home directory not found"))
}

pub fn cache_dir() -> Result<PathBuf> {
    Ok(home_dir()?.join(".bagua"))
}
