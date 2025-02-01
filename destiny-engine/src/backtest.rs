use crate::{history_data::*, traits::*};
use anyhow::{anyhow, ensure, Result};
use async_trait::async_trait;
use chrono::{DateTime, Duration, Timelike, Utc};
use derive_builder::Builder;
use destiny_helpers::prelude::*;
use destiny_types::prelude::*;
use parking_lot::Mutex;
use rayon::prelude::*;
use std::{collections::HashMap, sync::Arc};
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

pub struct Backtest {
    config: Arc<BacktestConfig>,
    account: Arc<Mutex<Account>>,
    trade_time: Arc<Mutex<DateTime<Utc>>>,
}

impl Engine for Backtest {
    fn time(&self) -> DateTime<Utc> {
        *self.trade_time.lock()
    }

    fn stop(&self) {}
}

impl EngineInit for Backtest {
    fn symbol_init(&self, symbol: &str) -> Result<()> {
        ensure!(
            !self.account.lock().positions.contains_key(symbol),
            "重复初始化交易对: {}",
            symbol
        );
        self.account.lock().positions.insert(
            symbol.to_string(),
            SymbolPosition {
                symbol: Symbol {
                    symbol: symbol.to_string(),
                    enable: true,
                    rule: SymbolRule {
                        price_min: 1e-8,
                        price_max: 1e8,
                        price_tick: 1e-8,
                        size_min: 1e-8,
                        size_max: 1e8,
                        size_tick: 1e-8,
                        amount_min: 1e-8,
                        order_max: 200,
                    },
                    market: SymbolMarket {
                        mark: 0.,
                        index: 0.,
                        last: 0.,
                        settlement: 0.,
                        settlement_time: Default::default(),
                        time: Default::default(),
                    },
                },
                leverage: 1,
                long: Position {
                    side: TradeSide::Long,
                    price: 0.,
                    size: 0.,
                },
                short: Position {
                    side: TradeSide::Short,
                    price: 0.,
                    size: 0.,
                },
                orders: Default::default(),
            },
        );

        Ok(())
    }
}

