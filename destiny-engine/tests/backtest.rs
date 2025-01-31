use anyhow::Result;
use async_trait::async_trait;
use destiny_engine::prelude::*;
use destiny_helpers::prelude::*;
use std::sync::Arc;

struct BacktestStrategy;

#[async_trait]
#[allow(unused_variables)]
impl Strategy for BacktestStrategy {
    async fn on_init(&self, engine: Arc<dyn Engine>) -> Result<()> {
        tracing::info!("on_init: {}", engine.now());
        // engine.init_symbol("TRUMPUSDT")?;
        engine.init_symbol("ETHUSDT")?;
        engine.init_symbol("BTCUSDT")?;
        engine.init_symbol("SOLUSDT")?;
        Ok(())
    }

    async fn on_start(&self, engine: Arc<dyn Engine>) -> Result<()> {
        tracing::info!("on_start: {}", engine.now());
        Ok(())
    }

    async fn on_stop(&self, engine: Arc<dyn Engine>) -> Result<()> {
        tracing::info!("on_stop: {}", engine.now());
        Ok(())
    }
}

#[tokio::test]
async fn test_backtest() -> Result<()> {
    if bool::has_github_action() {
        return Ok(());
    }

    LogConfigBuilder::default()
        .save_file(false)
        .targets(vec!["backtest".to_string()])
        .build()?
        .init_log()
        .await?;

    let config = BacktestConfigBuilder::default()
        .begin("20240101".to_date()?)
        .end("20240102".to_date()?)
        .build()?;

    run_backtest(config, Arc::new(BacktestStrategy)).await?;
    Ok(())
}
