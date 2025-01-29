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

pub fn str_to_date(s: &str) -> Result<DateTime<Utc>> {
    let t = if s.len() == 4 {
        DateTime::parse_from_str(&format!("{s}0101000000+00:00"), "%Y%m%d%H%M%S%z")?.to_utc()
    } else if s.len() == 6 {
        DateTime::parse_from_str(&format!("{s}01000000+00:00"), "%Y%m%d%H%M%S%z")?.to_utc()
    } else if s.len() == 8 {
        DateTime::parse_from_str(&format!("{s}000000+00:00"), "%Y%m%d%H%M%S%z")?.to_utc()
    } else if s.len() == 10 {
        DateTime::parse_from_str(&format!("{s}0000+00:00"), "%Y%m%d%H%M%S%z")?.to_utc()
    } else if s.len() == 12 {
        DateTime::parse_from_str(&format!("{s}00+00:00"), "%Y%m%d%H%M%S%z")?.to_utc()
    } else if s.len() == 14 {
        DateTime::parse_from_str(&format!("{s}+00:00"), "%Y%m%d%H%M%S%z")?.to_utc()
    } else {
        return Err(anyhow!("convert str to date failed: {}", s));
    };
    Ok(t)
}