impl EngineAccount for Backtest {
    fn order(&self, symbol: &str, id: &str) -> Option<Order> {
        self.account
            .lock()
            .positions
            .get(symbol)
            .and_then(|position| position.orders.get(id))
            .cloned()
    }
    fn orders(&self, symbol: &str) -> Vec<Order> {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.orders.values().cloned().collect::<Vec<Order>>())
            .unwrap_or_default()
    }
    fn orders_long(&self, symbol: &str) -> Vec<Order> {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| {
                position
                    .orders
                    .par_iter()
                    .filter_map(|(_, o)| {
                        if o.side == TradeSide::Long {
                            Some(o.to_owned())
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
    fn orders_long_open(&self, symbol: &str) -> Vec<Order> {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| {
                position
                    .orders
                    .par_iter()
                    .filter_map(|(_, o)| {
                        if o.side == TradeSide::Long && !o.reduce_only {
                            Some(o.to_owned())
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
    fn orders_long_close(&self, symbol: &str) -> Vec<Order> {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| {
                position
                    .orders
                    .par_iter()
                    .filter_map(|(_, o)| {
                        if o.side == TradeSide::Long && o.reduce_only {
                            Some(o.to_owned())
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
    fn orders_short(&self, symbol: &str) -> Vec<Order> {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| {
                position
                    .orders
                    .par_iter()
                    .filter_map(|(_, o)| {
                        if o.side == TradeSide::Short {
                            Some(o.to_owned())
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
    fn orders_short_open(&self, symbol: &str) -> Vec<Order> {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| {
                position
                    .orders
                    .par_iter()
                    .filter_map(|(_, o)| {
                        if o.side == TradeSide::Short && !o.reduce_only {
                            Some(o.to_owned())
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
    fn orders_short_close(&self, symbol: &str) -> Vec<Order> {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| {
                position
                    .orders
                    .par_iter()
                    .filter_map(|(_, o)| {
                        if o.side == TradeSide::Short && o.reduce_only {
                            Some(o.to_owned())
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
    fn leverage(&self, symbol: &str) -> u32 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|p| p.leverage)
            .unwrap_or(1)
    }
    fn cash(&self) -> f64 {
        self.account.lock().cash
    }
    fn cash_available(&self) -> f64 {
        self.account.lock().cash_available()
    }
    fn cash_frozen(&self) -> f64 {
        self.account.lock().cash_frozen()
    }
    fn margin(&self) -> f64 {
        self.account.lock().margin()
    }
    fn pnl(&self) -> f64 {
        self.account.lock().pnl()
    }
    fn long_price(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.long.price)
            .unwrap_or_default()
    }
    fn long_size(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.long.size)
            .unwrap_or_default()
    }
    fn long_size_available(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.long_size_available())
            .unwrap_or_default()
    }
    fn long_size_frozen(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.long_size_frozen())
            .unwrap_or_default()
    }
    fn long_margin(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.margin_long())
            .unwrap_or_default()
    }
    fn long_pnl(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.long_pnl())
            .unwrap_or_default()
    }
    fn short_price(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.short.price)
            .unwrap_or_default()
    }
    fn short_size(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.short.size)
            .unwrap_or_default()
    }
    fn short_size_available(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.short_size_available())
            .unwrap_or_default()
    }
    fn short_size_frozen(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.short_size_frozen())
            .unwrap_or_default()
    }
    fn short_margin(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.margin_short())
            .unwrap_or_default()
    }
    fn short_pnl(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.short_pnl())
            .unwrap_or_default()
    }
    fn symbol_pnl(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.pnl())
            .unwrap_or_default()
    }
    fn symbol_margin(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.margin())
            .unwrap_or_default()
    }
}

impl EngineMarket for Backtest {
    fn price_mark(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.market.mark)
            .unwrap_or_default()
    }

    fn price_last(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.market.last)
            .unwrap_or_default()
    }

    fn price_index(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.market.index)
            .unwrap_or_default()
    }

    fn price_settlement(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.market.settlement)
            .unwrap_or_default()
    }

    fn time_settlement(&self, symbol: &str) -> DateTime<Utc> {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.market.settlement_time)
            .unwrap_or_default()
    }

    fn rule_price_min(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.rule.price_min)
            .unwrap_or_default()
    }

    fn rule_price_max(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.rule.price_max)
            .unwrap_or_default()
    }

    fn rule_price_tick(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.rule.price_tick)
            .unwrap_or_default()
    }

    fn rule_size_min(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.rule.size_min)
            .unwrap_or_default()
    }

    fn rule_size_max(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.rule.size_max)
            .unwrap_or_default()
    }

    fn rule_size_tick(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.rule.size_tick)
            .unwrap_or_default()
    }

    fn rule_amount_min(&self, symbol: &str) -> f64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.rule.amount_min)
            .unwrap_or_default()
    }

    fn rule_order_max(&self, symbol: &str) -> i64 {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.rule.order_max)
            .unwrap_or_default()
    }
}

