use anyhow::{bail, Result};
use async_zip::base::read::seek::ZipFileReader;
use chrono::{DateTime, Datelike, Duration, Months, Utc};
use destiny_helpers::prelude::*;
use destiny_types::enums::KlineInterval;
use futures::AsyncReadExt;
use std::path::PathBuf;
use strum::IntoEnumIterator;
use tokio::{
    fs::{create_dir_all, File},
    io::{AsyncWriteExt, BufReader},
    time::sleep,
};

const DOWNLOAD_PREFIX: &str = "https://data.binance.vision/data/futures/um/monthly";

#[derive(Debug)]
pub enum SyncHistoryMeta {
    AggTrades {
        symbol: String,
        year: i64,
        month: i64,
    },
    BookTicker {
        symbol: String,
        year: i64,
        month: i64,
    },
    FundingRate {
        symbol: String,
        year: i64,
        month: i64,
    },
    IndexPriceKlines {
        symbol: String,
        interval: KlineInterval,
        year: i64,
        month: i64,
    },
    Klines {
        symbol: String,
        interval: KlineInterval,
        year: i64,
        month: i64,
    },
    MarkPriceKlines {
        symbol: String,
        interval: KlineInterval,
        year: i64,
        month: i64,
    },
    PremiumIndexKlines {
        symbol: String,
        interval: KlineInterval,
        year: i64,
        month: i64,
    },
    Trades {
        symbol: String,
        year: i64,
        month: i64,
    },
}

impl SyncHistoryMeta {
    pub fn url(&self) -> String {
        match self {
            SyncHistoryMeta::AggTrades {
                symbol,
                year,
                month,
            } => format!("{DOWNLOAD_PREFIX}/aggTrades/{symbol}/{symbol}-aggTrades-{year}-{month:02}.zip"),  
            SyncHistoryMeta::BookTicker {
                symbol,
                year,
                month,
            } => format!("{DOWNLOAD_PREFIX}/bookTicker/{symbol}/{symbol}-bookTicker-{year}-{month:02}.zip"),  
            SyncHistoryMeta::FundingRate {
                symbol,
                year,
                month,
            } => format!("{DOWNLOAD_PREFIX}/fundingRate/{symbol}/{symbol}-fundingRate-{year}-{month:02}.zip"),  
            SyncHistoryMeta::IndexPriceKlines {
                symbol,
                interval,
                year,
                month,
            } => format!("{DOWNLOAD_PREFIX}/indexPriceKlines/{symbol}/{interval}/{symbol}-{interval}-{year}-{month:02}.zip"),
            SyncHistoryMeta::Klines {
                symbol,
                interval,
                year,
                month,
            } =>  format!("{DOWNLOAD_PREFIX}/klines/{symbol}/{interval}/{symbol}-{interval}-{year}-{month:02}.zip"),
            SyncHistoryMeta::MarkPriceKlines {
                symbol,
                interval,
                year,
                month,
            } =>  format!("{DOWNLOAD_PREFIX}/markPriceKlines/{symbol}/{interval}/{symbol}-{interval}-{year}-{month:02}.zip"),
            SyncHistoryMeta::PremiumIndexKlines {
                symbol,
                interval,
                year,
                month,
            } =>  format!("{DOWNLOAD_PREFIX}/premiumIndexKlines/{symbol}/{interval}/{symbol}-{interval}-{year}-{month:02}.zip"), 
            SyncHistoryMeta::Trades {
                symbol,
                year,
                month,
            } => format!("{DOWNLOAD_PREFIX}/trades/{symbol}/{symbol}-trades-{year}-{month:02}.zip"),  
        }
    }

