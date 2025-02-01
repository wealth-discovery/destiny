use destiny_engine::prelude::*;

struct BacktestStrategy;

#[async_trait]
#[allow(unused_variables)]
impl Strategy for BacktestStrategy {
    async fn on_init(&self, engine: Arc<dyn Engine>) -> Result<()> {
        tracing::info!("on_init: {}", engine.now());
        // engine.init_symbol("TRUMPUSDT")?;
        engine.init_symbol("ETHUSDT")?;
        // engine.init_symbol("BTCUSDT")?;
        // engine.init_symbol("SOLUSDT")?;
        Ok(())
    }

    async fn on_tick(&self, engine: Arc<dyn Engine>) -> Result<()> {
        tracing::info!("on_tick: {}", engine.symbol_market("ETHUSDT")?.last_price);
        Ok(())
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_backtest() -> Result<()> {
    if bool::has_github_action() {
        return Ok(());
    }

    let log_collector = LogConfigBuilder::default()
        .save_file(false)
        .targets(vec!["backtest".to_string()])
        .build()?
        .init_log()
        .await?;

    let config = BacktestConfigBuilder::default()
        .begin("2023".to_date()?)
        .end("2024".to_date()?)
        .build()?;

    Backtest::run(config, Arc::new(BacktestStrategy)).await?;

    log_collector.done().await?;

    Ok(())
}
