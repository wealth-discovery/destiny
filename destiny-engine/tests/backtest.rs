use destiny_engine::prelude::*;

struct BacktestStrategy {
    symbol: String,
    is_buy: Arc<Mutex<bool>>,
}

impl BacktestStrategy {
    pub fn new() -> Self {
        Self {
            symbol: "ETHUSDT".to_string(),
            is_buy: Arc::new(Mutex::new(false)),
        }
    }
}

#[async_trait]
#[allow(unused_variables)]
impl Strategy for BacktestStrategy {
    async fn on_init(&self, engine: Arc<dyn Engine>) -> Result<()> {
        tracing::info!("{} on_init", engine.time());
        engine.symbol_init(&self.symbol)?;
        Ok(())
    }

    async fn on_minutely(&self, engine: Arc<dyn Engine>) -> Result<()> {
        if !*self.is_buy.lock() {
            engine
                .long_limit_open(&self.symbol, Decimal::new(1.0), Decimal::new(200.0))
                .await?;
            *self.is_buy.lock() = true;
        }
        let time = engine.time().str_ymd_hm();
        let price_mark = engine.price_mark(&self.symbol);
        let cash_available = engine.cash_available();
        info!("{time} 标记价({price_mark:.2}),可用资金({cash_available:.2})");
        Ok(())
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_backtest() -> Result<()> {
    if bool::has_github_action() {
        return Ok(());
    }

    let log_collector = LogConfigBuilder::default()
        .level(LogLevel::INFO)
        .save_file(false)
        .targets(vec!["backtest".to_string()])
        .build()?
        .init_log()
        .await?;

    let config = BacktestConfigBuilder::default()
        .begin("2020".to_date()?)
        .end("2024".to_date()?)
        .build()?;

    Backtest::run(config, Arc::new(BacktestStrategy::new())).await?;

    log_collector.done().await?;

    Ok(())
}
