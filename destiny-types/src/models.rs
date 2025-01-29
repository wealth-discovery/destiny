use crate::enums::*;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// 订单
#[derive(Debug, Clone)]
pub struct Order {
    /// ID
    pub id: String,
    /// 交易对
    pub symbol: String,
    /// 交易类型
    pub type_: TradeType,
    /// 交易方向
    pub side: TradeSide,
    /// 是否只减仓
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

/// 持仓
#[derive(Debug, Clone)]
pub struct Position {
    /// 方向
    pub side: TradeSide,
    /// 持仓均价
    pub price: f64,
    /// 持仓
    pub size: f64,
    /// 可用持仓
    pub available: f64,
    /// 冻结持仓
    pub frozen: f64,
}

/// 持仓
#[derive(Debug, Clone)]
pub struct SymbolPosition {
    /// 交易对
    pub symbol: String,
    /// 杠杆倍率
    pub leverage: i32,
    /// 做多持仓
    pub long: Position,
    /// 做空持仓
    pub short: Position,
}

/// 保证金
#[derive(Debug, Clone)]
pub struct Cash {
    /// 保证金
    pub size: f64,
    /// 可用保证金
    pub available: f64,
    /// 冻结保证金
    pub frozen: f64,
}

/// 账户
#[derive(Debug, Clone)]
pub struct Account {
    /// 保证金
    pub cash: Cash,
    /// 交易对
    pub symbols: HashMap<String, Symbol>,
    /// 持仓
    pub positions: HashMap<String, SymbolPosition>,
    /// 订单
    pub orders: HashMap<String, Order>,
}

/// 交易对
#[derive(Debug, Clone)]
pub struct Symbol {
    /// 交易对
    pub symbol: String,
    /// 规则
    pub rule: SymbolRule,
    /// 指数
    pub index: SymbolIndex,
}

/// 交易对规则
#[derive(Debug, Clone)]
pub struct SymbolRule {
    /// 是否可用
    pub enable: bool,
    /// 最小价格
    pub price_min: f64,
    /// 最大价格
    pub price_max: f64,
    /// 价格刻度
    pub price_tick: f64,
    /// 最小数量
    pub size_min: f64,
    /// 最大数量
    pub size_max: f64,
    /// 数量刻度
    pub size_tick: f64,
    /// 最小下单金额
    pub cash_min: f64,
    /// 最大订单数量
    pub order_max: i64,
}

/// 交易对指数
#[derive(Debug, Clone)]
pub struct SymbolIndex {
    /// 标记价格
    pub mark_price: f64,
    /// 指数价格
    pub index_price: f64,
    /// 最新价格
    pub last_price: f64,
    /// 结算价格
    pub settlement_price: f64,
    /// 下次结算时间
    pub next_settlement_time: DateTime<Utc>,
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
