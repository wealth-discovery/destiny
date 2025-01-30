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

impl EngineBasic for Backtest {
    fn now_ms(&self) -> i64 {
        self.trade_time.lock().timestamp_millis()
    }
    fn now(&self) -> DateTime<Utc> {
        *self.trade_time.lock()
    }
}

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
    async fn open_long_market(&self, symbol: &str, size: f64) -> Result<String> {
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

#[allow(unused_variables)]
impl EngineAccount for Backtest {
    fn cash(&self) -> Cash {
        todo!()
    }
    fn position(&self, symbol: &str) -> Result<SymbolPosition> {
        todo!()
    }
}

#[allow(unused_variables)]
impl EngineMarket for Backtest {
    fn symbol(&self, symbol: &str) -> Result<Symbol> {
        todo!()
    }
    fn symbol_rule(&self, symbol: &str) -> Result<SymbolRule> {
        todo!()
    }
    fn symbol_index(&self, symbol: &str) -> Result<SymbolIndex> {
        todo!()
    }
}

fn new(mut config: BacktestConfig) -> Result<Arc<Backtest>> {
    config.begin = config.begin.duration_trunc(Duration::minutes(1))?;
    config.end = config.end.duration_trunc(Duration::minutes(1))?;
    ensure!(config.begin < config.end, "开始时间必须小于结束时间");

    config.cash = truncate_float(config.cash, 8, false);
    ensure!(config.cash >= 0.0, "初始资金必须大于等于0");

    config.fee_rate_taker = truncate_float(config.fee_rate_taker, 8, false);
    config.fee_rate_maker = truncate_float(config.fee_rate_maker, 8, false);

    config.slippage_rate = truncate_float(config.slippage_rate, 8, false);
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

    sync_history_data(&symbols).await?;

    strategy.on_start(backtest.clone()).await?;
    strategy.on_stop(backtest.clone()).await?;
    Ok(())
}

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
            // history_data::sync(history_data::SyncHistoryMeta::agg_trades(
            //     symbol,
            //     start.year() as i64,
            //     start.month() as i64,
            // ))
            // .await;

            // history_data::sync(history_data::SyncHistoryMeta::book_ticker(
            //     symbol,
            //     start.year() as i64,
            //     start.month() as i64,
            // ))
            // .await;

            history_data::sync(history_data::SyncHistoryMeta::funding_rate(
                symbol,
                start.year() as i64,
                start.month() as i64,
            ))
            .await;

            // history_data::sync(history_data::SyncHistoryMeta::trades(
            //     symbol,
            //     start.year() as i64,
            //     start.month() as i64,
            // ))
            // .await;

            for interval in KlineInterval::iter() {
                history_data::sync(history_data::SyncHistoryMeta::index_price_klines(
                    symbol,
                    interval,
                    start.year() as i64,
                    start.month() as i64,
                ))
                .await;

                history_data::sync(history_data::SyncHistoryMeta::klines(
                    symbol,
                    interval,
                    start.year() as i64,
                    start.month() as i64,
                ))
                .await;

                history_data::sync(history_data::SyncHistoryMeta::mark_price_klines(
                    symbol,
                    interval,
                    start.year() as i64,
                    start.month() as i64,
                ))
                .await;

                history_data::sync(history_data::SyncHistoryMeta::premium_index_klines(
                    symbol,
                    interval,
                    start.year() as i64,
                    start.month() as i64,
                ))
                .await;
            }
        }
        start = start + Months::new(1);
    }

    Ok(())
}
