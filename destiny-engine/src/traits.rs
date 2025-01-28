use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use destiny_helpers::prelude::*;
use std::sync::Arc;

#[async_trait]
pub trait Engine: Send + Sync {
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
