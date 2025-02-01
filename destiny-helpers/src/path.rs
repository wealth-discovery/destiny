use anyhow::{anyhow, Result};
use homedir::my_home;
use std::path::PathBuf;

pub trait PathBufSupport {
    /// 获取用户主目录
    fn home() -> Result<PathBuf>;
    /// 获取缓存目录
    fn cache() -> Result<PathBuf>;
}

impl PathBufSupport for PathBuf {
    fn home() -> Result<PathBuf> {
        my_home()?.ok_or(anyhow!("用户主目录未找到"))
    }

    fn cache() -> Result<PathBuf> {
        Ok(Self::home()?.join("destiny"))
    }
}
