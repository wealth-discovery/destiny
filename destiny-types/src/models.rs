use crate::enums::*;
use chrono::{DateTime, Utc};
use destiny_helpers::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
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
    pub price_min: f64,
    /// 最大价格
    pub price_max: f64,
    /// 价格步长
    pub price_tick: f64,
    /// 最小数量
    pub size_min: f64,
    /// 最大数量
    pub size_max: f64,
    /// 数量步长
    pub size_tick: f64,
    /// 最小下单金额
    pub amount_min: f64,
    /// 最大订单数量
    pub order_max: i64,
}

/// 交易对行情
#[derive(Debug, Clone)]
pub struct SymbolMarket {
    /// 标记价格
    pub mark: f64,
    /// 指数价格
    pub index: f64,
    /// 最新价格
    pub last: f64,
    /// 结算价格
    pub settlement: f64,
    /// 下次结算时间
    pub settlement_time: DateTime<Utc>,
    /// 时间
    pub time: DateTime<Utc>,
}

/// K线
#[derive(Debug, Clone)]
pub struct Kline {
    /// 交易对
    pub symbol: String,
    /// 周期
    pub interval: KlineInterval,
    /// 开盘时间
    pub open_time: DateTime<Utc>,
    /// 开盘价
    pub open: f64,
    /// 最高价
    pub high: f64,
    /// 最低价
    pub low: f64,
    /// 收盘价
    pub close: f64,
    /// 成交量
    pub size: f64,
    /// 成交额
    pub cash: f64,
    /// 买方成交量
    pub buy_size: f64,
    /// 买方成交额
    pub buy_cash: f64,
    /// 交易笔数
    pub trades: i64,
    /// 时间
    pub time: DateTime<Utc>,
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
    pub price: f64,
    /// 数量
    pub size: f64,
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
    pub price: f64,
    /// 数量
    pub size: f64,
    /// 金额
    pub cash: f64,
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
    pub mark_price: f64,
    /// 资金费率
    pub rate: f64,
    /// 时间
    pub time: DateTime<Utc>,
}

/// 订单
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
    /// 只减仓
    pub reduce_only: bool,
    /// 订单状态
    pub status: OrderStatus,
    /// 价格
    pub price: f64,
    /// 数量
    pub size: f64,
    /// 成交价格
    pub deal_price: f64,
    /// 成交数量
    pub deal_size: f64,
    /// 成交手续费
    pub deal_fee: f64,
    /// 创建时间
    pub create_time: DateTime<Utc>,
}

impl Order {
    pub fn margin(&self, mark_price: f64, leverage: u32) -> f64 {
        if self.reduce_only {
            return 0.;
        }
        ((self.size - self.deal_size) * mark_price / leverage as f64).to_safe()
    }
}

/// 持仓
#[derive(Debug, Clone)]
pub struct Position {
    /// 方向
    pub side: TradeSide,
    /// 持仓均价
    pub price: f64,
    /// 持仓
    pub size: f64,
}

impl Position {
    pub fn margin(&self, mark_price: f64, leverage: u32) -> f64 {
        (self.size * mark_price / leverage as f64).to_safe()
    }

    pub fn pnl(&self, mark_price: f64) -> f64 {
        if self.size.is_zero() {
            return 0.;
        }
        ((mark_price - self.price)
            * self.size
            * match self.side {
                TradeSide::Long => 1.,
                TradeSide::Short => -1.,
            })
        .to_safe()
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
    pub fn margin_orders(&self) -> f64 {
        self.orders
            .par_iter()
            .map(|(_, order)| order.margin(self.symbol.market.mark, self.leverage))
            .sum::<f64>()
            .to_safe()
    }

    pub fn margin_long(&self) -> f64 {
        self.long.margin(self.symbol.market.mark, self.leverage)
    }

    pub fn margin_short(&self) -> f64 {
        self.short.margin(self.symbol.market.mark, self.leverage)
    }

    pub fn margin_positions(&self) -> f64 {
        (self.margin_long() + self.margin_short()).to_safe()
    }

    pub fn margin(&self) -> f64 {
        (self.margin_orders() + self.margin_positions()).to_safe()
    }

    pub fn long_pnl(&self) -> f64 {
        self.long.pnl(self.symbol.market.mark)
    }

    pub fn short_pnl(&self) -> f64 {
        self.short.pnl(self.symbol.market.mark)
    }

    pub fn pnl(&self) -> f64 {
        (self.long_pnl() + self.short_pnl()).to_safe()
    }

    pub fn long_size_frozen(&self) -> f64 {
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
            .sum::<f64>()
            .to_safe()
    }

    pub fn long_size_available(&self) -> f64 {
        (self.long.size - self.long_size_frozen()).to_safe()
    }

    pub fn short_size_frozen(&self) -> f64 {
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
            .sum::<f64>()
            .to_safe()
    }

    pub fn short_size_available(&self) -> f64 {
        (self.short.size - self.short_size_frozen()).to_safe()
    }
}

/// 账户
#[derive(Debug, Clone)]
pub struct Account {
    /// 资金
    pub cash: f64,
    /// 持仓
    pub positions: HashMap<String, SymbolPosition>,
}

impl Account {
    pub fn margin(&self) -> f64 {
        self.positions
            .par_iter()
            .map(|(_, pos)| pos.margin())
            .sum::<f64>()
            .to_safe()
    }

    pub fn pnl(&self) -> f64 {
        self.positions
            .par_iter()
            .map(|(_, pos)| pos.pnl())
            .sum::<f64>()
            .to_safe()
    }

    pub fn cash_frozen(&self) -> f64 {
        self.margin()
    }

    pub fn cash_available(&self) -> f64 {
        (self.cash - self.cash_frozen() + self.pnl()).to_safe()
    }
}
