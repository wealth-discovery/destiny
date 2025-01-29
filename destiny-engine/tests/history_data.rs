use anyhow::Result;
use destiny_engine::prelude::*;
use destiny_helpers::prelude::*;
use destiny_types::prelude::*;

#[tokio::test]
async fn test_sync_history_data() -> Result<()> {
    init_log(LogConfigBuilder::default().build()?).await?;
    sync_history_data(SyncHistoryMeta::klines(
        "BTCUSDT",
        KlineInterval::M1,
        2020,
        1,
    ))
    .await?;
    Ok(())
}
