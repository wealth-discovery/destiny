use crate::{history_data::*, traits::*};
use anyhow::{anyhow, ensure, Result};
use async_trait::async_trait;
use chrono::{DateTime, Duration, Timelike, Utc};
use derive_builder::Builder;
use destiny_helpers::prelude::*;
use destiny_types::prelude::*;
use parking_lot::Mutex;
use rayon::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
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
    #[builder(default = dec!(1000))]
    pub cash: Decimal,
    /// 吃单手续费率
    #[builder(default = dec!(0.0005))]
    pub fee_rate_taker: Decimal,
    /// 挂单手续费率
    #[builder(default = dec!(0.0005))]
    pub fee_rate_maker: Decimal,
    /// 滑点
    #[builder(default = dec!(0.01))]
    pub slippage_rate: Decimal,
}

pub struct Backtest {
    config: Arc<BacktestConfig>,
    account: Arc<Mutex<Account>>,
    trade_time: Arc<Mutex<DateTime<Utc>>>,
    strategy: Arc<dyn Strategy>,
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
                        price_min: dec!(1e-8),
                        price_max: dec!(1e8),
                        price_tick: dec!(1e-8),
                        size_min: dec!(1e-8),
                        size_max: dec!(1e8),
                        size_tick: dec!(1e-8),
                        amount_min: dec!(1e-8),
                        order_max: 200,
                    },
                    market: SymbolMarket {
                        mark: Decimal::ZERO,
                        index: Decimal::ZERO,
                        last: Decimal::ZERO,
                        settlement: Decimal::ZERO,
                        settlement_time: Default::default(),
                        time: Default::default(),
                    },
                },
                leverage: 1,
                long: Position {
                    side: TradeSide::Long,
                    price: Decimal::ZERO,
                    size: Decimal::ZERO,
                },
                short: Position {
                    side: TradeSide::Short,
                    price: Decimal::ZERO,
                    size: Decimal::ZERO,
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
    fn cash(&self) -> Decimal {
        self.account.lock().cash
    }
    fn cash_available(&self) -> Decimal {
        self.account.lock().cash_available()
    }
    fn cash_frozen(&self) -> Decimal {
        self.account.lock().cash_frozen()
    }
    fn margin(&self) -> Decimal {
        self.account.lock().margin()
    }
    fn pnl(&self) -> Decimal {
        self.account.lock().pnl()
    }
    fn long_price(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.long.price)
            .unwrap_or_default()
    }
    fn long_size(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.long.size)
            .unwrap_or_default()
    }
    fn long_size_available(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.long_size_available())
            .unwrap_or_default()
    }
    fn long_size_frozen(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.long_size_frozen())
            .unwrap_or_default()
    }
    fn long_margin(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.margin_long())
            .unwrap_or_default()
    }
    fn long_pnl(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.long_pnl())
            .unwrap_or_default()
    }
    fn short_price(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.short.price)
            .unwrap_or_default()
    }
    fn short_size(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.short.size)
            .unwrap_or_default()
    }
    fn short_size_available(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.short_size_available())
            .unwrap_or_default()
    }
    fn short_size_frozen(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.short_size_frozen())
            .unwrap_or_default()
    }
    fn short_margin(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.margin_short())
            .unwrap_or_default()
    }
    fn short_pnl(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.short_pnl())
            .unwrap_or_default()
    }
    fn symbols(&self) -> Vec<String> {
        self.account.lock().positions.keys().cloned().collect()
    }
    fn symbol_pnl(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.pnl())
            .unwrap_or_default()
    }
    fn symbol_margin(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.margin())
            .unwrap_or_default()
    }
}

