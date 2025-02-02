use anyhow::Result;
use destiny_helpers::prelude::*;

#[tokio::test]
async fn test_log() -> Result<()> {
    let log_collector = LogConfigBuilder::default()
        .save_file(false)
        .targets(vec!["log".to_string()])
        .build()?
        .init_log()?;

    tracing::trace!("trace");
    tracing::debug!("debug");
    tracing::info!("info");
    tracing::warn!("warn");
    tracing::error!("error");

    log_collector.done();

    Ok(())
}
