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
        println!("on_init: {}", engine.now_ms());
        Ok(())
    }

    async fn on_start(&self, engine: Arc<dyn Engine>) -> Result<()> {
        println!("on_start: {}", engine.now_ms());
        Ok(())
    }

    async fn on_stop(&self, engine: Arc<dyn Engine>) -> Result<()> {
        println!("on_stop: {}", engine.now_ms());
        Ok(())
    }
}

#[tokio::test]
async fn test_backtest() -> Result<()> {
    let config = BacktestConfigBuilder::default()
        .begin(str_to_date("2024-01-01T00:00:00Z")?)
        .end(str_to_date("2024-01-02T00:00:00Z")?)
        .build()?;

    run_backtest(config, Arc::new(BacktestStrategy)).await?;
    Ok(())
}