impl EngineMarket for Backtest {
    fn price_mark(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.market.mark)
            .unwrap_or_default()
    }

    fn price_last(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.market.last)
            .unwrap_or_default()
    }

    fn price_index(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.market.index)
            .unwrap_or_default()
    }

    fn price_settlement(&self, symbol: &str) -> Decimal {
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

    fn rule_price_min(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.rule.price_min)
            .unwrap_or_default()
    }

    fn rule_price_max(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.rule.price_max)
            .unwrap_or_default()
    }

    fn rule_price_tick(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.rule.price_tick)
            .unwrap_or_default()
    }

    fn rule_size_min(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.rule.size_min)
            .unwrap_or_default()
    }

    fn rule_size_max(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.rule.size_max)
            .unwrap_or_default()
    }

    fn rule_size_tick(&self, symbol: &str) -> Decimal {
        self.account
            .lock()
            .positions
            .get(symbol)
            .map(|position| position.symbol.rule.size_tick)
            .unwrap_or_default()
    }

    fn rule_amount_min(&self, symbol: &str) -> Decimal {
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
    async fn long_market_open(&self, symbol: &str, size: Decimal) -> Result<String> {
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
        let leverage = Decimal::from(self.leverage(symbol));
        let cash_available = self.cash_available();

        let size = size - size % size_tick;
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

        let amount = size * price_mark;
        ensure!(
            amount >= amount_min,
            "最小金额限制: 金额({}),限制({})",
            amount,
            amount_min,
        );

        let margin = amount / leverage;
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
                    price: Decimal::ZERO,
                    size,
                    deal_price: Decimal::ZERO,
                    deal_size: Decimal::ZERO,
                    deal_fee: Decimal::ZERO,
                    create_time: self.time(),
                },
            );
        Ok(order_id)
    }
    async fn long_limit_open(&self, symbol: &str, size: Decimal, price: Decimal) -> Result<String> {
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
        let leverage = Decimal::from(self.leverage(symbol));
        let cash_available = self.cash_available();

        let size = size - size % size_tick;
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

        let price = price - price % price_tick;
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

        let amount = size * price;
        ensure!(
            amount >= amount_min,
            "最小金额限制: 金额({}),限制({})",
            amount,
            amount_min
        );

        let margin = amount / leverage;
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
                    deal_price: Decimal::ZERO,
                    deal_size: Decimal::ZERO,
                    deal_fee: Decimal::ZERO,
                    create_time: self.time(),
                },
            );
        Ok(order_id)
    }
    async fn long_market_close(&self, symbol: &str, size: Decimal) -> Result<String> {
        ensure!(
            self.account.lock().positions.contains_key(symbol),
            "交易对不存在: {}",
            symbol
        );

        let size_min = self.rule_size_min(symbol);
        let size_tick = self.rule_size_tick(symbol);
        let long_size_available = self.long_size_available(symbol);

        let size = size - size % size_tick;
        ensure!(
            size >= size_min,
            "最小数量限制: 数量({}),限制({})",
            size,
            size_min
        );

        ensure!(
            long_size_available >= size,
            "持仓数量不足: 数量({}),可用({})",
            size,
            long_size_available,
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
                    price: Decimal::ZERO,
                    size,
                    deal_price: Decimal::ZERO,
                    deal_size: Decimal::ZERO,
                    deal_fee: Decimal::ZERO,
                    create_time: self.time(),
                },
            );
        Ok(order_id)
    }
    async fn long_limit_close(
        &self,
        symbol: &str,
        size: Decimal,
        price: Decimal,
    ) -> Result<String> {
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

        let size = size - size % size_tick;
        ensure!(
            size >= size_min,
            "最小数量限制: 数量({}),限制({})",
            size,
            size_min
        );

        let price = price - price % price_tick;
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
            "持仓数量不足: 数量({}),可用({})",
            size,
            long_size_available,
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
                    deal_price: Decimal::ZERO,
                    deal_size: Decimal::ZERO,
                    deal_fee: Decimal::ZERO,
                    create_time: self.time(),
                },
            );
        Ok(order_id)
    }
    async fn short_market_open(&self, symbol: &str, size: Decimal) -> Result<String> {
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
        let leverage = Decimal::from(self.leverage(symbol));
        let cash_available = self.cash_available();

        let size = size - size % size_tick;
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

        let amount = size * price_mark;
        ensure!(
            amount >= amount_min,
            "最小金额限制: 金额({}),限制({})",
            amount,
            amount_min
        );

        let margin = amount / leverage;
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
                    price: Decimal::ZERO,
                    size,
                    deal_price: Decimal::ZERO,
                    deal_size: Decimal::ZERO,
                    deal_fee: Decimal::ZERO,
                    create_time: self.time(),
                },
            );
        Ok(order_id)
    }
    async fn short_limit_open(
        &self,
        symbol: &str,
        size: Decimal,
        price: Decimal,
    ) -> Result<String> {
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
        let leverage = Decimal::from(self.leverage(symbol));
        let cash_available = self.cash_available();

        let size = size - size % size_tick;
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

        let price = price - price % price_tick;
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

        let amount = size * price;
        ensure!(
            amount >= amount_min,
            "最小金额限制: 金额({}),限制({})",
            amount,
            amount_min
        );

        let margin = amount / leverage;
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
                    deal_price: Decimal::ZERO,
                    deal_size: Decimal::ZERO,
                    deal_fee: Decimal::ZERO,
                    create_time: self.time(),
                },
            );
        Ok(order_id)
    }
    async fn short_market_close(&self, symbol: &str, size: Decimal) -> Result<String> {
        ensure!(
            self.account.lock().positions.contains_key(symbol),
            "交易对不存在: {}",
            symbol
        );

        let size_tick = self.rule_size_tick(symbol);
        let size_min = self.rule_size_min(symbol);
        let short_size_available = self.short_size_available(symbol);

        let size = size - size % size_tick;
        ensure!(
            size >= size_min,
            "最小数量限制: 数量({}),限制({})",
            size,
            size_min
        );

        ensure!(
            short_size_available >= size,
            "持仓数量不足: 数量({}),可用({})",
            size,
            short_size_available
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
                    price: Decimal::ZERO,
                    size,
                    deal_price: Decimal::ZERO,
                    deal_size: Decimal::ZERO,
                    deal_fee: Decimal::ZERO,
                    create_time: self.time(),
                },
            );
        Ok(order_id)
    }
    async fn short_limit_close(
        &self,
        symbol: &str,
        size: Decimal,
        price: Decimal,
    ) -> Result<String> {
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

        let size = size - size % size_tick;
        ensure!(
            size >= size_min,
            "最小数量限制: 数量({}),限制({})",
            size,
            size_min
        );

        let price = price - price % price_tick;
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
            "持仓数量不足: 数量({}),可用({})",
            size,
            short_size_available
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
                    deal_price: Decimal::ZERO,
                    deal_size: Decimal::ZERO,
                    deal_fee: Decimal::ZERO,
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
        backtest: &Arc<Backtest>,
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
        backtest: &Arc<Backtest>,
        symbol: &str,
        date: DateTime<Utc>,
    ) -> Result<()> {
        let history_data = self.0.get_mut(symbol).unwrap();
        if let Some(mut kline) = history_data
            .klines
            .take(date - Duration::minutes(1))
            .await?
        {
            backtest
                .account
                .lock()
                .positions
                .get_mut(symbol)
                .unwrap()
                .symbol
                .market
                .last = kline.close;

            backtest.cross_order(symbol).await?;

            kline.symbol = symbol.to_owned();
            backtest.strategy.on_kline(backtest.clone(), kline).await?;
        }

        Ok(())
    }

    async fn flush_market_index_price(
        &mut self,
        backtest: &Arc<Backtest>,
        symbol: &str,
        date: DateTime<Utc>,
    ) -> Result<()> {
        let history_data = self.0.get_mut(symbol).unwrap();
        if let Some(kline) = history_data
            .index_price_klines
            .take(date - Duration::minutes(1))
            .await?
        {
            let mut account = backtest.account.lock();
            let symbol_position = account.positions.get_mut(symbol).unwrap();
            symbol_position.symbol.market.index = kline.close;
        }
        Ok(())
    }

    async fn flush_market_mark_price(
        &mut self,
        backtest: &Arc<Backtest>,
        symbol: &str,
        date: DateTime<Utc>,
    ) -> Result<()> {
        let history_data = self.0.get_mut(symbol).unwrap();
        if let Some(kline) = history_data
            .mark_price_klines
            .take(date - Duration::minutes(1))
            .await?
        {
            backtest
                .account
                .lock()
                .positions
                .get_mut(symbol)
                .unwrap()
                .symbol
                .market
                .mark = kline.close;
        }
        Ok(())
    }

    pub async fn flush_market(
        &mut self,
        backtest: &Arc<Backtest>,
        symbols: &[String],
        date: DateTime<Utc>,
    ) -> Result<()> {
        for symbol in symbols {
            self.flush_market_settlement_price(backtest, symbol, date)
                .await?;
            self.flush_market_mark_price(backtest, symbol, date).await?;
            self.flush_market_index_price(backtest, symbol, date)
                .await?;
            self.flush_market_last_price(backtest, symbol, date).await?;
        }
        Ok(())
    }
}

