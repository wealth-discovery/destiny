use crate::dao::Dao;
use anyhow::Result;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion, Region};
use destiny_helpers::prelude::*;

pub async fn sync_file_list() -> Result<()> {
    tracing::info!("sync file list");
    let dao = Dao::new(&cache_dir()?.join("market_data"), "meta.db").await?;
    dao.file_meta_init().await?;

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
            let path = content.key().unwrap();
            let split_key = path.split("/").collect::<Vec<&str>>();
            let day = str_to_date(split_key[1])?;
            let hour = split_key[2].parse::<i32>()?;
            let symbol = split_key[4].split(".").next().unwrap();
            let update_time = ms_to_date(content.last_modified.unwrap().to_millis()?)?;
            dao.file_meta_sync(symbol, day, hour, path, update_time)
                .await?;
            tracing::info!("sync file meta: {:?}", path);
        }
        continuation_token = response.next_continuation_token().map(|s| s.to_string());
        if continuation_token.is_none() {
            break;
        }
    }

    Ok(())
}
