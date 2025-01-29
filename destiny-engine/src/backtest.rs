use crate::traits::*;
use anyhow::{ensure, Result};
use async_trait::async_trait;
use chrono::{DateTime, Duration, DurationRound, Utc};
use derive_builder::Builder;
use destiny_helpers::num::{is_zero, truncate_float};
use std::sync::Arc;

#[derive(Builder)]
#[builder(setter(into))]
pub struct BacktestConfig {
    pub begin: DateTime<Utc>,
    pub end: DateTime<Utc>,
    #[builder(default = 1000.)]
    pub spot_balance: f64,
    #[builder(default = 1000.)]
    pub contract_balance: f64,
    #[builder(default = 0.0005)]
    pub taker_fee_rate: f64,
    #[builder(default = 0.0005)]
    pub maker_fee_rate: f64,
    #[builder(default = 0.01)]
    pub slippage_rate: f64,
}

#[allow(dead_code)]
pub struct Backtest {
    config: Arc<BacktestConfig>,
}

#[async_trait]
impl Engine for Backtest {}

fn new(mut config: BacktestConfig) -> Result<Arc<Backtest>> {
    config.begin = config.begin.duration_trunc(Duration::minutes(1))?;
    config.end = config.end.duration_trunc(Duration::minutes(1))?;
    ensure!(
        config.begin < config.end,
        "begin time must be less than end time"
    );

    config.spot_balance = truncate_float(config.spot_balance, 8, false);
    ensure!(
        config.spot_balance >= 0.0,
        "spot balance must be greater than 0"
    );

    config.contract_balance = truncate_float(config.contract_balance, 8, false);
    ensure!(
        config.contract_balance >= 0.0,
        "contract balance must be greater than 0"
    );

    ensure!(
        !(is_zero(config.spot_balance) && is_zero(config.contract_balance)),
        "spot balance and contract balance must not be both 0"
    );

    config.taker_fee_rate = truncate_float(config.taker_fee_rate, 8, false);
    config.maker_fee_rate = truncate_float(config.maker_fee_rate, 8, false);

    config.slippage_rate = truncate_float(config.slippage_rate, 8, false);
    ensure!(
        config.slippage_rate >= 0.0,
        "slippage rate must be greater than 0"
    );

    Ok(Arc::new(Backtest {
        config: Arc::new(config),
    }))
}

pub async fn run(config: BacktestConfig, strategy: Arc<dyn Strategy>) -> Result<()> {
    let backtest = new(config)?;
    strategy.on_init(backtest.clone()).await?;
    strategy.on_start(backtest.clone()).await?;
    strategy.on_stop(backtest.clone()).await?;
    Ok(())
}
