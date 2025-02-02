use crate::enums::*;
use chrono::{DateTime, Utc};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rust_decimal::Decimal;
use std::collections::HashMap;

/// 交易对
#[derive(Debug, Clone)]
pub struct Symbol {
    /// 交易对
    pub symbol: String,
    /// 是否可用
    pub enable: bool,
    /// 规则
    pub rule: SymbolRule,
    /// 行情
    pub market: SymbolMarket,
}

/// 交易对规则
#[derive(Debug, Clone)]
pub struct SymbolRule {
    /// 最小价格
    pub price_min: Decimal,
    /// 最大价格
    pub price_max: Decimal,
    /// 价格步长
    pub price_tick: Decimal,
    /// 最小数量
    pub size_min: Decimal,
    /// 最大数量
    pub size_max: Decimal,
    /// 数量步长
    pub size_tick: Decimal,
    /// 最小下单金额
    pub amount_min: Decimal,
    /// 最大订单数量
    pub order_max: i64,
}

/// 交易对行情
#[derive(Debug, Clone)]
pub struct SymbolMarket {
    /// 标记价格
    pub mark: Decimal,
    /// 指数价格
    pub index: Decimal,
    /// 最新价格
    pub last: Decimal,
    /// 结算价格
    pub settlement: Decimal,
    /// 下次结算时间
    pub settlement_time: DateTime<Utc>,
    /// 时间
    pub time: DateTime<Utc>,
}

/// K线
#[cfg_attr(feature = "python", pyo3::pyclass(get_all))]
#[derive(Debug, Clone)]
pub struct Kline {
    /// 交易对
    pub symbol: String,
    /// 开盘时间
    pub open_time: DateTime<Utc>,
    /// 开盘价
    pub open: Decimal,
    /// 最高价
    pub high: Decimal,
    /// 最低价
    pub low: Decimal,
    /// 收盘价
    pub close: Decimal,
    /// 成交量
    pub size: Decimal,
    /// 成交额
    pub cash: Decimal,
    /// 买方成交量
    pub buy_size: Decimal,
    /// 买方成交额
    pub buy_cash: Decimal,
    /// 交易笔数
    pub trades: i64,
}

/// 深度信息
#[derive(Debug, Clone)]
pub struct Depth {
    /// 交易对
    pub symbol: String,
    /// 买盘
    pub buys: Vec<DepthLevel>,
    /// 卖盘
    pub sells: Vec<DepthLevel>,
    /// 时间
    pub time: DateTime<Utc>,
}

/// 深度档位
#[derive(Debug, Clone)]
pub struct DepthLevel {
    /// 价格
    pub price: Decimal,
    /// 数量
    pub size: Decimal,
}

/// 成交记录
#[derive(Debug, Clone)]
pub struct AggTrades {
    /// 交易对
    pub symbol: String,
    /// 成交
    pub trades: Vec<AggTrade>,
}

/// 成交记录
#[derive(Debug, Clone)]
pub struct AggTrade {
    /// ID
    pub id: i64,
    /// 价格
    pub price: Decimal,
    /// 数量
    pub size: Decimal,
    /// 金额
    pub cash: Decimal,
    /// 是否为主动买入
    pub is_buy: bool,
    /// 时间
    pub time: DateTime<Utc>,
}

/// 历史资金费率
#[derive(Debug, Clone)]
pub struct FundingRateHistory {
    /// 交易对
    pub symbol: String,
    /// 标记价格
    pub mark_price: Decimal,
    /// 资金费率
    pub rate: Decimal,
    /// 时间
    pub time: DateTime<Utc>,
}

/// 订单
#[cfg_attr(feature = "python", pyo3::pyclass(get_all))]
#[derive(Debug, Clone)]
pub struct Order {
    /// ID
    pub id: String,
    /// 交易对
    pub symbol: String,
    /// 交易类型
    pub r#type: TradeType,
    /// 交易方向
    pub side: TradeSide,
    /// 开仓订单
    pub reduce_only: bool,
    /// 订单状态
    pub status: OrderStatus,
    /// 价格
    pub price: Decimal,
    /// 数量
    pub size: Decimal,
    /// 成交价格
    pub deal_price: Decimal,
    /// 成交数量
    pub deal_size: Decimal,
    /// 成交手续费
    pub deal_fee: Decimal,
    /// 创建时间
    pub create_time: DateTime<Utc>,
}