#[async_trait]
impl EngineExchange for Backtest {
    async fn long_market_open(&self, symbol: &str, size: f64) -> Result<String> {
        ensure!(
            self.account.lock().positions.contains_key(symbol),
            "交易对不存在: {}",
            symbol
        );

        let price_mark = self.price_mark(symbol);
        let size_min = self.rule_size_min(symbol);
        let size_max = self.rule_size_max(symbol);
        let size_tick = self.rule_size_tick(symbol);
        let amount_min = self.rule_amount_min(symbol);
        let leverage = self.leverage(symbol) as f64;
        let cash_available = self.cash_available();

        let size = (size - (size_tick % size)).to_safe();
        ensure!(
            size >= size_min,
            "最小数量限制: 数量({}),限制({})",
            size,
            size_min,
        );
        ensure!(
            size <= size_max,
            "最大数量限制: 数量({}),限制({})",
            size,
            size_max,
        );

        let amount = (size * price_mark).to_safe();
        ensure!(
            amount >= amount_min,
            "最小金额限制: 金额({}),限制({})",
            amount,
            amount_min,
        );

        let margin = (amount / leverage).to_safe();
        ensure!(
            cash_available >= margin,
            "保证金不足: 保证金({}),可用({})",
            margin,
            cash_available
        );

        let order_id = String::gen_id();
        self.account
            .lock()
            .positions
            .get_mut(symbol)
            .unwrap()
            .orders
            .insert(
                order_id.clone(),
                Order {
                    id: order_id.clone(),
                    symbol: symbol.to_string(),
                    r#type: TradeType::Market,
                    side: TradeSide::Long,
                    reduce_only: false,
                    status: OrderStatus::Created,
                    price: 0.,
                    size,
                    deal_price: 0.,
                    deal_size: 0.,
                    deal_fee: 0.,
                    create_time: self.time(),
                },
            );
        Ok(order_id)
    }
    async fn long_limit_open(&self, symbol: &str, size: f64, price: f64) -> Result<String> {
        ensure!(
            self.account.lock().positions.contains_key(symbol),
            "交易对不存在: {}",
            symbol
        );

        let price_min = self.rule_price_min(symbol);
        let price_max = self.rule_price_max(symbol);
        let price_tick = self.rule_price_tick(symbol);
        let size_min = self.rule_size_min(symbol);
        let size_max = self.rule_size_max(symbol);
        let size_tick = self.rule_size_tick(symbol);
        let amount_min = self.rule_amount_min(symbol);
        let leverage = self.leverage(symbol) as f64;
        let cash_available = self.cash_available();

        let size = (size - (size_tick % size)).to_safe();
        ensure!(
            size >= size_min,
            "最小数量限制: 数量({}),限制({})",
            size,
            size_min,
        );
        ensure!(
            size <= size_max,
            "最大数量限制: 数量({}),限制({})",
            size,
            size_max,
        );

        let price = (price - (price_tick % price)).to_safe();
        ensure!(
            price >= price_min,
            "最低价格限制: 价格({}),限制({})",
            price,
            price_min
        );
        ensure!(
            price <= price_max,
            "最高价格限制: 价格({}),限制({})",
            price,
            price_max
        );

        let amount = (size * price).to_safe();
        ensure!(
            amount >= amount_min,
            "最小金额限制: 金额({}),限制({})",
            amount,
            amount_min
        );

        let margin = (amount / leverage).to_safe();
        ensure!(
            cash_available >= margin,
            "保证金不足: 保证金({}),可用({})",
            margin,
            cash_available
        );

        let order_id = String::gen_id();
        self.account
            .lock()
            .positions
            .get_mut(symbol)
            .unwrap()
            .orders
            .insert(
                order_id.clone(),
                Order {
                    id: order_id.clone(),
                    symbol: symbol.to_string(),
                    r#type: TradeType::Limit,
                    side: TradeSide::Long,
                    reduce_only: false,
                    status: OrderStatus::Created,
                    price,
                    size,
                    deal_price: 0.,
                    deal_size: 0.,
                    deal_fee: 0.,
                    create_time: self.time(),
                },
            );
        Ok(order_id)
    }
    async fn long_market_close(&self, symbol: &str, size: f64) -> Result<String> {
        ensure!(
            self.account.lock().positions.contains_key(symbol),
            "交易对不存在: {}",
            symbol
        );

        let size_min = self.rule_size_min(symbol);
        let size_tick = self.rule_size_tick(symbol);
        let long_size_available = self.long_size_available(symbol);

        let size = (size - (size_tick % size)).to_safe();
        ensure!(
            size >= size_min,
            "最小数量限制: 数量({}),限制({})",
            size,
            size_min
        );

        ensure!(
            long_size_available >= size,
            "持仓数量不足: 数量({}),限制({})",
            long_size_available,
            size
        );

        let order_id = String::gen_id();
        self.account
            .lock()
            .positions
            .get_mut(symbol)
            .unwrap()
            .orders
            .insert(
                order_id.clone(),
                Order {
                    id: order_id.clone(),
                    symbol: symbol.to_string(),
                    r#type: TradeType::Market,
                    side: TradeSide::Long,
                    reduce_only: true,
                    status: OrderStatus::Created,
                    price: 0.,
                    size,
                    deal_price: 0.,
                    deal_size: 0.,
                    deal_fee: 0.,
                    create_time: self.time(),
                },
            );
        Ok(order_id)
    }
    async fn long_limit_close(&self, symbol: &str, size: f64, price: f64) -> Result<String> {
        ensure!(
            self.account.lock().positions.contains_key(symbol),
            "交易对不存在: {}",
            symbol
        );

        let price_min = self.rule_price_min(symbol);
        let price_max = self.rule_price_max(symbol);
        let price_tick = self.rule_price_tick(symbol);
        let size_min = self.rule_size_min(symbol);
        let size_tick = self.rule_size_tick(symbol);
        let long_size_available = self.long_size_available(symbol);

        let size = (size - (size_tick % size)).to_safe();
        ensure!(
            size >= size_min,
            "最小数量限制: 数量({}),限制({})",
            size,
            size_min
        );

        let price = (price - (price_tick % price)).to_safe();
        ensure!(
            price >= price_min,
            "最低价格限制: 价格({}),限制({})",
            price,
            price_min
        );
        ensure!(
            price <= price_max,
            "最高价格限制: 价格({}),限制({})",
            price,
            price_max
        );

        ensure!(
            long_size_available >= size,
            "持仓数量不足: 数量({}),限制({})",
            long_size_available,
            size
        );

        let order_id = String::gen_id();
        self.account
            .lock()
            .positions
            .get_mut(symbol)
            .unwrap()
            .orders
            .insert(
                order_id.clone(),
                Order {
                    id: order_id.clone(),
                    symbol: symbol.to_string(),
                    r#type: TradeType::Limit,
                    side: TradeSide::Long,
                    reduce_only: true,
                    status: OrderStatus::Created,
                    price,
                    size,
                    deal_price: 0.,
                    deal_size: 0.,
                    deal_fee: 0.,
                    create_time: self.time(),
                },
            );
        Ok(order_id)
    }
    async fn short_market_open(&self, symbol: &str, size: f64) -> Result<String> {
        ensure!(
            self.account.lock().positions.contains_key(symbol),
            "交易对不存在: {}",
            symbol
        );

        let price_mark = self.price_mark(symbol);
        let size_min = self.rule_size_min(symbol);
        let size_max = self.rule_size_max(symbol);
        let size_tick = self.rule_size_tick(symbol);
        let amount_min = self.rule_amount_min(symbol);
        let leverage = self.leverage(symbol) as f64;
        let cash_available = self.cash_available();

        let size = (size - (size_tick % size)).to_safe();
        ensure!(
            size >= size_min,
            "最小数量限制: 数量({}),限制({})",
            size,
            size_min
        );
        ensure!(
            size <= size_max,
            "最大数量限制: 数量({}),限制({})",
            size,
            size_max,
        );

        let amount = (size * price_mark).to_safe();
        ensure!(
            amount >= amount_min,
            "最小金额限制: 金额({}),限制({})",
            amount,
            amount_min
        );

        let margin = (amount / leverage).to_safe();
        ensure!(
            cash_available >= margin,
            "保证金不足: 保证金({}),可用({})",
            margin,
            cash_available
        );

        let order_id = String::gen_id();
        self.account
            .lock()
            .positions
            .get_mut(symbol)
            .unwrap()
            .orders
            .insert(
                order_id.clone(),
                Order {
                    id: order_id.clone(),
                    symbol: symbol.to_string(),
                    r#type: TradeType::Market,
                    side: TradeSide::Short,
                    reduce_only: false,
                    status: OrderStatus::Created,
                    price: 0.,
                    size,
                    deal_price: 0.,
                    deal_size: 0.,
                    deal_fee: 0.,
                    create_time: self.time(),
                },
            );
        Ok(order_id)
    }
    async fn short_limit_open(&self, symbol: &str, size: f64, price: f64) -> Result<String> {
        ensure!(
            self.account.lock().positions.contains_key(symbol),
            "交易对不存在: {}",
            symbol
        );

        let price_min = self.rule_price_min(symbol);
        let price_max = self.rule_price_max(symbol);
        let price_tick = self.rule_price_tick(symbol);
        let size_min = self.rule_size_min(symbol);
        let size_max = self.rule_size_max(symbol);
        let size_tick = self.rule_size_tick(symbol);
        let amount_min = self.rule_amount_min(symbol);
        let leverage = self.leverage(symbol) as f64;
        let cash_available = self.cash_available();

        let size = (size - (size_tick % size)).to_safe();
        ensure!(
            size >= size_min,
            "最小数量限制: 数量({}),限制({})",
            size,
            size_min
        );
        ensure!(
            size <= size_max,
            "最大数量限制: 数量({}),限制({})",
            size,
            size_max,
        );

        let price = (price - (price_tick % price)).to_safe();
        ensure!(
            price >= price_min,
            "最低价格限制: 价格({}),限制({})",
            price,
            price_min
        );
        ensure!(
            price <= price_max,
            "最高价格限制: 价格({}),限制({})",
            price,
            price_max
        );

        let amount = (size * price).to_safe();
        ensure!(
            amount >= amount_min,
            "最小金额限制: 金额({}),限制({})",
            amount,
            amount_min
        );

        let margin = (amount / leverage).to_safe();
        ensure!(
            cash_available >= margin,
            "保证金不足: 保证金({}),可用({})",
            margin,
            cash_available
        );

        let order_id = String::gen_id();
        self.account
            .lock()
            .positions
            .get_mut(symbol)
            .unwrap()
            .orders
            .insert(
                order_id.clone(),
                Order {
                    id: order_id.clone(),
                    symbol: symbol.to_string(),
                    r#type: TradeType::Limit,
                    side: TradeSide::Short,
                    reduce_only: false,
                    status: OrderStatus::Created,
                    price,
                    size,
                    deal_price: 0.,
                    deal_size: 0.,
                    deal_fee: 0.,
                    create_time: self.time(),
                },
            );
        Ok(order_id)
    }
    async fn short_market_close(&self, symbol: &str, size: f64) -> Result<String> {
        ensure!(
            self.account.lock().positions.contains_key(symbol),
            "交易对不存在: {}",
            symbol
        );

        let size_tick = self.rule_size_tick(symbol);
        let size_min = self.rule_size_min(symbol);
        let short_size_available = self.short_size_available(symbol);

        let size = (size - (size_tick % size)).to_safe();
        ensure!(
            size >= size_min,
            "最小数量限制: 数量({}),限制({})",
            size,
            size_min
        );

        ensure!(
            short_size_available >= size,
            "持仓数量不足: 数量({}),限制({})",
            short_size_available,
            size
        );

        let order_id = String::gen_id();
        self.account
            .lock()
            .positions
            .get_mut(symbol)
            .unwrap()
            .orders
            .insert(
                order_id.clone(),
                Order {
                    id: order_id.clone(),
                    symbol: symbol.to_string(),
                    r#type: TradeType::Market,
                    side: TradeSide::Short,
                    reduce_only: true,
                    status: OrderStatus::Created,
                    price: 0.,
                    size,
                    deal_price: 0.,
                    deal_size: 0.,
                    deal_fee: 0.,
                    create_time: self.time(),
                },
            );
        Ok(order_id)
    }
    async fn short_limit_close(&self, symbol: &str, size: f64, price: f64) -> Result<String> {
        ensure!(
            self.account.lock().positions.contains_key(symbol),
            "交易对不存在: {}",
            symbol
        );

        let size_tick = self.rule_size_tick(symbol);
        let price_min = self.rule_price_min(symbol);
        let price_max = self.rule_price_max(symbol);
        let price_tick = self.rule_price_tick(symbol);
        let size_min = self.rule_size_min(symbol);
        let short_size_available = self.short_size_available(symbol);

        let size = (size - (size_tick % size)).to_safe();
        ensure!(
            size >= size_min,
            "最小数量限制: 数量({}),限制({})",
            size,
            size_min
        );

        let price = (price - (price_tick % price)).to_safe();
        ensure!(
            price >= price_min,
            "最低价格限制: 价格({}),限制({})",
            price,
            price_min
        );
        ensure!(
            price <= price_max,
            "最高价格限制: 价格({}),限制({})",
            price,
            price_max
        );

        ensure!(
            short_size_available >= size,
            "持仓数量不足: 数量({}),限制({})",
            short_size_available,
            size
        );

        let order_id = String::gen_id();
        self.account
            .lock()
            .positions
            .get_mut(symbol)
            .unwrap()
            .orders
            .insert(
                order_id.clone(),
                Order {
                    id: order_id.clone(),
                    symbol: symbol.to_string(),
                    r#type: TradeType::Limit,
                    side: TradeSide::Short,
                    reduce_only: true,
                    status: OrderStatus::Created,
                    price,
                    size,
                    deal_price: 0.,
                    deal_size: 0.,
                    deal_fee: 0.,
                    create_time: self.time(),
                },
            );
        Ok(order_id)
    }
    async fn order_close(&self, symbol: &str, id: &str) -> Result<()> {
        self.account
            .lock()
            .positions
            .get_mut(symbol)
            .ok_or(anyhow!("交易对不存在: {}", symbol))?
            .orders
            .remove(id);
        Ok(())
    }
    async fn order_cancel_many(&self, symbol: &str, ids: &[String]) -> Result<()> {
        for id in ids {
            self.account
                .lock()
                .positions
                .get_mut(symbol)
                .ok_or(anyhow!("交易对不存在: {}", symbol))?
                .orders
                .remove(id);
        }
        Ok(())
    }
    async fn leverage_set(&self, symbol: &str, leverage: u32) -> Result<()> {
        ensure!(leverage >= 1, "杠杆倍率必须大于等于1");
        self.account
            .lock()
            .positions
            .get_mut(symbol)
            .ok_or(anyhow!("交易对不存在: {}", symbol))?
            .leverage = leverage;
        Ok(())
    }
}

