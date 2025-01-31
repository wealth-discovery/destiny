use anyhow::Result;
use destiny_engine::prelude::*;
use destiny_helpers::prelude::*;

#[tokio::test]
async fn test_sync_history_data() -> Result<()> {
    if bool::has_github_action() {
        return Ok(());
    }
    LogConfigBuilder::default()
        .save_file(false)
        .targets(vec!["history_data".to_string()])
        .build()?
        .init_log()
        .await?;

    sync_symbol_history_data("ETHUSDT", "202001".to_date()?, "202412".to_date()?).await?;
    sync_symbol_history_data("BTCUSDT", "202001".to_date()?, "202412".to_date()?).await?;
    sync_symbol_history_data("SOLUSDT", "202001".to_date()?, "202412".to_date()?).await?;
    sync_symbol_history_data("DOGEUSDT", "202001".to_date()?, "202412".to_date()?).await?;

    Ok(())
}
