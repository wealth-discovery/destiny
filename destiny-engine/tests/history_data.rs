use destiny_engine::prelude::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_sync_history_data() -> Result<()> {
    if bool::has_github_action() {
        return Ok(());
    }
    let log_collector = LogConfigBuilder::default()
        .save_file(false)
        .targets(vec!["history_data".to_string()])
        .build()?
        .init_log()
        .await?;

    let _ = tokio::join!(
        tokio::spawn(async move {
            SyncHistoryData::sync_symbol("ETHUSDT", "202001".to_date()?, "202412".to_date()?).await
        }),
        tokio::spawn(async move {
            SyncHistoryData::sync_symbol("BTCUSDT", "202001".to_date()?, "202412".to_date()?).await
        }),
        tokio::spawn(async move {
            SyncHistoryData::sync_symbol("SOLUSDT", "202001".to_date()?, "202412".to_date()?).await
        }),
        tokio::spawn(async move {
            SyncHistoryData::sync_symbol("DOGEUSDT", "202001".to_date()?, "202412".to_date()?).await
        }),
    );

    log_collector.done().await?;

    Ok(())
}
