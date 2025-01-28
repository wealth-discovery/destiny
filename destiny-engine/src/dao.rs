use anyhow::Result;
use chrono::{DateTime, Utc};
use destiny_helpers::prelude::*;
use sqlx::{Pool, Sqlite};
use std::path::PathBuf;

pub struct Dao(Pool<Sqlite>);

impl Dao {
    pub async fn new(path: &PathBuf, file_name: &str) -> Result<Self> {
        let dir = cache_dir()?.join(path);
        std::fs::create_dir_all(&dir)?;
        let db = open_db(&dir.join(file_name)).await?;
        Ok(Self(db))
    }
}

impl Dao {
    pub async fn file_meta_init(&self) -> Result<()> {
        let mut tx = self.0.begin().await?;
        sqlx::query(
            "
        create table if not exists file_meta (
            id integer primary key autoincrement, 
            day date not null,
            hour integer not null, 
            symbol text not null,
            path text not null,
            update_time datetime not null,
            local_time datetime
        )
        ",
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            "create unique index if not exists idx_file_meta on file_meta(symbol, day, hour)",
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn file_meta_sync(
        &self,
        day: DateTime<Utc>,
        hour: i32,
        symbol: &str,
        path: &str,
        update_time: DateTime<Utc>,
    ) -> Result<()> {
        let mut tx = self.0.begin().await?;

        sqlx::query(
            "
        insert into file_meta 
            (day, hour, symbol, path, update_time) 
        values 
            (?, ?, ?, ?, ?)
        on conflict 
            (symbol, day, hour)
        do update set 
            update_time = excluded.update_time
        ",
        )
        .bind(day)
        .bind(hour)
        .bind(symbol)
        .bind(path)
        .bind(update_time)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }
}
