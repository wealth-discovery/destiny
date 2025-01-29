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

pub trait EngineBasic: Send + Sync {
    fn ms_to_date(&self, ms: i64) -> Result<DateTime<Utc>> {
        ms_to_date(ms)
    }
    fn now_ms(&self) -> i64 {
        now_ms()
    }
    fn now(&self) -> DateTime<Utc> {
        now()
    }
    fn gen_id(&self) -> String {
        gen_id()
    }
    fn truncate_float(&self, val: f64, decimals: u32, round_up: bool) -> f64 {
        truncate_float(val, decimals, round_up)
    }
    fn is_zero(&self, val: f64) -> bool {
        is_zero(val)
    }
}

pub trait EngineInit: Send + Sync {
    /// 初始化交易对
    fn init_symbol(&self, symbol: &str) -> Result<()>;
}

#[async_trait]
pub trait EngineTrade: Send + Sync {
    /// 市价开多
    async fn open_long_market(&self, symbol: &str, size: f64) -> Result<String>;
    /// 限价开多
    async fn open_long_limit(&self, symbol: &str, size: f64, price: f64) -> Result<String>;
    /// 市价平多
    async fn close_long_market(&self, symbol: &str, size: f64) -> Result<String>;
    /// 限价平多
    async fn close_long_limit(&self, symbol: &str, size: f64, price: f64) -> Result<String>;
    /// 市价开空
    async fn open_short_market(&self, symbol: &str, size: f64) -> Result<String>;
    /// 限价开空
    async fn open_short_limit(&self, symbol: &str, size: f64, price: f64) -> Result<String>;
    /// 市价平空
    async fn close_short_market(&self, symbol: &str, size: f64) -> Result<String>;
    /// 限价平空
    async fn close_short_limit(&self, symbol: &str, size: f64, price: f64) -> Result<String>;
    /// 撤单
    async fn cancel_order(&self, symbol: &str, order_id: &str) -> Result<()>;
    /// 批量撤单
    async fn cancel_orders(&self, symbol: &str, order_ids: &[&str]) -> Result<()>;
    /// 设置杠杆倍率
    async fn set_leverage(&self, symbol: &str, leverage: u32) -> Result<()>;
    /// 获取杠杆倍率
    fn leverage(&self, symbol: &str) -> Result<u32>;
    /// 获取订单
    fn order(&self, symbol: &str, order_id: &str) -> Result<Option<Order>>;
    /// 获取交易对订单
    fn orders(&self, symbol: &str) -> Result<Vec<Order>>;
    /// 获取多头订单
    fn orders_long(&self, symbol: &str) -> Result<Vec<Order>>;
    /// 获取多头开仓订单
    fn orders_open_long(&self, symbol: &str) -> Result<Vec<Order>>;
    /// 获取多头平仓订单
    fn orders_close_long(&self, symbol: &str) -> Result<Vec<Order>>;
    /// 获取空头订单
    fn orders_short(&self, symbol: &str) -> Result<Vec<Order>>;
    /// 获取空头开仓订单
    fn orders_open_short(&self, symbol: &str) -> Result<Vec<Order>>;
    /// 获取空头平仓订单
    fn orders_close_short(&self, symbol: &str) -> Result<Vec<Order>>;
}

pub trait EngineAccount: Send + Sync {
    /// 获取保证金
    fn cash(&self) -> Cash;
    /// 获取持仓
    fn position(&self, symbol: &str) -> Result<SymbolPosition>;
}

pub trait EngineMarket: Send + Sync {
    /// 获取交易对
    fn symbol(&self, symbol: &str) -> Result<Symbol>;
    /// 获取交易规则
    fn symbol_rule(&self, symbol: &str) -> Result<SymbolRule>;
    /// 获取指数
    fn symbol_index(&self, symbol: &str) -> Result<SymbolIndex>;
}

#[async_trait]
#[allow(unused_variables)]
pub trait Strategy: Send + Sync {
    async fn on_init(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }

    async fn on_start(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }

    async fn on_stop(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }

    async fn on_daily(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }

    async fn on_hourly(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }

    async fn on_minutely(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }

    async fn on_secondly(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }

    async fn on_tick(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }

    async fn on_order(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }

    async fn on_market(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }

    async fn on_account(&self, engine: Arc<dyn Engine>) -> Result<()> {
        Ok(())
    }
}