struct SymbolHistoryData_ {
    funding_rate: HistoryDataStream<FundingRateHistory>,
    klines: HistoryDataStream<Kline>,
    index_price_klines: HistoryDataStream<Kline>,
    mark_price_klines: HistoryDataStream<Kline>,
}

struct SymbolHistoryData(HashMap<String, SymbolHistoryData_>);

impl SymbolHistoryData {
    pub fn new(symbols: &[String], begin: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        let mut result = HashMap::new();
        for symbol in symbols {
            let history_data = SymbolHistoryData_ {
                funding_rate: HistoryDataStream::new(
                    symbol.to_owned(),
                    HistoryDataStreamType::FundingRate,
                    begin,
                    end,
                ),
                klines: HistoryDataStream::new(
                    symbol.to_owned(),
                    HistoryDataStreamType::Klines,
                    begin,
                    end,
                ),
                index_price_klines: HistoryDataStream::new(
                    symbol.to_owned(),
                    HistoryDataStreamType::IndexPriceKlines,
                    begin,
                    end,
                ),
                mark_price_klines: HistoryDataStream::new(
                    symbol.to_owned(),
                    HistoryDataStreamType::MarkPriceKlines,
                    begin,
                    end,
                ),
            };
            result.insert(symbol.to_owned(), history_data);
        }
        Self(result)
    }