    pub fn save_path(&self) -> PathBuf {
        match self {
            SyncHistoryMeta::AggTrades {
                symbol,
                year: _,
                month: _,
            } => PathBuf::new().join(symbol).join("aggTrades"),
            SyncHistoryMeta::BookTicker {
                symbol,
                year: _,
                month: _,
            } => PathBuf::new().join(symbol).join("bookTicker"),
            SyncHistoryMeta::FundingRate {
                symbol,
                year: _,
                month: _,
            } => PathBuf::new().join(symbol).join("fundingRate"),
            SyncHistoryMeta::IndexPriceKlines {
                symbol,
                interval,
                year: _,
                month: _,
            } => PathBuf::new()
                .join(symbol)
                .join("indexPriceKlines")
                .join(interval.to_string()),
            SyncHistoryMeta::Klines {
                symbol,
                interval,
                year: _,
                month: _,
            } => PathBuf::new()
                .join(symbol)
                .join("klines")
                .join(interval.to_string()),
            SyncHistoryMeta::MarkPriceKlines {
                symbol,
                interval,
                year: _,
                month: _,
            } => PathBuf::new()
                .join(symbol)
                .join("markPriceKlines")
                .join(interval.to_string()),
            SyncHistoryMeta::PremiumIndexKlines {
                symbol,
                interval,
                year: _,
                month: _,
            } => PathBuf::new()
                .join(symbol)
                .join("premiumIndexKlines")
                .join(interval.to_string()),
            SyncHistoryMeta::Trades {
                symbol,
                year: _,
                month: _,
            } => PathBuf::new().join(symbol).join("trades"),
        }
    }

    pub fn save_file_name(&self) -> String {
        match self {
            SyncHistoryMeta::AggTrades {
                symbol: _,
                year,
                month,
            } => format!("{year}{month:02}.csv"),
            SyncHistoryMeta::BookTicker {
                symbol: _,
                year,
                month,
            } => format!("{year}{month:02}.csv"),
            SyncHistoryMeta::FundingRate {
                symbol: _,
                year,
                month,
            } => format!("{year}{month:02}.csv"),
            SyncHistoryMeta::IndexPriceKlines {
                symbol: _,
                interval: _,
                year,
                month,
            } => format!("{year}{month:02}.csv"),
            SyncHistoryMeta::Klines {
                symbol: _,
                interval: _,
                year,
                month,
            } => format!("{year}{month:02}.csv"),
            SyncHistoryMeta::MarkPriceKlines {
                symbol: _,
                interval: _,
                year,
                month,
            } => format!("{year}{month:02}.csv"),
            SyncHistoryMeta::PremiumIndexKlines {
                symbol: _,
                interval: _,
                year,
                month,
            } => format!("{year}{month:02}.csv"),
            SyncHistoryMeta::Trades {
                symbol: _,
                year,
                month,
            } => format!("{year}{month:02}.csv"),
        }
    }
}

impl SyncHistoryMeta {
    pub fn agg_trades(symbol: &str, year: i64, month: i64) -> Self {
        Self::AggTrades {
            symbol: symbol.to_string(),
            year,
            month,
        }
    }

    pub fn book_ticker(symbol: &str, year: i64, month: i64) -> Self {
        Self::BookTicker {
            symbol: symbol.to_string(),
            year,
            month,
        }
    }

    pub fn funding_rate(symbol: &str, year: i64, month: i64) -> Self {
        Self::FundingRate {
            symbol: symbol.to_string(),
            year,
            month,
        }
    }

    pub fn index_price_klines(
        symbol: &str,
        interval: KlineInterval,
        year: i64,
        month: i64,
    ) -> Self {
        Self::IndexPriceKlines {
            symbol: symbol.to_string(),
            interval,
            year,
            month,
        }
    }

    pub fn klines(symbol: &str, interval: KlineInterval, year: i64, month: i64) -> Self {
        Self::Klines {
            symbol: symbol.to_string(),
            interval,
            year,
            month,
        }
    }

    pub fn mark_price_klines(symbol: &str, interval: KlineInterval, year: i64, month: i64) -> Self {
        Self::MarkPriceKlines {
            symbol: symbol.to_string(),
            interval,
            year,
            month,
        }
    }

    pub fn premium_index_klines(
        symbol: &str,
        interval: KlineInterval,
        year: i64,
        month: i64,
    ) -> Self {
        Self::PremiumIndexKlines {
            symbol: symbol.to_string(),
            interval,
            year,
            month,
        }
    }

    pub fn trades(symbol: &str, year: i64, month: i64) -> Self {
        Self::Trades {
            symbol: symbol.to_string(),
            year,
            month,
        }
    }
}

