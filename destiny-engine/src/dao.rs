use anyhow::Result;
use chrono::{DateTime, Utc};
use destiny_helpers::prelude::*;
use destiny_types::models::MarketFileMeta;
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
    pub async fn market_file_meta_init(&self) -> Result<()> {
        let mut tx = self.0.begin().await?;
        sqlx::query(
            "
        create table if not exists market_file_meta (
            id integer not null primary key autoincrement,
            symbol text not null,
            day date not null,
            hour integer not null,
            path text not null,
            update_time datetime not null,
            local_time datetime
        )
        ",
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            "create unique index if not exists idx_market_file_meta on market_file_meta(symbol, day, hour)",
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn market_file_meta_sync(
        &self,
        symbol: &str,
        day: DateTime<Utc>,
        hour: i32,
        path: &str,
        update_time: DateTime<Utc>,
    ) -> Result<()> {
        sqlx::query(
            "
        insert into market_file_meta 
            (symbol, day, hour, path, update_time) 
        values 
            (?, date(?), ?, ?, ?)
        on conflict 
            (symbol, day, hour)
        do update set 
            update_time = excluded.update_time
        ",
        )
        .bind(symbol)
        .bind(day)
        .bind(hour)
        .bind(path)
        .bind(update_time)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn market_file_meta_get_by_symbol(
        &self,
        symbol: &str,
    ) -> Result<Vec<MarketFileMeta>> {
        let result = sqlx::query_as("select * from market_file_meta where symbol = ?")
            .bind(symbol)
            .fetch_all(&self.0)
            .await?;

        Ok(result)
    }

    pub async fn market_file_meta_update_local_time(
        &self,
        id: i64,
        local_time: DateTime<Utc>,
    ) -> Result<()> {
        sqlx::query("update market_file_meta set local_time = ? where id = ?")
            .bind(local_time)
            .bind(id)
            .execute(&self.0)
            .await?;

        Ok(())
    }
}
