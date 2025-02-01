use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

/// 运行模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Display)]
pub enum RunMode {
    /// 回测
    #[serde(rename = "backtest")]
    #[strum(serialize = "backtest")]
    Backtest,
    /// 测试网
    #[serde(rename = "testnet")]
    #[strum(serialize = "testnet")]
    Testnet,
    /// 主网
    #[serde(rename = "mainnet")]
    #[strum(serialize = "mainnet")]
    Mainnet,
}

/// 交易类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Display)]
pub enum TradeType {
    /// 限价
    #[serde(rename = "limit")]
    #[strum(serialize = "limit")]
    Limit,
    /// 市价
    #[serde(rename = "market")]
    #[strum(serialize = "market")]
    Market,
}

/// 交易方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Display)]
pub enum TradeSide {
    /// 做多
    #[serde(rename = "long")]
    #[strum(serialize = "long")]
    Long,
    /// 做空
    #[serde(rename = "short")]
    #[strum(serialize = "short")]
    Short,
}

/// K线周期
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Display, EnumIter)]
pub enum KlineInterval {
    /// 1分钟
    #[serde(rename = "1m")]
    #[strum(serialize = "1m")]
    M1,
    /// 3分钟
    #[serde(rename = "3m")]
    #[strum(serialize = "3m")]
    M3,
    /// 5分钟
    #[serde(rename = "5m")]
    #[strum(serialize = "5m")]
    M5,
    /// 15分钟
    #[serde(rename = "15m")]
    #[strum(serialize = "15m")]
    M15,
    /// 30分钟
    #[serde(rename = "30m")]
    #[strum(serialize = "30m")]
    M30,
    /// 1小时
    #[serde(rename = "1h")]
    #[strum(serialize = "1h")]
    H1,
    /// 2小时
    #[serde(rename = "2h")]
    #[strum(serialize = "2h")]
    H2,
    /// 4小时
    #[serde(rename = "4h")]
    #[strum(serialize = "4h")]
    H4,
    /// 6小时
    #[serde(rename = "6h")]
    #[strum(serialize = "6h")]
    H6,
    /// 8小时
    #[serde(rename = "8h")]
    #[strum(serialize = "8h")]
    H8,
    /// 12小时
    #[serde(rename = "12h")]
    #[strum(serialize = "12h")]
    H12,
    /// 1天
    #[serde(rename = "1d")]
    #[strum(serialize = "1d")]
    D1,
    /// 3天
    #[serde(rename = "3d")]
    #[strum(serialize = "3d")]
    D3,
    /// 1周
    #[serde(rename = "1w")]
    #[strum(serialize = "1w")]
    W1,
    /// 1月
    #[serde(rename = "1mo")]
    #[strum(serialize = "1mo")]
    Mo1,
}

/// 订单状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Display)]
pub enum OrderStatus {
    /// 已创建
    #[serde(rename = "created")]
    #[strum(serialize = "created")]
    Created,
    /// 已提交
    #[serde(rename = "submitted")]
    #[strum(serialize = "submitted")]
    Submitted,
    /// 部分成交
    #[serde(rename = "partial_filled")]
    #[strum(serialize = "partial_filled")]
    PartialFilled,
    /// 完全成交
    #[serde(rename = "filled")]
    #[strum(serialize = "filled")]
    Filled,
    /// 取消中
    #[serde(rename = "canceling")]
    #[strum(serialize = "canceling")]
    Canceling,
    /// 已取消
    #[serde(rename = "canceled")]
    #[strum(serialize = "canceled")]
    Canceled,
    /// 已拒绝
    #[serde(rename = "rejected")]
    #[strum(serialize = "rejected")]
    Rejected,
}
