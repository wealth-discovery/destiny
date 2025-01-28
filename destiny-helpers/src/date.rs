use anyhow::{anyhow, Result};
use chrono::{Date, DateTime, TimeZone, Utc};

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

pub fn str_to_date(s: &str) -> Result<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(s)
        .map(|d| d.to_utc())
        .map_err(|e| anyhow!("convert str to date failed: {}", e))
}
