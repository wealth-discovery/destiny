use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use destiny_helpers::prelude::*;
use destiny_types::prelude::*;
use std::sync::Arc;

pub trait Engine:
    EngineBasic + EngineInit + EngineTrade + EngineAccount + EngineMarket + Send + Sync
{
}

/// 引擎基础功能
pub trait EngineBasic: Send + Sync {
    /// 将毫秒转换为日期
    fn ms_to_date(&self, ms: i64) -> Result<DateTime<Utc>> {
        ms_to_date(ms)
    }

    /// 获取当前时间戳
    fn now_ms(&self) -> i64 {
        now_ms()
    }

    /// 获取当前日期
    fn now(&self) -> DateTime<Utc> {
        now()
    }

    /// 生成一个32位的小写UUID(V4版本)
    fn gen_id(&self) -> String {
        gen_id()
    }

    /// 截断数值
    /// <br> [`decimals`]: 表示小数点后保留的位数.
    /// <br> [`round_up`]: 表示是否四舍五入.
    fn truncate_float(&self, val: f64, decimals: u32, round_up: bool) -> f64 {
        truncate_float(val, decimals, round_up)
    }

    /// 判断是否为0
    /// <br> 如果[`val`]的绝对值小于[`1e-8`],则认为[`val`]为0
    fn is_zero(&self, val: f64) -> bool {
        is_zero(val)
    }
}

/// 引擎初始化
pub trait EngineInit: Send + Sync {
    /// 初始化交易对
    /// <br> 重复初始化会报错.
    fn init_symbol(&self, symbol: &str) -> Result<()>;
}

/// 引擎交易
#[async_trait]
pub trait EngineTrade: Send + Sync {
    /// 市价开多
    /// <br> [`symbol`]: 交易对
    /// <br> [`size`]: 数量
    /// <br> 返回订单ID
    async fn open_long_market(&self, symbol: &str, size: f64) -> Result<String>;
    /// 限价开多
    /// <br> [`symbol`]: 交易对
    /// <br> [`size`]: 数量
    /// <br> [`price`]: 开仓价格
    /// <br> 返回订单ID
    async fn open_long_limit(&self, symbol: &str, size: f64, price: f64) -> Result<String>;
    /// 市价平多
    /// <br> [`symbol`]: 交易对
    /// <br> [`size`]: 数量
    /// <br> 返回订单ID
    async fn close_long_market(&self, symbol: &str, size: f64) -> Result<String>;
    /// 限价平多
    /// <br> [`symbol`]: 交易对
    /// <br> [`size`]: 数量
    /// <br> [`price`]: 平仓价格
    /// <br> 返回订单ID
    async fn close_long_limit(&self, symbol: &str, size: f64, price: f64) -> Result<String>;
    /// 市价开空
    /// <br> [`symbol`]: 交易对
    /// <br> [`size`]: 数量
    /// <br> 返回订单ID
    async fn open_short_market(&self, symbol: &str, size: f64) -> Result<String>;
    /// 限价开空
    /// <br> [`symbol`]: 交易对
    /// <br> [`size`]: 数量
    /// <br> [`price`]: 开仓价格
    /// <br> 返回订单ID
    async fn open_short_limit(&self, symbol: &str, size: f64, price: f64) -> Result<String>;
    /// 市价平空
    /// <br> [`symbol`]: 交易对
    /// <br> [`size`]: 数量
    /// <br> 返回订单ID
    async fn close_short_market(&self, symbol: &str, size: f64) -> Result<String>;
    /// 限价平空
    /// <br> [`symbol`]: 交易对
    /// <br> [`size`]: 数量
    /// <br> [`price`]: 平仓价格
    /// <br> 返回订单ID
    async fn close_short_limit(&self, symbol: &str, size: f64, price: f64) -> Result<String>;
    /// 撤单
    /// <br> [`symbol`]: 交易对
    /// <br> [`order_id`]: 订单ID
    async fn cancel_order(&self, symbol: &str, order_id: &str) -> Result<()>;
    /// 批量撤单
    /// <br> [`symbol`]: 交易对
    /// <br> [`order_ids`]: 订单ID列表
    async fn cancel_orders(&self, symbol: &str, order_ids: &[&str]) -> Result<()>;
    /// 设置杠杆倍率
    /// <br> [`symbol`]: 交易对
    /// <br> [`leverage`]: 杠杆倍率
    async fn set_leverage(&self, symbol: &str, leverage: u32) -> Result<()>;
    /// 获取杠杆倍率
    /// <br> [`symbol`]: 交易对
    fn leverage(&self, symbol: &str) -> Result<u32>;
    /// 获取订单
    /// <br> [`symbol`]: 交易对
    /// <br> [`order_id`]: 订单ID
    fn order(&self, symbol: &str, order_id: &str) -> Result<Option<Order>>;
    /// 获取交易对订单
    /// <br> [`symbol`]: 交易对
    fn orders(&self, symbol: &str) -> Result<Vec<Order>>;
    /// 获取多头订单
    /// <br> [`symbol`]: 交易对
    fn orders_long(&self, symbol: &str) -> Result<Vec<Order>>;
    /// 获取多头开仓订单
    /// <br> [`symbol`]: 交易对
    fn orders_open_long(&self, symbol: &str) -> Result<Vec<Order>>;
    /// 获取多头平仓订单
    /// <br> [`symbol`]: 交易对
    fn orders_close_long(&self, symbol: &str) -> Result<Vec<Order>>;
    /// 获取空头订单
    /// <br> [`symbol`]: 交易对
    fn orders_short(&self, symbol: &str) -> Result<Vec<Order>>;
    /// 获取空头开仓订单
    /// <br> [`symbol`]: 交易对
    fn orders_open_short(&self, symbol: &str) -> Result<Vec<Order>>;
    /// 获取空头平仓订单
    /// <br> [`symbol`]: 交易对
    fn orders_close_short(&self, symbol: &str) -> Result<Vec<Order>>;
}

pub trait EngineAccount: Send + Sync {
    /// 获取保证金
    fn cash(&self) -> Cash;
    /// 获取持仓
    /// <br> [`symbol`]: 交易对
    fn position(&self, symbol: &str) -> Result<SymbolPosition>;
}

pub trait EngineMarket: Send + Sync {
    /// 获取交易对
    /// <br> [`symbol`]: 交易对
    fn symbol(&self, symbol: &str) -> Result<Symbol>;
    /// 获取交易规则
    /// <br> [`symbol`]: 交易对
    fn symbol_rule(&self, symbol: &str) -> Result<SymbolRule>;
    /// 获取指数
    /// <br> [`symbol`]: 交易对
    fn symbol_index(&self, symbol: &str) -> Result<SymbolIndex>;
}

/// 策略
#[async_trait]
#[allow(unused_variables)]
pub trait Strategy: Send + Sync {
    /// 初始化事件
    async fn on_init(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }

    /// 开始事件
    async fn on_start(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }

    /// 停止事件
    async fn on_stop(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }

    /// 每日事件
    async fn on_daily(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }

    /// 每小时事件
    async fn on_hourly(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }

    /// 每分钟事件
    async fn on_minutely(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }

    /// 每秒事件
    async fn on_secondly(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }

    /// Tick事件
    async fn on_tick(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }

    /// 订单事件
    async fn on_order(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }

    /// 市场行情变化事件
    async fn on_market(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }

    /// 账户变化事件
    async fn on_account(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }
}
