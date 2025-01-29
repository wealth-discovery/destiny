use crate::traits::*;
use anyhow::{ensure, Result};
use async_trait::async_trait;
use chrono::{DateTime, Duration, DurationRound, Utc};
use derive_builder::Builder;
use destiny_helpers::prelude::*;
use destiny_types::prelude::*;
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

impl Engine for Backtest {}

impl EngineBasic for Backtest {}

impl EngineInit for Backtest {
    /// 初始化交易对
    fn init_symbol(&self, symbol: &str) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
impl EngineTrade for Backtest {
    /// 市价开多
    async fn open_long_market(&self, symbol: &str, size: f64) -> Result<String> {
        todo!()
    }
    /// 限价开多
    async fn open_long_limit(&self, symbol: &str, size: f64, price: f64) -> Result<String> {
        todo!()
    }
    /// 市价平多
    async fn close_long_market(&self, symbol: &str, size: f64) -> Result<String> {
        todo!()
    }
    /// 限价平多
    async fn close_long_limit(&self, symbol: &str, size: f64, price: f64) -> Result<String> {
        todo!()
    }
    /// 市价开空
    async fn open_short_market(&self, symbol: &str, size: f64) -> Result<String> {
        todo!()
    }
    /// 限价开空
    async fn open_short_limit(&self, symbol: &str, size: f64, price: f64) -> Result<String> {
        todo!()
    }
    /// 市价平空
    async fn close_short_market(&self, symbol: &str, size: f64) -> Result<String> {
        todo!()
    }
    /// 限价平空
    async fn close_short_limit(&self, symbol: &str, size: f64, price: f64) -> Result<String> {
        todo!()
    }
    /// 撤单
    async fn cancel_order(&self, symbol: &str, order_id: &str) -> Result<()> {
        todo!()
    }
    /// 批量撤单
    async fn cancel_orders(&self, symbol: &str, order_ids: &[&str]) -> Result<()> {
        todo!()
    }
    /// 设置杠杆倍率
    async fn set_leverage(&self, symbol: &str, leverage: u32) -> Result<()> {
        todo!()
    }
    /// 获取杠杆倍率
    fn leverage(&self, symbol: &str) -> Result<u32> {
        todo!()
    }
    /// 获取订单
    fn order(&self, symbol: &str, order_id: &str) -> Result<Option<Order>> {
        todo!()
    }
    /// 获取交易对订单
    fn orders(&self, symbol: &str) -> Result<Vec<Order>> {
        todo!()
    }
    /// 获取多头订单
    fn orders_long(&self, symbol: &str) -> Result<Vec<Order>> {
        todo!()
    }
    /// 获取多头开仓订单
    fn orders_open_long(&self, symbol: &str) -> Result<Vec<Order>> {
        todo!()
    }
    /// 获取多头平仓订单
    fn orders_close_long(&self, symbol: &str) -> Result<Vec<Order>> {
        todo!()
    }
    /// 获取空头订单
    fn orders_short(&self, symbol: &str) -> Result<Vec<Order>> {
        todo!()
    }
    /// 获取空头开仓订单
    fn orders_open_short(&self, symbol: &str) -> Result<Vec<Order>> {
        todo!()
    }
    /// 获取空头平仓订单
    fn orders_close_short(&self, symbol: &str) -> Result<Vec<Order>> {
        todo!()
    }
}

impl EngineAccount for Backtest {
    /// 获取保证金
    fn cash(&self) -> Cash {
        todo!()
    }
    /// 获取持仓
    fn position(&self, symbol: &str) -> Result<SymbolPosition> {
        todo!()
    }
}

impl EngineMarket for Backtest {
    /// 获取交易对
    fn symbol(&self, symbol: &str) -> Result<Symbol> {
        todo!()
    }
    /// 获取交易规则
    fn symbol_rule(&self, symbol: &str) -> Result<SymbolRule> {
        todo!()
    }
    /// 获取指数
    fn symbol_index(&self, symbol: &str) -> Result<SymbolIndex> {
        todo!()
    }
}

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
