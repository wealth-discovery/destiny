use crate::enums::*;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Candle {
    pub product: ProductType,
    pub code: String,
    pub interval: CandleInterval,
    pub open_time: DateTime<Utc>,
    pub close_time: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub amount: f64,
    pub taker_volume: f64,
    pub taker_amount: f64,
    pub trades: i64,
    pub update_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Orderbook {
    pub buys: Vec<(f64, f64)>,
    pub sells: Vec<(f64, f64)>,
    pub update_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Price {
    pub value: f64,
    pub update_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct TradeFeeRate {
    pub taker: f64,
    pub maker: f64,
}

#[derive(Debug, Clone)]
pub struct TradeRule {
    pub min_volume: f64,
    pub min_price: f64,
    pub min_amount: f64,
}

#[derive(Debug, Clone)]
pub struct Pair {
    pub product: ProductType,
    pub code: String,
    pub trade_rule: TradeRule,
    pub trade_fee_rate: TradeFeeRate,
    pub mark_price: Price,
    pub index_price: Price,
    pub last_price: Price,
    pub funding_rate: Price,
    pub orderbook: Orderbook,
    pub candles: HashMap<CandleInterval, Vec<Candle>>,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: String,
    pub eid: String,
    pub product: ProductType,
    pub code: String,
    pub type_: TradeType,
    pub side: TradeSide,
    pub reduce_only: bool,
    pub status: OrderStatus,
    pub volume: f64,
    pub price: f64,
    pub deal_volume: f64,
    pub deal_price: f64,
    pub deal_fee: f64,
    pub create_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub product: ProductType,
    pub code: String,
    pub side: TradeSide,
    pub volume: f64,
    pub price: f64,
    pub leverage: i32,
}

#[derive(Debug, Clone)]
pub struct Account {
    pub positions: Vec<Position>,
    pub orders: HashMap<String, Order>,
}

#[derive(Debug, Clone)]
pub struct BacktestConfig {
    pub spot_balance: f64,
    pub contract_balance: f64,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub trade_fee_rate: TradeFeeRate,
    pub trade_slippage_rate: f64,
}

#[derive(Debug, Clone)]
pub struct TestnetConfig {
    pub api_key: String,
    pub api_secret: String,
    pub api_passphrase: String,
}

#[derive(Debug, Clone)]
pub struct MainnetConfig {
    pub api_key: String,
    pub api_secret: String,
    pub api_passphrase: String,
}
