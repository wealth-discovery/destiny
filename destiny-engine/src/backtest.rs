use crate::{history_data, traits::*};
use anyhow::{ensure, Result};
use async_trait::async_trait;
use chrono::{
    DateTime, Datelike, Duration, DurationRound, Months, NaiveDate, NaiveDateTime, NaiveTime,
    TimeZone, Utc,
};
use derive_builder::Builder;
use destiny_helpers::prelude::*;
use destiny_types::prelude::*;
use parking_lot::Mutex;
use std::sync::Arc;
use strum::IntoEnumIterator;
use tracing::instrument;

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

pub struct Backtest {
    config: Arc<BacktestConfig>,
    account: Arc<Mutex<Account>>,
    trade_time: Arc<Mutex<DateTime<Utc>>>,
}

impl Engine for Backtest {}

impl EngineBasic for Backtest {
    fn now_ms(&self) -> i64 {
        self.trade_time.lock().timestamp_millis()
    }
    fn now(&self) -> DateTime<Utc> {
        self.trade_time.lock().clone()
    }
}

impl EngineInit for Backtest {
    /// 初始化交易对
    fn init_symbol(&self, symbol: &str) -> Result<()> {
        ensure!(
            !self.account.lock().symbols.contains_key(symbol),
            "symbol already exists: {}",
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
        backtest.account.lock().symbols.len() > 0,
        "no symbols initialized"
    );

    let symbols = backtest
        .account
        .lock()
        .symbols
        .keys()
        .cloned()
        .collect::<Vec<String>>();

    sync_history_data(&symbols).await?;

    strategy.on_start(backtest.clone()).await?;
    strategy.on_stop(backtest.clone()).await?;
    Ok(())
}

#[instrument(name = "SyncHistoryData")]
async fn sync_history_data(symbols: &[String]) -> Result<()> {
    let mut start = Utc.from_utc_datetime(&NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
    ));
    let end = now()
        .with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        .unwrap()
        .with_day(1)
        .unwrap()
        - Duration::days(1);

    while start <= end {
        for symbol in symbols {
            history_data::sync(history_data::SyncHistoryMeta::agg_trades(
                &symbol,
                start.year() as i64,
                start.month() as i64,
            ))
            .await?;

            history_data::sync(history_data::SyncHistoryMeta::book_ticker(
                &symbol,
                start.year() as i64,
                start.month() as i64,
            ))
            .await?;

            history_data::sync(history_data::SyncHistoryMeta::funding_rate(
                &symbol,
                start.year() as i64,
                start.month() as i64,
            ))
            .await?;

            history_data::sync(history_data::SyncHistoryMeta::trades(
                &symbol,
                start.year() as i64,
                start.month() as i64,
            ))
            .await?;

            for interval in KlineInterval::iter() {
                history_data::sync(history_data::SyncHistoryMeta::index_price_klines(
                    &symbol,
                    interval,
                    start.year() as i64,
                    start.month() as i64,
                ))
                .await?;

                history_data::sync(history_data::SyncHistoryMeta::klines(
                    &symbol,
                    interval,
                    start.year() as i64,
                    start.month() as i64,
                ))
                .await?;

                history_data::sync(history_data::SyncHistoryMeta::mark_price_klines(
                    &symbol,
                    interval,
                    start.year() as i64,
                    start.month() as i64,
                ))
                .await?;

                history_data::sync(history_data::SyncHistoryMeta::premium_index_klines(
                    &symbol,
                    interval,
                    start.year() as i64,
                    start.month() as i64,
                ))
                .await?;
            }
        }
        start = start + Months::new(1);
    }

    Ok(())
}