    async fn flush_market_settlement_price(
        &mut self,
        backtest: &Backtest,
        symbol: &str,
        date: DateTime<Utc>,
    ) -> Result<()> {
        let history_data = self.0.get_mut(symbol).unwrap();
        if let Some(funding_rate) = history_data.funding_rate.take(date).await? {
            let mut account = backtest.account.lock();
            let symbol_position = account.positions.get_mut(symbol).unwrap();
            symbol_position.symbol.market.settlement = funding_rate.rate;
            symbol_position.symbol.market.settlement_time = date + Duration::hours(8);
        }
        Ok(())
    }

    async fn flush_market_last_price(
        &mut self,
        backtest: &Backtest,
        symbol: &str,
        date: DateTime<Utc>,
    ) -> Result<()> {
        let history_data = self.0.get_mut(symbol).unwrap();
        if let Some(kline) = history_data.klines.take(date).await? {
            let mut account = backtest.account.lock();
            let symbol_position = account.positions.get_mut(symbol).unwrap();
            symbol_position.symbol.market.last = kline.open;
        }
        Ok(())
    }

    async fn flush_market_index_price(
        &mut self,
        backtest: &Backtest,
        symbol: &str,
        date: DateTime<Utc>,
    ) -> Result<()> {
        let history_data = self.0.get_mut(symbol).unwrap();
        if let Some(kline) = history_data.index_price_klines.take(date).await? {
            let mut account = backtest.account.lock();
            let symbol_position = account.positions.get_mut(symbol).unwrap();
            symbol_position.symbol.market.index = kline.open;
        }
        Ok(())
    }

