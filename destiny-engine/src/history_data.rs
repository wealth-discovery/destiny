use crate::dao::Dao;
use anyhow::Result;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion, Region};
use destiny_helpers::prelude::*;
use std::{fs::File, io::Write};
use tokio::fs::{create_dir_all, remove_file};
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
    let market_data_dir = cache_dir()?.join("market_data");
    let dao = Dao::new(&market_data_dir, "meta.db").await?;
    dao.market_file_meta_init().await?;

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(RegionProviderChain::default_provider().or_else(Region::new("us-east-1")))
        .no_credentials()
        .load()
        .await;
    let client = aws_sdk_s3::Client::new(&config);

    let file_metas = dao.market_file_meta_get_unsync_by_symbol(symbol).await?;
    for file_meta in file_metas {
        let response = client
            .get_object()
            .bucket("hyperliquid-archive")
            .key(file_meta.path)
            .send()
            .await?;
        let body = response.body.collect().await?;

        let symbol_dir = market_data_dir.join(&file_meta.symbol);
        create_dir_all(&symbol_dir).await?;
        let lz4_file_name = format!(
            "{}-{:02}.lz4",
            file_meta.day.format("%Y%m%d"),
            file_meta.hour
        );
        let lz4_save_file = symbol_dir.join(&lz4_file_name);
        if lz4_save_file.exists() {
            remove_file(&lz4_save_file).await?;
        }
        let mut lz4_file = File::create(&lz4_save_file)?;
        lz4_file.write_all(&body.into_bytes())?;
        drop(lz4_file);

        let lz4_file = File::open(&lz4_save_file)?;
        let mut lz4_decode = lz4::Decoder::new(lz4_file)?;

        let csv_file_name = format!(
            "{}-{:02}.csv",
            file_meta.day.format("%Y%m%d"),
            file_meta.hour
        );
        let csv_save_file = symbol_dir.join(&csv_file_name);
        if csv_save_file.exists() {
            remove_file(&csv_save_file).await?;
        }
        let mut csv_file = File::create(&csv_save_file)?;
        std::io::copy(&mut lz4_decode, &mut csv_file)?;

        dao.market_file_meta_update_local_time(file_meta.id, file_meta.update_time)
            .await?;

        tracing::info!("symbol({}), file({})", file_meta.symbol, lz4_file_name);
    }

    Ok(())
}
