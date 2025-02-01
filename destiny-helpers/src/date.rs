use anyhow::{anyhow, Result};
use chrono::{DateTime, Datelike, Duration, DurationRound, TimeZone, Utc};

pub trait I64DateSupport {
    /// 毫秒转换为日期
    fn to_date(&self) -> Result<DateTime<Utc>>;
    /// 获取当前时间戳
    fn now_ms() -> i64;
}

impl I64DateSupport for i64 {
    fn to_date(&self) -> Result<DateTime<Utc>> {
        Utc.timestamp_millis_opt(*self)
            .single()
            .ok_or(anyhow!("毫秒转换日期失败"))
    }

    fn now_ms() -> i64 {
        Utc::now().timestamp_millis()
    }
}

pub trait DateTimeSupport {
    /// 获取当前日期
    fn now() -> DateTime<Utc>;
    /// 将日期截断到秒
    fn truncate_second(&self) -> Result<DateTime<Utc>>;
    /// 将日期截断到分钟
    fn truncate_minute(&self) -> Result<DateTime<Utc>>;
    /// 将日期截断到小时
    fn truncate_hour(&self) -> Result<DateTime<Utc>>;
    /// 将日期截断到天
    fn truncate_day(&self) -> Result<DateTime<Utc>>;
    /// 将日期截断到月
    fn truncate_month(&self) -> Result<DateTime<Utc>>;
    /// 将日期截断到年
    fn truncate_year(&self) -> Result<DateTime<Utc>>;
    /// 格式化日期 [`202501`]
    fn str_ym(&self) -> String;
    /// 格式化日期 [`20250102`]
    fn str_ymd(&self) -> String;
    /// 格式化日期 [`20250102_0304`]
    fn str_ymd_hm(&self) -> String;
    /// 格式化日期 [`20250102_030405`]
    fn str_ymd_hms(&self) -> String;
    /// 格式化日期 [`20250102_030405_060708`]
    fn str_ymd_hms_6(&self) -> String;
}

impl DateTimeSupport for DateTime<Utc> {
    fn now() -> DateTime<Utc> {
        Utc::now()
    }

    fn truncate_second(&self) -> Result<DateTime<Utc>> {
        Ok(self.duration_trunc(Duration::seconds(1))?)
    }

    fn truncate_minute(&self) -> Result<DateTime<Utc>> {
        Ok(self.duration_trunc(Duration::minutes(1))?)
    }

    fn truncate_hour(&self) -> Result<DateTime<Utc>> {
        Ok(self.duration_trunc(Duration::hours(1))?)
    }

    fn truncate_day(&self) -> Result<DateTime<Utc>> {
        Ok(self.duration_trunc(Duration::days(1))?)
    }

    fn truncate_month(&self) -> Result<DateTime<Utc>> {
        self.with_day(1).ok_or(anyhow!("日期转换失败"))
    }

    fn truncate_year(&self) -> Result<DateTime<Utc>> {
        self.with_month(1).ok_or(anyhow!("日期转换失败"))
    }

    fn str_ym(&self) -> String {
        self.format("%Y%m").to_string()
    }

    fn str_ymd(&self) -> String {
        self.format("%Y%m%d").to_string()
    }

    fn str_ymd_hm(&self) -> String {
        self.format("%Y%m%d_%H%M").to_string()
    }

    fn str_ymd_hms(&self) -> String {
        self.format("%Y%m%d%_H%M%S").to_string()
    }

    fn str_ymd_hms_6(&self) -> String {
        self.format("%Y%m%d_%H%M%S_%6f").to_string()
    }
}

pub trait StrDateSupport {
    /// 字符串转换为日期
    /// <br> 支持的日期格式如下:
    /// <br> [`2025`]
    /// <br> [`202501`]
    /// <br> [`20250102`]
    /// <br> [`2025010203`]
    /// <br> [`202501020304`]
    /// <br> [`20250102030405`]
    fn to_date(&self) -> Result<DateTime<Utc>>;
}

impl StrDateSupport for &str {
    fn to_date(&self) -> Result<DateTime<Utc>> {
        let t = if self.len() == 4 {
            DateTime::parse_from_str(&format!("{self}0101000000+00:00"), "%Y%m%d%H%M%S%z")?.to_utc()
        } else if self.len() == 6 {
            DateTime::parse_from_str(&format!("{self}01000000+00:00"), "%Y%m%d%H%M%S%z")?.to_utc()
        } else if self.len() == 8 {
            DateTime::parse_from_str(&format!("{self}000000+00:00"), "%Y%m%d%H%M%S%z")?.to_utc()
        } else if self.len() == 10 {
            DateTime::parse_from_str(&format!("{self}0000+00:00"), "%Y%m%d%H%M%S%z")?.to_utc()
        } else if self.len() == 12 {
            DateTime::parse_from_str(&format!("{self}00+00:00"), "%Y%m%d%H%M%S%z")?.to_utc()
        } else if self.len() == 14 {
            DateTime::parse_from_str(&format!("{self}+00:00"), "%Y%m%d%H%M%S%z")?.to_utc()
        } else {
            return Err(anyhow!("日期转换失败: {}", self));
        };
        Ok(t)
    }
}
