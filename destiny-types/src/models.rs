use crate::enums::*;
use chrono::{DateTime, NaiveDate, Utc};
use std::collections::HashMap;

/// 订单
#[derive(Debug, Clone)]
pub struct Order {
    /// 订单ID
    pub id: String,
    /// 交易所订单ID
    pub eid: String,
    /// 产品类型
    pub product: ProductType,
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
    /// 数量
    pub volume: f64,
    /// 价格
    pub price: f64,
    /// 成交数量
    pub deal_volume: f64,
    /// 成交价格
    pub deal_price: f64,
    /// 成交手续费
    pub deal_fee: f64,
    /// 创建时间
    pub create_time: DateTime<Utc>,
}

/// 现货资产
#[derive(Debug, Clone)]
pub struct SpotAsset {
    /// 资产名
    pub name: String,
    /// 资产数量
    pub total: f64,
    /// 可用数量
    pub available: f64,
    /// 冻结数量
    pub frozen: f64,
}

/// 合约持仓
#[derive(Debug, Clone)]
pub struct ContractPosition {
    /// 交易对
    pub symbol: String,
    /// 方向
    pub side: TradeSide,
    /// 持仓
    pub total: f64,
    /// 可用持仓
    pub available: f64,
    /// 冻结持仓
    pub frozen: f64,
    /// 持仓均价
    pub avg_price: f64,
    /// 杠杆
    pub leverage: i32,
}

/// 账户
#[derive(Debug, Clone)]
pub struct Account {
    /// 现货资产
    pub spot_assets: HashMap<String, SpotAsset>,
    /// 合约持仓
    pub contract_positions: HashMap<String, ContractPosition>,
    /// 合约保证金
    pub contract_margin: f64,
    /// 合约可用保证金
    pub contract_available_margin: f64,
    /// 合约冻结保证金
    pub contract_frozen_margin: f64,
    /// 订单
    pub orders: HashMap<String, Order>,
}

/// 文件元数据
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MarketFileMeta {
    /// ID
    pub id: i64,
    /// 交易对
    pub symbol: String,
    /// 日期
    pub day: NaiveDate,
    /// 小时
    pub hour: i32,
    /// 路径
    pub path: String,
    /// 更新时间
    pub update_time: DateTime<Utc>,
    /// 本地时间
    pub local_time: Option<DateTime<Utc>>,
}
