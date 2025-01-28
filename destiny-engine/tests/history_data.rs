use anyhow::Result;
use destiny_engine::history_data;

#[tokio::test]
async fn test_backtest() -> Result<()> {
    history_data::sync_file_list().await?;
    Ok(())
}
