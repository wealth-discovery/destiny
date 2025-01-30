use anyhow::{anyhow, Result};
use chrono::{DateTime, TimeZone, Utc};

/// 毫秒转为日期
pub fn ms_to_date(ms: i64) -> Result<DateTime<Utc>> {
    Utc.timestamp_millis_opt(ms)
        .single()
        .ok_or(anyhow!("毫秒转换日期失败"))
}

/// 获取当前时间戳
pub fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

/// 获取当前日期
pub fn now() -> DateTime<Utc> {
    Utc::now()
}

/// 字符串转换为日期
/// <br> 支持的日期格式如下:
/// <br> [`2025`]
/// <br> [`202501`]
/// <br> [`20250102`]
/// <br> [`2025010203`]
/// <br> [`202501020304`]
/// <br> [`20250102030405`]
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
        return Err(anyhow!("日期转换失败: {}", s));
    };
    Ok(t)
}
