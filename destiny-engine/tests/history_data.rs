use anyhow::Result;
use destiny_engine::prelude::*;
use destiny_helpers::prelude::*;
use destiny_types::prelude::*;
use strum::IntoEnumIterator;
use tracing::instrument;

#[tokio::test]
async fn test_sync_history_data() -> Result<()> {
    if has_github_action_env() {
        return Ok(());
    }
    init_log(
        LogConfigBuilder::default()
            .save_file(false)
            .targets(vec!["history_data".to_string()])
            .build()?,
    )
    .await?;

    download_agg_trades().await?;
    download_book_ticker().await?;
    download_funding_rate().await?;
    download_trades().await?;

    for interval in KlineInterval::iter() {
        download_index_price_klines(interval).await?;
        download_kline(interval).await?;
        download_mark_price_klines(interval).await?;
        download_premium_index_klines(interval).await?;
    }

    Ok(())
}

#[instrument(name = "DownloadAggTrades")]
async fn download_agg_trades() -> Result<()> {
    sync_history_data(SyncHistoryMeta::agg_trades("BTCUSDT", 2020, 1)).await;
    Ok(())
}

#[instrument(name = "DownloadBookTicker")]
async fn download_book_ticker() -> Result<()> {
    sync_history_data(SyncHistoryMeta::book_ticker("BTCUSDT", 2020, 1)).await;
    Ok(())
}

#[instrument(name = "DownloadFundingRate")]
async fn download_funding_rate() -> Result<()> {
    sync_history_data(SyncHistoryMeta::funding_rate("BTCUSDT", 2020, 1)).await;
    Ok(())
}

#[instrument(name = "DownloadIndexPriceKlines")]
async fn download_index_price_klines(interval: KlineInterval) -> Result<()> {
    sync_history_data(SyncHistoryMeta::index_price_klines(
        "BTCUSDT", interval, 2020, 1,
    ))
    .await;
    Ok(())
}

#[instrument(name = "DownloadKline")]
async fn download_kline(interval: KlineInterval) -> Result<()> {
    sync_history_data(SyncHistoryMeta::klines("BTCUSDT", interval, 2020, 1)).await;
    Ok(())
}

#[instrument(name = "DownloadMarkPriceKlines")]
async fn download_mark_price_klines(interval: KlineInterval) -> Result<()> {
    sync_history_data(SyncHistoryMeta::mark_price_klines(
        "BTCUSDT", interval, 2020, 1,
    ))
    .await;
    Ok(())
}

#[instrument(name = "DownloadPremiumIndexKlines")]
async fn download_premium_index_klines(interval: KlineInterval) -> Result<()> {
    sync_history_data(SyncHistoryMeta::premium_index_klines(
        "BTCUSDT", interval, 2020, 1,
    ))
    .await;
    Ok(())
}

#[instrument(name = "DownloadTrades")]
async fn download_trades() -> Result<()> {
    sync_history_data(SyncHistoryMeta::trades("BTCUSDT", 2020, 1)).await;
    Ok(())
}
