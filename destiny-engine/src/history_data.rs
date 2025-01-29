use crate::dao::Dao;
use anyhow::Result;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion, Region};
use destiny_helpers::prelude::*;
use tracing::instrument;

#[instrument(name = "SyncFileList", skip_all)]
pub async fn sync_file_list() -> Result<()> {
    let dao = Dao::new(&cache_dir()?.join("market_data"), "meta.db").await?;
    dao.market_file_meta_init().await?;

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
            let path = content.key().expect("key is none");
            let split_key = path.split("/").collect::<Vec<&str>>();
            let day = str_to_date(split_key[1])?;
            let hour = split_key[2].parse::<i32>()?;
            let symbol = split_key[4].split(".").next().expect("symbol is none");
            let update_time = ms_to_date(
                content
                    .last_modified
                    .expect("last_modified is none")
                    .to_millis()?,
            )?;
            dao.market_file_meta_sync(symbol, day, hour, path, update_time)
                .await?;
            tracing::info!(
                "symbol({}), day({}), hour({}), update_time({}), path({})",
                symbol,
                day.format("%Y-%m-%d"),
                hour,
                update_time.format("%Y-%m-%d %H:%M:%S"),
                path
            );
        }
        continuation_token = response.next_continuation_token().map(|s| s.to_string());
        if continuation_token.is_none() {
            break;
        }
    }

    Ok(())
}

#[instrument(name = "DownloadFiles", skip_all)]
pub async fn download_files(symbol: &str) -> Result<()> {
    let dao = Dao::new(&cache_dir()?.join("market_data"), "meta.db").await?;
    dao.market_file_meta_init().await?;

    let file_metas = dao.market_file_meta_get_by_symbol(symbol).await?;
    for file_meta in file_metas {
        tracing::info!("{:?}", file_meta);
    }

    Ok(())
}
