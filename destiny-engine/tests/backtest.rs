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
                .long_limit_open(&self.symbol, dec!(1), dec!(200))
                .await?;
            *self.is_buy.lock() = true;
        }
        let time = engine.time().str_ymd_hm();
        let price_mark = engine.price_mark(&self.symbol);
        let cash_available = engine.cash_available();
        let margin = engine.margin();
        let long_size = engine.long_size(&self.symbol);
        info!("{time} 标记价({price_mark:.2}),可用资金({cash_available:.4}),保证金({margin:.4}),多仓({long_size:.4})");
        Ok(())
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_backtest() -> Result<()> {
    if bool::has_github_action() {
        return Ok(());
    }

    Backtest::run(
        BacktestConfigBuilder::default()
            .begin("2020".to_date()?)
            .end("2024".to_date()?)
            .log_show_std(true)
            .log_targets(vec!["backtest".to_string()])
            .build()?,
        Arc::new(BacktestStrategy::new()),
    )
    .await?;

    Ok(())
}