impl SyncHistoryMeta {
    pub fn desc(&self) -> String {
        match self {
            SyncHistoryMeta::AggTrades {
                symbol,
                year,
                month,
            } => format!("交易对({symbol}),日期({year}-{month:02}),类型(聚合交易)"),
            SyncHistoryMeta::BookTicker {
                symbol,
                year,
                month,
            } => format!("交易对({symbol}),日期({year}-{month:02}),类型(盘口)"),
            SyncHistoryMeta::FundingRate {
                symbol,
                year,
                month,
            } => format!("交易对({symbol}),日期({year}-{month:02}),类型(资金费率)"),
            SyncHistoryMeta::IndexPriceKlines {
                symbol,
                interval,
                year,
                month,
            } => {
                format!("交易对({symbol}),日期({year}-{month:02}),类型(指数价格),周期({interval})")
            }
            SyncHistoryMeta::Klines {
                symbol,
                interval,
                year,
                month,
            } => format!("交易对({symbol}),日期({year}-{month:02}),类型(K线),周期({interval})"),
            SyncHistoryMeta::MarkPriceKlines {
                symbol,
                interval,
                year,
                month,
            } => {
                format!("交易对({symbol}),日期({year}-{month:02}),类型(标记价格),周期({interval})")
            }
            SyncHistoryMeta::PremiumIndexKlines {
                symbol,
                interval,
                year,
                month,
            } => {
                format!("交易对({symbol}),日期({year}-{month:02}),类型(溢价指数),周期({interval})")
            }
            SyncHistoryMeta::Trades {
                symbol,
                year,
                month,
            } => format!("交易对({symbol}),日期({year}-{month:02}),类型(交易)"),
        }
    }
}

impl SyncHistoryMeta {
    async fn sync(&self) {
        while let Err(err) = self.sync0().await {
            tracing::error!("同步失败: {}", err);
            sleep(Duration::milliseconds(200).to_std().unwrap()).await;
        }
    }

    async fn sync0(&self) -> Result<()> {
        tracing::trace!("同步信息: {}", self.desc());

        let save_path = PathBuf::cache()?
            .join("history_data")
            .join(self.save_path());
        if !save_path.exists() {
            create_dir_all(&save_path).await?;
        }

        let save_file_path = save_path.join(self.save_file_name());
        if save_file_path.exists() {
            tracing::trace!("本地数据已存在");
            return Ok(());
        }

        tracing::trace!("开始下载...");

        let request_url = self.url();
        let response = reqwest::get(request_url).await?;
        if !response.status().is_success() {
            if response.status().as_u16() == 404 {
                tracing::trace!("状态码: 404");
                return Ok(());
            }
            bail!("下载失败: {}", response.status());
        }

        let bytes = response.bytes().await?;
        let reader = BufReader::new(std::io::Cursor::new(bytes));
        let mut zip = ZipFileReader::with_tokio(reader).await?;
        let mut csv_reader = zip.reader_with_entry(0).await?;
        let mut csv_data = Vec::new();
        csv_reader.read_to_end(&mut csv_data).await?;

        let mut csv_file = File::create(save_file_path).await?;
        csv_file.write_all(&csv_data).await?;
        csv_file.shutdown().await?;

        tracing::trace!("下载成功");

        Ok(())
    }
}

pub struct SyncHistoryData;
impl SyncHistoryData {
    pub async fn sync_symbol(symbol: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<()> {
        let mut start = start.truncate_month()?;
        let end = end.truncate_month()?;

        while start <= end {
            // SyncHistoryMeta::agg_trades(symbol, start.year() as i64, start.month() as i64)
            //     .sync()
            //     .await;

            // SyncHistoryMeta::book_ticker(symbol, start.year() as i64, start.month() as i64)
            //     .sync()
            //     .await;

            SyncHistoryMeta::funding_rate(symbol, start.year() as i64, start.month() as i64)
                .sync()
                .await;

            // SyncHistoryMeta::trades(symbol, start.year() as i64, start.month() as i64)
            //     .sync()
            //     .await;

            for interval in KlineInterval::iter() {
                if matches!(
                    interval,
                    KlineInterval::D3 | KlineInterval::W1 | KlineInterval::Mo1
                ) {
                    continue;
                }

                SyncHistoryMeta::index_price_klines(
                    symbol,
                    interval,
                    start.year() as i64,
                    start.month() as i64,
                )
                .sync()
                .await;

                SyncHistoryMeta::klines(
                    symbol,
                    interval,
                    start.year() as i64,
                    start.month() as i64,
                )
                .sync()
                .await;

                SyncHistoryMeta::mark_price_klines(
                    symbol,
                    interval,
                    start.year() as i64,
                    start.month() as i64,
                )
                .sync()
                .await;

                SyncHistoryMeta::premium_index_klines(
                    symbol,
                    interval,
                    start.year() as i64,
                    start.month() as i64,
                )
                .sync()
                .await;
            }
            start = start + Months::new(1);
        }

        Ok(())
    }
}