impl Backtest {
    fn new(mut config: BacktestConfig, strategy: Arc<dyn Strategy>) -> Result<Arc<Backtest>> {
        config.begin = config.begin.truncate_minute()?;
        config.end = config.end.truncate_minute()?;
        ensure!(config.begin < config.end, "开始时间必须小于结束时间");

        ensure!(config.cash >= Decimal::ZERO, "初始资金必须大于等于0");

        ensure!(config.slippage_rate >= Decimal::ZERO, "滑点率必须大于等于0");

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
            strategy,
        }))
    }

    pub async fn run(config: BacktestConfig, strategy: Arc<dyn Strategy>) -> Result<()> {
        Self::new(config, strategy)?.run0().await?;
        Ok(())
    }
}

impl Backtest {
    pub async fn run0(self: &Arc<Self>) -> Result<()> {
        self.strategy.on_init(self.clone()).await?;

        ensure!(!self.account.lock().positions.is_empty(), "未初始化交易对");

        let symbols = self
            .account
            .lock()
            .positions
            .keys()
            .cloned()
            .collect::<Vec<String>>();

        self.strategy.on_start(self.clone()).await?;

        let mut begin = self.config.begin;
        let end = self.config.end;

        let mut symbol_history_data = SymbolHistoryData::new(&symbols, begin, end);

        let backtest_instant = Instant::now();

        while begin <= end {
            *self.trade_time.lock() = begin;

            symbol_history_data
                .flush_market(self, &symbols, begin)
                .await?;

            self.on_daily(begin).await?;
            self.on_hourly(begin).await?;
            self.on_minutely(begin).await?;

            begin += Duration::minutes(1);
        }

        tracing::info!("回测耗时: {:?}", backtest_instant.elapsed());

        self.on_stop().await?;

        Ok(())
    }
}

