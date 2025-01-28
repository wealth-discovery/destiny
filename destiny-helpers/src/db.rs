use anyhow::Result;
use sqlx::{sqlite::SqliteConnectOptions, Connection, SqliteConnection};
use std::{path::PathBuf, str::FromStr};

pub async fn open_db(path: &PathBuf) -> Result<SqliteConnection> {
    let options =
        SqliteConnectOptions::from_str(format!("sqlite://{}", path.to_string_lossy()).as_str())?
            .create_if_missing(true);
    let conn = SqliteConnection::connect_with(&options).await?;
    Ok(conn)
}
