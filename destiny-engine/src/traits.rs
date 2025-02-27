use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use destiny_types::prelude::*;
use rust_decimal::Decimal;
use std::sync::Arc;

pub trait Engine: EngineInit + EngineAccount + EngineMarket + EngineExchange + Send + Sync {
    fn time(&self) -> DateTime<Utc>;
    fn stop(&self);
}

pub trait EngineInit: Send + Sync {
    fn symbol_init(&self, symbol: &str) -> Result<()>;
}

pub trait EngineAccount: Send + Sync {
    fn order(&self, symbol: &str, id: &str) -> Option<Order>;
    fn orders(&self, symbol: &str) -> Vec<Order>;
    fn orders_long(&self, symbol: &str) -> Vec<Order>;
    fn orders_long_open(&self, symbol: &str) -> Vec<Order>;
    fn orders_long_close(&self, symbol: &str) -> Vec<Order>;
    fn orders_short(&self, symbol: &str) -> Vec<Order>;
    fn orders_short_open(&self, symbol: &str) -> Vec<Order>;
    fn orders_short_close(&self, symbol: &str) -> Vec<Order>;
    fn leverage(&self, symbol: &str) -> u32;
    fn cash(&self) -> Decimal;
    fn cash_available(&self) -> Decimal;
    fn cash_frozen(&self) -> Decimal;
    fn margin(&self) -> Decimal;
    fn pnl(&self) -> Decimal;
    fn long_price(&self, symbol: &str) -> Decimal;
    fn long_size(&self, symbol: &str) -> Decimal;
    fn long_size_available(&self, symbol: &str) -> Decimal;
    fn long_size_frozen(&self, symbol: &str) -> Decimal;
    fn long_margin(&self, symbol: &str) -> Decimal;
    fn long_pnl(&self, symbol: &str) -> Decimal;
    fn short_price(&self, symbol: &str) -> Decimal;
    fn short_size(&self, symbol: &str) -> Decimal;
    fn short_size_available(&self, symbol: &str) -> Decimal;
    fn short_size_frozen(&self, symbol: &str) -> Decimal;
    fn short_margin(&self, symbol: &str) -> Decimal;
    fn short_pnl(&self, symbol: &str) -> Decimal;
    fn symbols(&self) -> Vec<String>;
    fn symbol_pnl(&self, symbol: &str) -> Decimal;
    fn symbol_margin(&self, symbol: &str) -> Decimal;
}

pub trait EngineMarket: Send + Sync {
    fn price_mark(&self, symbol: &str) -> Decimal;
    fn price_last(&self, symbol: &str) -> Decimal;
    fn price_index(&self, symbol: &str) -> Decimal;
    fn price_settlement(&self, symbol: &str) -> Decimal;
    fn time_settlement(&self, symbol: &str) -> DateTime<Utc>;
    fn rule_price_min(&self, symbol: &str) -> Decimal;
    fn rule_price_max(&self, symbol: &str) -> Decimal;
    fn rule_price_tick(&self, symbol: &str) -> Decimal;
    fn rule_size_min(&self, symbol: &str) -> Decimal;
    fn rule_size_max(&self, symbol: &str) -> Decimal;
    fn rule_size_tick(&self, symbol: &str) -> Decimal;
    fn rule_amount_min(&self, symbol: &str) -> Decimal;
    fn rule_order_max(&self, symbol: &str) -> i64;
}

#[async_trait]
pub trait EngineExchange: Send + Sync {
    async fn long_market_open(&self, symbol: &str, size: Decimal) -> Result<String>;
    async fn long_limit_open(&self, symbol: &str, size: Decimal, price: Decimal) -> Result<String>;
    async fn long_market_close(&self, symbol: &str, size: Decimal) -> Result<String>;
    async fn long_limit_close(&self, symbol: &str, size: Decimal, price: Decimal)
        -> Result<String>;
    async fn short_market_open(&self, symbol: &str, size: Decimal) -> Result<String>;
    async fn short_limit_open(&self, symbol: &str, size: Decimal, price: Decimal)
        -> Result<String>;
    async fn short_market_close(&self, symbol: &str, size: Decimal) -> Result<String>;
    async fn short_limit_close(
        &self,
        symbol: &str,
        size: Decimal,
        price: Decimal,
    ) -> Result<String>;
    async fn order_close(&self, symbol: &str, id: &str) -> Result<()>;
    async fn order_cancel_many(&self, symbol: &str, ids: &[String]) -> Result<()>;
    async fn leverage_set(&self, symbol: &str, leverage: u32) -> Result<()>;
}

/// 策略
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
    async fn on_kline(&self, engine: Arc<dyn Engine>, kline: Kline) -> Result<()> {
        Ok(())
    }
    async fn on_order(&self, engine: Arc<dyn Engine>, order: Order) -> Result<()> {
        Ok(())
    }
    async fn on_position(&self, engine: Arc<dyn Engine>, position: Position) -> Result<()> {
        Ok(())
    }
}
