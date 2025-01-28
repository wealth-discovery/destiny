use serde::{Deserialize, Serialize};

/// 运行模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum RunMode {
    /// 回测
    Backtest,
    /// 测试网
    Testnet,
    /// 主网
    Mainnet,
}

/// 产品类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ProductType {
    /// 现货
    Spot,
    /// 合约
    Contract,
}

/// 交易类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum TradeType {
    /// 限价
    Limit,
    /// 市价
    Market,
}

/// 交易方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum TradeSide {
    /// 做多
    Long,
    /// 做空
    Short,
}

/// K线周期
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CandlePeriod {
    /// 1分钟
    #[serde(rename = "1m")]
    M1,
    /// 3分钟
    #[serde(rename = "3m")]
    M3,
    /// 5分钟
    #[serde(rename = "5m")]
    M5,
    /// 15分钟
    #[serde(rename = "15m")]
    M15,
    /// 30分钟
    #[serde(rename = "30m")]
    M30,
    /// 1小时
    #[serde(rename = "1h")]
    H1,
    /// 2小时
    #[serde(rename = "2h")]
    H2,
    /// 4小时
    #[serde(rename = "4h")]
    H4,
    /// 6小时
    #[serde(rename = "6h")]
    H6,
    /// 8小时
    #[serde(rename = "8h")]
    H8,
    /// 12小时
    #[serde(rename = "12h")]
    H12,
    /// 1天
    #[serde(rename = "1d")]
    D1,
    /// 3天
    #[serde(rename = "3d")]
    D3,
    /// 1周
    #[serde(rename = "1w")]
    W1,
    /// 1月
    #[serde(rename = "1M")]
    Mo1,
}

/// 订单状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum OrderStatus {
    /// 已创建
    Created,
    /// 已提交
    Submitted,
    /// 部分成交
    PartialFilled,
    /// 完全成交
    Filled,
    /// 已取消
    Canceled,
    /// 已拒绝
    Rejected,
}
