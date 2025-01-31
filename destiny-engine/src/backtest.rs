use crate::traits::*;
use anyhow::{anyhow, ensure, Result};
use async_trait::async_trait;
use chrono::{DateTime, Duration, DurationRound, Utc};
use derive_builder::Builder;
use destiny_helpers::prelude::*;
use destiny_types::prelude::*;
use parking_lot::Mutex;
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
    account: Arc<Mutex<Account>>,
    trade_time: Arc<Mutex<DateTime<Utc>>>,
}

impl Engine for Backtest {}

impl EngineInit for Backtest {
    fn init_symbol(&self, symbol: &str) -> Result<()> {
        ensure!(
            !self.account.lock().symbols.contains_key(symbol),
            "重复初始化交易对: {}",
            symbol
        );
        self.account.lock().symbols.insert(
            symbol.to_string(),
            Symbol {
                symbol: symbol.to_string(),
                rule: SymbolRule {
                    enable: true,
                    price_min: 1e-8,
                    price_max: 1e8,
                    price_tick: 1e-8,
                    size_min: 1e-8,
                    size_max: 1e8,
                    size_tick: 1e-8,
                    cash_min: 1e-8,
                    order_max: 200,
                },
                index: SymbolIndex {
                    mark_price: 0.,
                    index_price: 0.,
                    last_price: 0.,
                    settlement_price: 0.,
                    next_settlement_time: Default::default(),
                    time: Default::default(),
                },
            },
        );
        Ok(())
    }
}

#[async_trait]
#[allow(unused_variables)]
impl EngineTrade for Backtest {
    fn now(&self) -> DateTime<Utc> {
        *self.trade_time.lock()
    }
    async fn open_long_market(&self, symbol: &str, size: f64) -> Result<String> {
        let symbol_index = self.symbol_index(symbol)?;
        let symbol_rule = self.symbol_rule(symbol)?;
        let position = self.position(symbol)?;
        let size = (symbol_rule.size_tick % size).to_safe();
        ensure!(
            size >= symbol_rule.size_min,
            "数量小于: {}",
            symbol_rule.size_min
        );
        let cash = (size * symbol_index.last_price).to_safe();
        ensure!(
            cash >= symbol_rule.cash_min,
            "金额小于: {}",
            symbol_rule.cash_min
        );
        let margin = (cash / position.leverage as f64).to_safe();

        todo!()
    }
    async fn open_long_limit(&self, symbol: &str, size: f64, price: f64) -> Result<String> {
        todo!()
    }
    async fn close_long_market(&self, symbol: &str, size: f64) -> Result<String> {
        todo!()
    }
    async fn close_long_limit(&self, symbol: &str, size: f64, price: f64) -> Result<String> {
        todo!()
    }
    async fn open_short_market(&self, symbol: &str, size: f64) -> Result<String> {
        todo!()
    }
    async fn open_short_limit(&self, symbol: &str, size: f64, price: f64) -> Result<String> {
        todo!()
    }
    async fn close_short_market(&self, symbol: &str, size: f64) -> Result<String> {
        todo!()
    }
    async fn close_short_limit(&self, symbol: &str, size: f64, price: f64) -> Result<String> {
        todo!()
    }
    async fn cancel_order(&self, symbol: &str, order_id: &str) -> Result<()> {
        todo!()
    }
    async fn cancel_orders(&self, symbol: &str, order_ids: &[&str]) -> Result<()> {
        todo!()
    }
    async fn set_leverage(&self, symbol: &str, leverage: u32) -> Result<()> {
        todo!()
    }
    fn leverage(&self, symbol: &str) -> Result<u32> {
        todo!()
    }
    fn order(&self, symbol: &str, order_id: &str) -> Result<Option<Order>> {
        todo!()
    }
    fn orders(&self, symbol: &str) -> Result<Vec<Order>> {
        todo!()
    }
    fn orders_long(&self, symbol: &str) -> Result<Vec<Order>> {
        todo!()
    }
    fn orders_open_long(&self, symbol: &str) -> Result<Vec<Order>> {
        todo!()
    }
    fn orders_close_long(&self, symbol: &str) -> Result<Vec<Order>> {
        todo!()
    }
    fn orders_short(&self, symbol: &str) -> Result<Vec<Order>> {
        todo!()
    }
    fn orders_open_short(&self, symbol: &str) -> Result<Vec<Order>> {
        todo!()
    }
    fn orders_close_short(&self, symbol: &str) -> Result<Vec<Order>> {
        todo!()
    }
}

impl EngineAccount for Backtest {
    fn cash(&self) -> Cash {
        self.account.lock().cash.clone()
    }
    fn position(&self, symbol: &str) -> Result<SymbolPosition> {
        self.account
            .lock()
            .positions
            .get(symbol)
            .cloned()
            .ok_or(anyhow!("仓位不存在: {}", symbol))
    }
}

impl EngineMarket for Backtest {
    fn symbol(&self, symbol: &str) -> Result<Symbol> {
        self.account
            .lock()
            .symbols
            .get(symbol)
            .cloned()
            .ok_or(anyhow!("交易对不存在: {}", symbol))
    }
    fn symbol_rule(&self, symbol: &str) -> Result<SymbolRule> {
        Ok(self.symbol(symbol)?.rule)
    }
    fn symbol_index(&self, symbol: &str) -> Result<SymbolIndex> {
        Ok(self.symbol(symbol)?.index)
    }
}

fn new(mut config: BacktestConfig) -> Result<Arc<Backtest>> {
    config.begin = config.begin.duration_trunc(Duration::minutes(1))?;
    config.end = config.end.duration_trunc(Duration::minutes(1))?;
    ensure!(config.begin < config.end, "开始时间必须小于结束时间");

    config.cash = config.cash.to_safe();
    ensure!(config.cash >= 0.0, "初始资金必须大于等于0");

    config.fee_rate_taker = config.fee_rate_taker.to_safe();
    config.fee_rate_maker = config.fee_rate_maker.to_safe();

    config.slippage_rate = config.slippage_rate.to_safe();
    ensure!(config.slippage_rate >= 0.0, "滑点率必须大于等于0");

    let config = Arc::new(config);

    let account = Arc::new(Mutex::new(Account {
        cash: Cash {
            size: config.cash,
            available: config.cash,
            frozen: 0.,
        },
        symbols: Default::default(),
        positions: Default::default(),
        orders: Default::default(),
    }));
    let trade_time = Arc::new(Mutex::new(config.begin));

    Ok(Arc::new(Backtest {
        config,
        account,
        trade_time,
    }))
}

pub async fn run(config: BacktestConfig, strategy: Arc<dyn Strategy>) -> Result<()> {
    let backtest = new(config)?;

    strategy.on_init(backtest.clone()).await?;

    ensure!(
        !backtest.account.lock().symbols.is_empty(),
        "未初始化交易对"
    );

    let symbols = backtest
        .account
        .lock()
        .symbols
        .keys()
        .cloned()
        .collect::<Vec<String>>();

    strategy.on_start(backtest.clone()).await?;
    strategy.on_stop(backtest.clone()).await?;
    Ok(())
}
