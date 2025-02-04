use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

/// 运行模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Display)]
pub enum RunMode {
    /// 回测
    Backtest,
    /// 测试网
    Testnet,
    /// 主网
    Mainnet,
}

/// 交易类型
#[cfg_attr(feature = "python", pyo3::pyclass(eq, eq_int, frozen))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Display)]
pub enum TradeType {
    /// 限价
    Limit,
    /// 市价
    Market,
}

/// 交易方向
#[cfg_attr(feature = "python", pyo3::pyclass(eq, eq_int, frozen))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Display)]
pub enum TradeSide {
    /// 做多
    Long,
    /// 做空
    Short,
}

/// K线周期
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Display, EnumIter)]
pub enum KlineInterval {
    /// 1分钟
    #[strum(serialize = "1m")]
    M1,
    /// 3分钟
    #[strum(serialize = "3m")]
    M3,
    /// 5分钟
    #[strum(serialize = "5m")]
    M5,
    /// 15分钟
    #[strum(serialize = "15m")]
    M15,
    /// 30分钟
    #[strum(serialize = "30m")]
    M30,
    /// 1小时
    #[strum(serialize = "1h")]
    H1,
    /// 2小时
    #[strum(serialize = "2h")]
    H2,
    /// 4小时
    #[strum(serialize = "4h")]
    H4,
    /// 6小时
    #[strum(serialize = "6h")]
    H6,
    /// 8小时
    #[strum(serialize = "8h")]
    H8,
    /// 12小时
    #[strum(serialize = "12h")]
    H12,
    /// 1天
    #[strum(serialize = "1d")]
    D1,
    /// 3天
    #[strum(serialize = "3d")]
    D3,
    /// 1周
    #[strum(serialize = "1w")]
    W1,
    /// 1月
    #[strum(serialize = "1mo")]
    Mo1,
}

/// 订单状态
#[cfg_attr(feature = "python", pyo3::pyclass(eq, eq_int, frozen))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Display)]
pub enum OrderStatus {
    /// 已创建
    Created,
    /// 已提交
    Submitted,
    /// 部分成交
    PartialFilled,
    /// 完全成交
    Filled,
    /// 取消中
    Canceling,
    /// 已取消
    Canceled,
    /// 已拒绝
    Rejected,
}
