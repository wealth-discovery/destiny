use anyhow::Result;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};
use std::{path::Path, str::FromStr};

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
