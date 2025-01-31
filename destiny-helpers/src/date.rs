use anyhow::{anyhow, Result};
use chrono::{DateTime, Datelike, Duration, DurationRound, TimeZone, Utc};

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

/// 将日期截断到秒
pub fn truncate_date_to_second(date: DateTime<Utc>) -> Result<DateTime<Utc>> {
    Ok(date.duration_trunc(Duration::seconds(1))?)
}

/// 将日期截断到分钟
pub fn truncate_date_to_minute(date: DateTime<Utc>) -> Result<DateTime<Utc>> {
    Ok(date.duration_trunc(Duration::minutes(1))?)
}

/// 将日期截断到小时
pub fn truncate_date_to_hour(date: DateTime<Utc>) -> Result<DateTime<Utc>> {
    Ok(date.duration_trunc(Duration::hours(1))?)
}

/// 将日期截断到天
pub fn truncate_date_to_day(date: DateTime<Utc>) -> Result<DateTime<Utc>> {
    Ok(date.duration_trunc(Duration::days(1))?)
}

/// 将日期截断到月
pub fn truncate_date_to_month(date: DateTime<Utc>) -> Result<DateTime<Utc>> {
    truncate_date_to_day(date.with_day(1).ok_or(anyhow!("日期转换失败"))?)
}

/// 将日期截断到年
pub fn truncate_date_to_year(date: DateTime<Utc>) -> Result<DateTime<Utc>> {
    truncate_date_to_month(date.with_month(1).ok_or(anyhow!("日期转换失败"))?)
}
