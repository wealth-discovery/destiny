use crate::traits::*;
use anyhow::{anyhow, ensure, Result};
use async_trait::async_trait;
use chrono::{DateTime, Duration, DurationRound, Timelike, Utc};
use derive_builder::Builder;
use destiny_helpers::prelude::*;
use destiny_types::prelude::*;
use parking_lot::Mutex;
use rayon::prelude::*;
use std::sync::Arc;
use tokio::time::Instant;

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
                    amount_min: 1e-8,
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
    fn now(&self) -> DateTime<Utc> {
        *self.trade_time.lock()
    }
    async fn open_market_long(&self, symbol: &str, size: f64) -> Result<String> {
        let symbol_index = self.symbol_index(symbol)?;
        let symbol_rule = self.symbol_rule(symbol)?;
        let cash = self.cash();
        let position = self.position(symbol)?;

        let size = (symbol_rule.size_tick % size).to_safe();
        ensure!(
            size >= symbol_rule.size_min,
            "最小数量限制: 数量({}),限制({})",
            size,
            symbol_rule.size_min,
        );
        ensure!(
            size <= symbol_rule.size_max,
            "最大数量限制: 数量({}),限制({})",
            size,
            symbol_rule.size_max,
        );

        let amount = (size * symbol_index.mark_price).to_safe();
        ensure!(
            amount >= symbol_rule.amount_min,
            "最小金额限制: 金额({}),限制({})",
            amount,
            symbol_rule.amount_min,
        );

        let margin = (amount / position.leverage as f64).to_safe();
        ensure!(
            cash.available >= margin,
            "保证金不足: 保证金({}),可用({})",
            margin,
            cash.available
        );

        self.account.lock().cash.available -= margin;
        self.account.lock().cash.frozen += margin;

        let order_id = String::gen_id();
        self.account.lock().orders.insert(
            order_id.clone(),
            Order {
                id: order_id.clone(),
                symbol: symbol.to_string(),
                type_: TradeType::Market,
                side: TradeSide::Long,
                reduce_only: false,
                status: OrderStatus::Created,
                price: 0.,
                size,
                deal_price: 0.,
                deal_size: 0.,
                deal_fee: 0.,
                create_time: self.now(),
            },
        );
        Ok(order_id)
    }
    async fn open_limit_long(&self, symbol: &str, size: f64, price: f64) -> Result<String> {
        let symbol_rule = self.symbol_rule(symbol)?;
        let cash = self.cash();
        let position = self.position(symbol)?;

        let size = (symbol_rule.size_tick % size).to_safe();
        ensure!(
            size >= symbol_rule.size_min,
            "最小数量限制: 数量({}),限制({})",
            size,
            symbol_rule.size_min,
        );
        ensure!(
            size <= symbol_rule.size_max,
            "最大数量限制: 数量({}),限制({})",
            size,
            symbol_rule.size_max,
        );

        let price = (symbol_rule.price_tick % price).to_safe();
        ensure!(
            price >= symbol_rule.price_min,
            "最低价格限制: 价格({}),限制({})",
            price,
            symbol_rule.price_min
        );
        ensure!(
            price <= symbol_rule.price_max,
            "最高价格限制: 价格({}),限制({})",
            price,
            symbol_rule.price_max
        );

        let amount = (size * price).to_safe();
        ensure!(
            amount >= symbol_rule.amount_min,
            "最小金额限制: 金额({}),限制({})",
            amount,
            symbol_rule.amount_min
        );

        let margin = (amount / position.leverage as f64).to_safe();
        ensure!(
            cash.available >= margin,
            "保证金不足: 保证金({}),可用({})",
            margin,
            cash.available
        );

        self.account.lock().cash.available -= margin;
        self.account.lock().cash.frozen += margin;

        let order_id = String::gen_id();
        self.account.lock().orders.insert(
            order_id.clone(),
            Order {
                id: order_id.clone(),
                symbol: symbol.to_string(),
                type_: TradeType::Limit,
                side: TradeSide::Long,
                reduce_only: false,
                status: OrderStatus::Created,
                price,
                size,
                deal_price: 0.,
                deal_size: 0.,
                deal_fee: 0.,
                create_time: self.now(),
            },
        );
        Ok(order_id)
    }
    async fn close_market_long(&self, symbol: &str, size: f64) -> Result<String> {
        let symbol_rule = self.symbol_rule(symbol)?;
        let position = self.position(symbol)?;

        let size = (symbol_rule.size_tick % size).to_safe();
        ensure!(
            size >= symbol_rule.size_min,
            "最小数量限制: 数量({}),限制({})",
            size,
            symbol_rule.size_min
        );

        ensure!(
            position.long.size >= size,
            "持仓数量不足: 数量({}),限制({})",
            position.long.size,
            size
        );

        self.account
            .lock()
            .positions
            .get_mut(symbol)
            .unwrap()
            .long
            .available -= size;
        self.account
            .lock()
            .positions
            .get_mut(symbol)
            .unwrap()
            .long
            .frozen += size;

        let order_id = String::gen_id();
        self.account.lock().orders.insert(
            order_id.clone(),
            Order {
                id: order_id.clone(),
                symbol: symbol.to_string(),
                type_: TradeType::Market,
                side: TradeSide::Long,
                reduce_only: true,
                status: OrderStatus::Created,
                price: 0.,
                size,
                deal_price: 0.,
                deal_size: 0.,
                deal_fee: 0.,
                create_time: self.now(),
            },
        );
        Ok(order_id)
    }
    async fn close_limit_long(&self, symbol: &str, size: f64, price: f64) -> Result<String> {
        let symbol_rule = self.symbol_rule(symbol)?;
        let position = self.position(symbol)?;

        let size = (symbol_rule.size_tick % size).to_safe();
        ensure!(
            size >= symbol_rule.size_min,
            "最小数量限制: 数量({}),限制({})",
            size,
            symbol_rule.size_min
        );

        let price = (symbol_rule.price_tick % price).to_safe();
        ensure!(
            price >= symbol_rule.price_min,
            "最低价格限制: 价格({}),限制({})",
            price,
            symbol_rule.price_min
        );
        ensure!(
            price <= symbol_rule.price_max,
            "最高价格限制: 价格({}),限制({})",
            price,
            symbol_rule.price_max
        );

        ensure!(
            position.long.size >= size,
            "持仓数量不足: 数量({}),限制({})",
            position.long.size,
            size
        );

        self.account
            .lock()
            .positions
            .get_mut(symbol)
            .unwrap()
            .long
            .available -= size;
        self.account
            .lock()
            .positions
            .get_mut(symbol)
            .unwrap()
            .long
            .frozen += size;

        let order_id = String::gen_id();
        self.account.lock().orders.insert(
            order_id.clone(),
            Order {
                id: order_id.clone(),
                symbol: symbol.to_string(),
                type_: TradeType::Limit,
                side: TradeSide::Long,
                reduce_only: true,
                status: OrderStatus::Created,
                price,
                size,
                deal_price: 0.,
                deal_size: 0.,
                deal_fee: 0.,
                create_time: self.now(),
            },
        );
        Ok(order_id)
    }
    async fn open_market_short(&self, symbol: &str, size: f64) -> Result<String> {
        let symbol_index = self.symbol_index(symbol)?;
        let symbol_rule = self.symbol_rule(symbol)?;
        let cash = self.cash();
        let position = self.position(symbol)?;

        let size = (symbol_rule.size_tick % size).to_safe();
        ensure!(
            size >= symbol_rule.size_min,
            "最小数量限制: 数量({}),限制({})",
            size,
            symbol_rule.size_min
        );
        ensure!(
            size <= symbol_rule.size_max,
            "最大数量限制: 数量({}),限制({})",
            size,
            symbol_rule.size_max,
        );

        let amount = (size * symbol_index.mark_price).to_safe();
        ensure!(
            amount >= symbol_rule.amount_min,
            "最小金额限制: 金额({}),限制({})",
            amount,
            symbol_rule.amount_min
        );

        let margin = (amount / position.leverage as f64).to_safe();
        ensure!(
            cash.available >= margin,
            "保证金不足: 保证金({}),可用({})",
            margin,
            cash.available
        );

        self.account.lock().cash.available -= margin;
        self.account.lock().cash.frozen += margin;

        let order_id = String::gen_id();
        self.account.lock().orders.insert(
            order_id.clone(),
            Order {
                id: order_id.clone(),
                symbol: symbol.to_string(),
                type_: TradeType::Market,
                side: TradeSide::Short,
                reduce_only: false,
                status: OrderStatus::Created,
                price: 0.,
                size,
                deal_price: 0.,
                deal_size: 0.,
                deal_fee: 0.,
                create_time: self.now(),
            },
        );
        Ok(order_id)
    }
    async fn open_limit_short(&self, symbol: &str, size: f64, price: f64) -> Result<String> {
        let symbol_rule = self.symbol_rule(symbol)?;
        let cash = self.cash();
        let position = self.position(symbol)?;

        let size = (symbol_rule.size_tick % size).to_safe();
        ensure!(
            size >= symbol_rule.size_min,
            "最小数量限制: 数量({}),限制({})",
            size,
            symbol_rule.size_min
        );
        ensure!(
            size <= symbol_rule.size_max,
            "最大数量限制: 数量({}),限制({})",
            size,
            symbol_rule.size_max,
        );

        let price = (symbol_rule.price_tick % price).to_safe();
        ensure!(
            price >= symbol_rule.price_min,
            "最低价格限制: 价格({}),限制({})",
            price,
            symbol_rule.price_min
        );
        ensure!(
            price <= symbol_rule.price_max,
            "最高价格限制: 价格({}),限制({})",
            price,
            symbol_rule.price_max
        );

        let amount = (size * price).to_safe();
        ensure!(
            amount >= symbol_rule.amount_min,
            "最小金额限制: 金额({}),限制({})",
            amount,
            symbol_rule.amount_min
        );

        let margin = (amount / position.leverage as f64).to_safe();
        ensure!(
            cash.available >= margin,
            "保证金不足: 保证金({}),可用({})",
            margin,
            cash.available
        );

        self.account.lock().cash.available -= margin;
        self.account.lock().cash.frozen += margin;

        let order_id = String::gen_id();
        self.account.lock().orders.insert(
            order_id.clone(),
            Order {
                id: order_id.clone(),
                symbol: symbol.to_string(),
                type_: TradeType::Limit,
                side: TradeSide::Short,
                reduce_only: false,
                status: OrderStatus::Created,
                price,
                size,
                deal_price: 0.,
                deal_size: 0.,
                deal_fee: 0.,
                create_time: self.now(),
            },
        );
        Ok(order_id)
    }
    async fn close_market_short(&self, symbol: &str, size: f64) -> Result<String> {
        let symbol_rule = self.symbol_rule(symbol)?;
        let position = self.position(symbol)?;

        let size = (symbol_rule.size_tick % size).to_safe();
        ensure!(
            size >= symbol_rule.size_min,
            "最小数量限制: 数量({}),限制({})",
            size,
            symbol_rule.size_min
        );

        ensure!(
            position.short.size >= size,
            "持仓数量不足: 数量({}),限制({})",
            position.short.size,
            size
        );

        self.account
            .lock()
            .positions
            .get_mut(symbol)
            .unwrap()
            .short
            .available -= size;
        self.account
            .lock()
            .positions
            .get_mut(symbol)
            .unwrap()
            .short
            .frozen += size;

        let order_id = String::gen_id();
        self.account.lock().orders.insert(
            order_id.clone(),
            Order {
                id: order_id.clone(),
                symbol: symbol.to_string(),
                type_: TradeType::Market,
                side: TradeSide::Short,
                reduce_only: true,
                status: OrderStatus::Created,
                price: 0.,
                size,
                deal_price: 0.,
                deal_size: 0.,
                deal_fee: 0.,
                create_time: self.now(),
            },
        );
        Ok(order_id)
    }
    async fn close_limit_short(&self, symbol: &str, size: f64, price: f64) -> Result<String> {
        let symbol_rule = self.symbol_rule(symbol)?;
        let position = self.position(symbol)?;

        let size = (symbol_rule.size_tick % size).to_safe();
        ensure!(
            size >= symbol_rule.size_min,
            "最小数量限制: 数量({}),限制({})",
            size,
            symbol_rule.size_min
        );

        let price = (symbol_rule.price_tick % price).to_safe();
        ensure!(
            price >= symbol_rule.price_min,
            "最低价格限制: 价格({}),限制({})",
            price,
            symbol_rule.price_min
        );
        ensure!(
            price <= symbol_rule.price_max,
            "最高价格限制: 价格({}),限制({})",
            price,
            symbol_rule.price_max
        );

        ensure!(
            position.short.size >= size,
            "持仓数量不足: 数量({}),限制({})",
            position.short.size,
            size
        );

        self.account
            .lock()
            .positions
            .get_mut(symbol)
            .unwrap()
            .short
            .available -= size;
        self.account
            .lock()
            .positions
            .get_mut(symbol)
            .unwrap()
            .short
            .frozen += size;

        let order_id = String::gen_id();
        self.account.lock().orders.insert(
            order_id.clone(),
            Order {
                id: order_id.clone(),
                symbol: symbol.to_string(),
                type_: TradeType::Limit,
                side: TradeSide::Short,
                reduce_only: true,
                status: OrderStatus::Created,
                price,
                size,
                deal_price: 0.,
                deal_size: 0.,
                deal_fee: 0.,
                create_time: self.now(),
            },
        );
        Ok(order_id)
    }
    async fn order_cancel(&self, id: &str) -> Result<()> {
        self.account.lock().orders.remove(id);
        Ok(())
    }
    async fn orders_cancel(&self, ids: &[&str]) -> Result<()> {
        for id in ids {
            self.account.lock().orders.remove(*id);
        }
        Ok(())
    }
    async fn leverage_set(&self, symbol: &str, leverage: u32) -> Result<()> {
        ensure!(leverage >= 1, "杠杆倍率必须大于等于1");
        self.account
            .lock()
            .positions
            .get_mut(symbol)
            .unwrap()
            .leverage = leverage;
        Ok(())
    }
    fn leverage(&self, symbol: &str) -> u32 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|p| p.leverage)
            .unwrap_or(1)
    }
    fn order(&self, id: &str) -> Option<Order> {
        self.account.lock().orders.get(id).cloned()
    }
    fn orders(&self, symbol: &str) -> Vec<Order> {
        self.account
            .lock()
            .orders
            .par_iter()
            .filter_map(|(_, o)| {
                if o.symbol.as_str() == symbol {
                    Some(o.to_owned())
                } else {
                    None
                }
            })
            .collect()
    }
    fn orders_long(&self, symbol: &str) -> Vec<Order> {
        self.account
            .lock()
            .orders
            .par_iter()
            .filter_map(|(_, o)| {
                if o.symbol.as_str() == symbol && o.side == TradeSide::Long {
                    Some(o.to_owned())
                } else {
                    None
                }
            })
            .collect()
    }
    fn orders_open_long(&self, symbol: &str) -> Vec<Order> {
        self.account
            .lock()
            .orders
            .par_iter()
            .filter_map(|(_, o)| {
                if o.symbol.as_str() == symbol && o.side == TradeSide::Long && !o.reduce_only {
                    Some(o.to_owned())
                } else {
                    None
                }
            })
            .collect()
    }
    fn orders_close_long(&self, symbol: &str) -> Vec<Order> {
        self.account
            .lock()
            .orders
            .par_iter()
            .filter_map(|(_, o)| {
                if o.symbol.as_str() == symbol && o.side == TradeSide::Long && o.reduce_only {
                    Some(o.to_owned())
                } else {
                    None
                }
            })
            .collect()
    }
    fn orders_short(&self, symbol: &str) -> Vec<Order> {
        self.account
            .lock()
            .orders
            .par_iter()
            .filter_map(|(_, o)| {
                if o.symbol.as_str() == symbol && o.side == TradeSide::Short {
                    Some(o.to_owned())
                } else {
                    None
                }
            })
            .collect()
    }
    fn orders_open_short(&self, symbol: &str) -> Vec<Order> {
        self.account
            .lock()
            .orders
            .par_iter()
            .filter_map(|(_, o)| {
                if o.symbol.as_str() == symbol && o.side == TradeSide::Short && !o.reduce_only {
                    Some(o.to_owned())
                } else {
                    None
                }
            })
            .collect()
    }
    fn orders_close_short(&self, symbol: &str) -> Vec<Order> {
        self.account
            .lock()
            .orders
            .par_iter()
            .filter_map(|(_, o)| {
                if o.symbol.as_str() == symbol && o.side == TradeSide::Short && o.reduce_only {
                    Some(o.to_owned())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl EngineAccount for Backtest {
    fn cash(&self) -> Cash {
        self.account.lock().cash.to_owned()
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

impl Backtest {
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
        let backtest = Self::new(config)?;

        strategy.on_init(backtest.clone()).await?;

        ensure!(
            !backtest.account.lock().symbols.is_empty(),
            "未初始化交易对"
        );

        // let symbols = backtest
        //     .account
        //     .lock()
        //     .symbols
        //     .keys()
        //     .cloned()
        //     .collect::<Vec<String>>();

        strategy.on_start(backtest.clone()).await?;

        let mut begin = backtest.config.begin;
        let end = backtest.config.end;

        let event_instant = Instant::now();

        while begin <= end {
            *backtest.trade_time.lock() = begin;

            // 市场行情变化事件
            // 每日事件
            if begin.hour() == 0 && begin.minute() == 0 {
                let event_instant = Instant::now();
                if let Err(err) = strategy.on_daily(backtest.clone()).await {
                    tracing::error!("{} 每日事件执行失败: {}", begin.str_ymd_hm(), err);
                } else {
                    tracing::debug!(
                        "{} 每日事件执行耗时: {:?}",
                        begin.str_ymd_hm(),
                        event_instant.elapsed()
                    );
                }
            }
            // 每小时事件
            if begin.minute() == 0 {
                let event_instant = Instant::now();
                if let Err(err) = strategy.on_hourly(backtest.clone()).await {
                    tracing::error!("{} 每小时事件执行失败: {}", begin.str_ymd_hm(), err);
                } else {
                    tracing::debug!(
                        "{} 每小时事件执行耗时: {:?}",
                        begin.str_ymd_hm(),
                        event_instant.elapsed()
                    );
                }
            }
            // 每分钟事件
            {
                let event_instant = Instant::now();
                if let Err(err) = strategy.on_minutely(backtest.clone()).await {
                    tracing::error!("{} 每分钟事件执行失败: {}", begin.str_ymd_hm(), err);
                } else {
                    tracing::debug!(
                        "{} 每分钟事件执行耗时: {:?}",
                        begin.str_ymd_hm(),
                        event_instant.elapsed()
                    );
                }
            }
            // Tick事件
            {
                let event_instant = Instant::now();
                if let Err(err) = strategy.on_tick(backtest.clone()).await {
                    tracing::error!("{} Tick事件执行失败: {}", begin.str_ymd_hm(), err);
                } else {
                    tracing::debug!(
                        "{} Tick事件执行耗时: {:?}",
                        begin.str_ymd_hm(),
                        event_instant.elapsed()
                    );
                }
            }

            begin += Duration::minutes(1);
        }

        tracing::debug!("回测耗时: {:?}", event_instant.elapsed());

        strategy.on_stop(backtest.clone()).await?;

        Ok(())
    }
}
