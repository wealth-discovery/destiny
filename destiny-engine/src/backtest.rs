use crate::traits::*;
use anyhow::{ensure, Result};
use async_trait::async_trait;
use chrono::{DateTime, Duration, DurationRound, Utc};
use derive_builder::Builder;
use destiny_helpers::prelude::*;
use std::sync::Arc;

/// 回测配置
#[derive(Builder)]
#[builder(setter(into))]
pub struct BacktestConfig {
    /// 开始时间
    pub begin: DateTime<Utc>,
    /// 结束时间
    pub end: DateTime<Utc>,
    /// 初始资金
    #[builder(default = 1000.)]
    pub cash: f64,
    /// 吃单手续费率
    #[builder(default = 0.0005)]
    pub fee_rate_taker: f64,
    /// 挂单手续费率
    #[builder(default = 0.0005)]
    pub fee_rate_maker: f64,
    /// 滑点
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

    config.cash = truncate_float(config.cash, 8, false);
    ensure!(config.cash >= 0.0, "cash must be greater than 0");

    config.fee_rate_taker = truncate_float(config.fee_rate_taker, 8, false);
    config.fee_rate_maker = truncate_float(config.fee_rate_maker, 8, false);

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