impl Backtest {
    async fn on_daily(self: &Arc<Self>, time: DateTime<Utc>) -> Result<()> {
        if !(time.hour() == 0 && time.minute() == 0) {
            return Ok(());
        }
        let instant = Instant::now();
        if let Err(err) = self.strategy.on_daily(self.clone()).await {
            tracing::error!("{} 每日事件执行失败: {}", time.str_ymd_hm(), err);
        } else {
            tracing::debug!(
                "{} 每日事件执行耗时: {:?}",
                time.str_ymd_hm(),
                instant.elapsed()
            );
        }
        Ok(())
    }

    async fn on_hourly(self: &Arc<Self>, time: DateTime<Utc>) -> Result<()> {
        if time.minute() != 0 {
            return Ok(());
        }
        let instant = Instant::now();
        if let Err(err) = self.strategy.on_hourly(self.clone()).await {
            tracing::error!("{} 每小时事件执行失败: {}", time.str_ymd_hm(), err);
        } else {
            tracing::debug!(
                "{} 每小时事件执行耗时: {:?}",
                time.str_ymd_hm(),
                instant.elapsed()
            );
        }
        Ok(())
    }

    async fn on_order(self: &Arc<Self>, order: Order) -> Result<()> {
        let instant = Instant::now();
        if let Err(err) = self.strategy.on_order(self.clone(), order).await {
            tracing::error!("{} 订单事件失败: {}", self.time().str_ymd_hm(), err);
        } else {
            tracing::debug!(
                "{} 订单事件执行耗时: {:?}",
                self.time().str_ymd_hm(),
                instant.elapsed()
            );
        }
        Ok(())
    }