    async fn flush_market_mark_price(
        &mut self,
        backtest: &Backtest,
        symbol: &str,
        date: DateTime<Utc>,
    ) -> Result<()> {
        let history_data = self.0.get_mut(symbol).unwrap();
        if let Some(kline) = history_data.mark_price_klines.take(date).await? {
            let mut account = backtest.account.lock();
            let symbol_position = account.positions.get_mut(symbol).unwrap();
            symbol_position.symbol.market.mark = kline.open;
        }
        Ok(())
    }

    pub async fn flush_market(
        &mut self,
        backtest: &Backtest,
        symbols: &[String],
        date: DateTime<Utc>,
    ) -> Result<()> {
        for symbol in symbols {
            self.flush_market_settlement_price(backtest, symbol, date)
                .await?;
            self.flush_market_last_price(backtest, symbol, date).await?;
            self.flush_market_index_price(backtest, symbol, date)
                .await?;
            self.flush_market_mark_price(backtest, symbol, date).await?;
        }
        Ok(())
    }
}

impl Backtest {
    fn new(mut config: BacktestConfig) -> Result<Arc<Backtest>> {
        config.begin = config.begin.truncate_minute()?;
        config.end = config.end.truncate_minute()?;
        ensure!(config.begin < config.end, "开始时间必须小于结束时间");

        config.cash = config.cash.to_safe();
        ensure!(config.cash >= 0.0, "初始资金必须大于等于0");

        config.fee_rate_taker = config.fee_rate_taker.to_safe();
        config.fee_rate_maker = config.fee_rate_maker.to_safe();

        config.slippage_rate = config.slippage_rate.to_safe();
        ensure!(config.slippage_rate >= 0.0, "滑点率必须大于等于0");

        let config = Arc::new(config);

        let account = Arc::new(Mutex::new(Account {
            cash: config.cash,
            positions: Default::default(),
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
            !backtest.account.lock().positions.is_empty(),
            "未初始化交易对"
        );

        let symbols = backtest
            .account
            .lock()
            .positions
            .keys()
            .cloned()
            .collect::<Vec<String>>();

        strategy.on_start(backtest.clone()).await?;

        let mut begin = backtest.config.begin;
        let end = backtest.config.end;

        let mut symbol_history_data = SymbolHistoryData::new(&symbols, begin, end);

        let backtest_instant = Instant::now();

        while begin <= end {
            *backtest.trade_time.lock() = begin;

            // 更新市场行情
            symbol_history_data
                .flush_market(&backtest, &symbols, begin)
                .await?;

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

        tracing::debug!("回测耗时: {:?}", backtest_instant.elapsed());

        strategy.on_stop(backtest.clone()).await?;

        Ok(())
    }
}