impl Order {
    pub fn margin(&self, mark_price: Decimal, leverage: u32) -> Decimal {
        if self.reduce_only {
            return Decimal::ZERO;
        }
        (self.size - self.deal_size)
            * if self.r#type == TradeType::Limit {
                self.price
            } else {
                mark_price
            }
            / Decimal::from(leverage)
    }
}

/// 持仓
#[derive(Debug, Clone)]
pub struct Position {
    /// 方向
    pub side: TradeSide,
    /// 持仓均价
    pub price: Decimal,
    /// 持仓
    pub size: Decimal,
}

impl Position {
    pub fn margin(&self, leverage: u32) -> Decimal {
        self.size * self.price / Decimal::from(leverage)
    }

    pub fn pnl(&self, mark_price: Decimal) -> Decimal {
        if self.size.is_zero() {
            return Decimal::ZERO;
        }
        (mark_price - self.price)
            * self.size
            * match self.side {
                TradeSide::Long => Decimal::ONE,
                TradeSide::Short => Decimal::NEGATIVE_ONE,
            }
    }
}

/// 持仓
#[derive(Debug, Clone)]
pub struct SymbolPosition {
    /// 交易对
    pub symbol: Symbol,
    /// 杠杆倍率
    pub leverage: u32,
    /// 做多持仓
    pub long: Position,
    /// 做空持仓
    pub short: Position,
    /// 订单
    pub orders: HashMap<String, Order>,
}

impl SymbolPosition {
    pub fn margin_orders(&self) -> Decimal {
        self.orders
            .par_iter()
            .map(|(_, order)| order.margin(self.symbol.market.mark, self.leverage))
            .sum::<Decimal>()
    }

    pub fn margin_long(&self) -> Decimal {
        self.long.margin(self.leverage)
    }

    pub fn margin_short(&self) -> Decimal {
        self.short.margin(self.leverage)
    }

    pub fn margin_positions(&self) -> Decimal {
        self.margin_long() + self.margin_short()
    }

    pub fn margin(&self) -> Decimal {
        self.margin_orders() + self.margin_positions()
    }

    pub fn long_pnl(&self) -> Decimal {
        self.long.pnl(self.symbol.market.mark)
    }

    pub fn short_pnl(&self) -> Decimal {
        self.short.pnl(self.symbol.market.mark)
    }

    pub fn pnl(&self) -> Decimal {
        self.long_pnl() + self.short_pnl()
    }

    pub fn long_size_frozen(&self) -> Decimal {
        self.orders
            .par_iter()
            .filter_map(|(_, order)| {
                if order.side == TradeSide::Long && order.reduce_only {
                    Some(order)
                } else {
                    None
                }
            })
            .map(|order| order.size - order.deal_size)
            .sum::<Decimal>()
    }

    pub fn long_size_available(&self) -> Decimal {
        self.long.size - self.long_size_frozen()
    }

    pub fn short_size_frozen(&self) -> Decimal {
        self.orders
            .par_iter()
            .filter_map(|(_, order)| {
                if order.side == TradeSide::Short && order.reduce_only {
                    Some(order)
                } else {
                    None
                }
            })
            .map(|order| order.size - order.deal_size)
            .sum::<Decimal>()
    }

    pub fn short_size_available(&self) -> Decimal {
        self.short.size - self.short_size_frozen()
    }
}

/// 账户
#[derive(Debug, Clone)]
pub struct Account {
    /// 资金
    pub cash: Decimal,
    /// 持仓
    pub positions: HashMap<String, SymbolPosition>,
}

impl Account {
    pub fn margin(&self) -> Decimal {
        self.positions
            .par_iter()
            .map(|(_, pos)| pos.margin())
            .sum::<Decimal>()
    }

    pub fn pnl(&self) -> Decimal {
        self.positions
            .par_iter()
            .map(|(_, pos)| pos.pnl())
            .sum::<Decimal>()
    }

    pub fn cash_frozen(&self) -> Decimal {
        self.margin()
    }

    pub fn cash_available(&self) -> Decimal {
        self.cash - self.cash_frozen() + self.pnl()
    }
}