    async fn on_minutely(self: &Arc<Self>, time: DateTime<Utc>) -> Result<()> {
        let instant = Instant::now();
        if let Err(err) = self.strategy.on_minutely(self.clone()).await {
            tracing::error!("{} 每分钟事件执行失败: {}", time.str_ymd_hm(), err);
        } else {
            tracing::debug!(
                "{} 每分钟事件执行耗时: {:?}",
                time.str_ymd_hm(),
                instant.elapsed()
            );
        }
        Ok(())
    }

    async fn on_stop(self: &Arc<Self>) -> Result<()> {
        self.strategy.on_stop(self.clone()).await?;
        Ok(())
    }

    async fn cross_order(self: &Arc<Self>, symbol: &str) -> Result<()> {
        let orders = {
            let price_last = self.price_last(symbol);
            let mut account = self.account.lock();
            let positions = account.positions.get_mut(symbol).unwrap();
            let mut fee = Decimal::ZERO;
            let mut profit = Decimal::ZERO;

            let cross_order_ids = positions
                .orders
                .par_iter_mut()
                .filter_map(|(id, order)| {
                    let corss = match order.r#type {
                        TradeType::Limit => match order.side {
                            TradeSide::Long => {
                                if order.reduce_only {
                                    order.price <= price_last
                                } else {
                                    order.price >= price_last
                                }
                            }
                            TradeSide::Short => {
                                if order.reduce_only {
                                    order.price >= price_last
                                } else {
                                    order.price <= price_last
                                }
                            }
                        },
                        TradeType::Market => true,
                    };
                    if corss {
                        Some(id.to_owned())
                    } else {
                        if order.status == OrderStatus::Created {
                            order.status = OrderStatus::Submitted;
                        }
                        None
                    }
                })
                .collect::<Vec<String>>();

            let mut cross_orders = Vec::with_capacity(cross_order_ids.len());
            for id in cross_order_ids {
                let mut order = positions.orders.remove(&id).unwrap();

                order.deal_fee = order.size
                    * price_last
                    * if order.status == OrderStatus::Created {
                        self.config.fee_rate_taker
                    } else {
                        self.config.fee_rate_maker
                    };
                fee += order.deal_fee;

                if order.reduce_only {
                    match order.side {
                        TradeSide::Long => {
                            profit += (price_last - positions.long.price) * order.size;
                            positions.long.size -= order.size;
                            if positions.long.size == Decimal::ZERO {
                                positions.long.price = Decimal::ZERO;
                            }
                        }
                        TradeSide::Short => {
                            profit += (positions.short.price - price_last) * order.size;
                            positions.short.size -= order.size;
                            if positions.short.size == Decimal::ZERO {
                                positions.short.price = Decimal::ZERO;
                            }
                        }
                    };
                } else {
                    match order.side {
                        TradeSide::Long => {
                            positions.long.price = (order.price * order.size
                                + positions.long.price * positions.long.size)
                                / (order.size + positions.long.size);
                            positions.long.size += order.size;
                        }
                        TradeSide::Short => {
                            positions.short.price = (order.price * order.size
                                + positions.short.price * positions.short.size)
                                / (order.size + positions.short.size);
                            positions.short.size += order.size;
                        }
                    }
                }

                order.status = OrderStatus::Filled;
                order.deal_price = price_last;
                order.deal_size = order.size;

                cross_orders.push(order);
            }

            account.cash -= fee;
            account.cash += profit;

            cross_orders
        };

        for order in orders {
            self.on_order(order).await?;
        }

        Ok(())
    }
}
