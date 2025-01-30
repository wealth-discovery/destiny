use anyhow::Result;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};
use std::{path::Path, str::FromStr};

/// 打开数据库
/// <br> `path` 表示完整的文件路径, 例如: `/home/husky/data/db.sqlite`.
/// <br> 如果文件不存在, 则创建文件.
pub async fn open_db(path: &Path) -> Result<Pool<Sqlite>> {
    let options =
        SqliteConnectOptions::from_str(format!("sqlite://{}", path.to_string_lossy()).as_str())?
            .create_if_missing(true);

    Ok(SqlitePoolOptions::new()
        .max_connections(1)
        .min_connections(1)
        .connect_with(options)
        .await?)
}
