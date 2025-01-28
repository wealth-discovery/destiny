use anyhow::{anyhow, Result};
use homedir::my_home;
use std::{fs::File, path::PathBuf};

pub fn home_dir() -> Result<PathBuf> {
    my_home()?.ok_or(anyhow!("user home directory not found"))
}

pub fn cache_dir() -> Result<PathBuf> {
    Ok(home_dir()?.join("destiny"))
}

pub fn create_dir(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn create_file(path: &PathBuf) -> Result<File> {
    Ok(std::fs::File::create(path)?)
}

pub fn delete_file(path: &PathBuf) -> Result<()> {
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    Ok(())
}
