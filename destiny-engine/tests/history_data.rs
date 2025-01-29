use anyhow::Result;
use destiny_engine::history_data;

#[tokio::test]
async fn test_sync_file_list() -> Result<()> {
    if std::env::var("GITHUB_ACTIONS").is_ok() {
        return Ok(());
    }
    history_data::sync_file_list().await?;
    Ok(())
}
