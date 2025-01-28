use anyhow::{anyhow, Result};
use chrono::{DateTime, TimeZone, Utc};

pub fn ms_to_date(ms: i64) -> Result<DateTime<Utc>> {
    Utc.timestamp_millis_opt(ms)
        .single()
        .ok_or(anyhow!("convert ms to date failed"))
}

pub fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

pub fn now() -> DateTime<Utc> {
    Utc::now()
}
