use anyhow::Result;
use destiny_helpers::prelude::*;
use sqlx::{Pool, Sqlite};
use std::path::PathBuf;
use tokio::fs::create_dir_all;

#[allow(dead_code)]
pub struct Dao(Pool<Sqlite>);

impl Dao {
    pub async fn new(path: &PathBuf, file_name: &str) -> Result<Self> {
        let dir = cache_dir()?.join(path);
        create_dir_all(&dir).await?;

        let db = open_db(&dir.join(file_name)).await?;
        Ok(Self(db))
    }
}
