use anyhow::Result;
use destiny_helpers::prelude::*;

#[tokio::test]
async fn test_log() -> Result<()> {
    init_log(
        LogConfigBuilder::default()
            .save_file(false)
            .targets(vec!["log".to_string()])
            .build()?,
    )
    .await?;

    tracing::trace!("trace");
    tracing::debug!("debug");
    tracing::info!("info");
    tracing::warn!("warn");
    tracing::error!("error");

    Ok(())
}
