use anyhow::Result;
use destiny_engine::history_data;
use destiny_helpers::prelude::*;

#[tokio::test]
async fn test_sync_file_list() -> Result<()> {
    if std::env::var("GITHUB_ACTIONS").is_ok() {
        return Ok(());
    }

    init_log(LogConfigBuilder::default().build()?).await?;
    history_data::sync_file_list().await?;
    Ok(())
}

#[tokio::test]
async fn test_download_files() -> Result<()> {
    if std::env::var("GITHUB_ACTIONS").is_ok() {
        return Ok(());
    }
    init_log(LogConfigBuilder::default().build()?).await?;
    history_data::download_files("BTC").await?;
    Ok(())
}
