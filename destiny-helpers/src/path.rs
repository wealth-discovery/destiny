use anyhow::{anyhow, Result};
use homedir::my_home;
use std::path::PathBuf;

/// 获取用户主目录
pub fn home_dir() -> Result<PathBuf> {
    my_home()?.ok_or(anyhow!("用户主目录未找到"))
}

/// 获取缓存目录
pub fn cache_dir() -> Result<PathBuf> {
    Ok(home_dir()?.join("destiny"))
}
