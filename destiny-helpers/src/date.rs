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
    let t = if s.len() == 4 {
        DateTime::parse_from_str(
            &format!("{s}-01-01 00:00:00 +00:00"),
            "%Y-%m-%d %H:%M:%S %z",
        )?
        .to_utc()
    } else if s.len() == 7 {
        DateTime::parse_from_str(&format!("{s}-01 00:00:00 +00:00"), "%Y-%m-%d %H:%M:%S %z")?
            .to_utc()
    } else if s.len() == 10 {
        DateTime::parse_from_str(&format!("{s} 00:00:00 +00:00"), "%Y-%m-%d %H:%M:%S %z")?.to_utc()
    } else if s.len() == 13 {
        DateTime::parse_from_str(&format!("{s}:00:00 +00:00"), "%Y-%m-%d %H:%M:%S %z")?.to_utc()
    } else if s.len() == 16 {
        DateTime::parse_from_str(&format!("{s}:00 +00:00"), "%Y-%m-%d %H:%M:%S %z")?.to_utc()
    } else if s.len() == 19 {
        DateTime::parse_from_str(&format!("{s} +00:00"), "%Y-%m-%d %H:%M:%S %z")?.to_utc()
    } else {
        return Err(anyhow!("convert str to date failed: {}", s));
    };
    Ok(t)
}
