use anyhow::Result;
use destiny_engine::prelude::*;
use destiny_helpers::prelude::*;

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

    sync_symbol_history_data("ETHUSDT", str_to_date("202001")?, str_to_date("202412")?).await?;
    sync_symbol_history_data("BTCUSDT", str_to_date("202001")?, str_to_date("202412")?).await?;
    sync_symbol_history_data("SOLUSDT", str_to_date("202001")?, str_to_date("202412")?).await?;
    sync_symbol_history_data("DOGEUSDT", str_to_date("202001")?, str_to_date("202412")?).await?;

    Ok(())
}
