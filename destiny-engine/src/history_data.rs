use anyhow::Result;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion, Region};
use destiny_helpers::prelude::*;
use sqlx::SqliteConnection;

pub async fn sync_file_list() -> Result<()> {
    let save_dir = cache_dir()?.join("market_data");
    std::fs::create_dir_all(&save_dir)?;
    let mut db = open_db(&save_dir.join("meta.db")).await?;
    create_table_file_meta_new(&mut db).await?;
    init_table_file_meta_new(&mut db).await?;

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(RegionProviderChain::default_provider().or_else(Region::new("us-east-1")))
        .no_credentials()
        .load()
        .await;
    let client = aws_sdk_s3::Client::new(&config);

    let mut continuation_token: Option<String> = None;

    loop {
        let response = client
            .list_objects_v2()
            .bucket("hyperliquid-archive")
            .prefix("market_data/")
            .set_continuation_token(continuation_token)
            .send()
            .await?;

        for content in response.contents() {
            let key = content.key().unwrap();
            let split_key = key.split("/").collect::<Vec<&str>>();
            let day = split_key[1];
            let day = format!(
                "{}-{}-{}",
                day[0..4].to_owned(),
                day[4..6].to_owned(),
                day[6..8].to_owned()
            );
            let hour = split_key[2].parse::<i32>()?;
            let symbol = split_key[4].split(".").next().unwrap();
            save_file_meta_new(&mut db, &day, hour, symbol, key).await?;
        }
        continuation_token = response.next_continuation_token().map(|s| s.to_string());
        if continuation_token.is_none() {
            break;
        }
    }

    Ok(())
}

async fn create_table_file_meta_new(db: &mut SqliteConnection) -> Result<()> {
    sqlx::query(
        "
        create table if not exists file_meta_new (
            id integer primary key autoincrement, 
            day date,
            hour integer, 
            symbol text,
            path text
        )
        ",
    )
    .execute(db)
    .await?;
    Ok(())
}

async fn init_table_file_meta_new(db: &mut SqliteConnection) -> Result<()> {
    sqlx::query("delete from file_meta_new").execute(db).await?;
    Ok(())
}

async fn save_file_meta_new(
    db: &mut SqliteConnection,
    day: &str,
    hour: i32,
    symbol: &str,
    path: &str,
) -> Result<()> {
    sqlx::query("insert into file_meta_new (day, hour, symbol, path) values (?, ?, ?, ?)")
        .bind(day)
        .bind(hour)
        .bind(symbol)
        .bind(path)
        .execute(db)
        .await?;
    Ok(())
}
