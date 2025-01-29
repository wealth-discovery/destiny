use anyhow::{bail, Result};
use async_zip::base::read::seek::ZipFileReader;
use destiny_helpers::path::cache_dir;
use destiny_types::enums::KlineInterval;
use futures::AsyncReadExt;
use std::path::PathBuf;
use tokio::{
    fs::{create_dir_all, File},
    io::{AsyncWriteExt, BufReader},
};
use tracing::instrument;

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

#[instrument(name = "SyncHistoryData")]
pub async fn sync(meta: SyncHistoryMeta) -> Result<()> {
    let save_path = cache_dir()?.join("history_data").join(meta.save_path());
    if !save_path.exists() {
        create_dir_all(&save_path).await?;
    }

    let save_file_path = save_path.join(meta.save_file_name());
    if save_file_path.exists() {
        tracing::info!("history data already exists: {}", meta.save_file_name());
        return Ok(());
    }

    let request_url = meta.url();
    let response = reqwest::get(request_url).await?;
    if !response.status().is_success() {
        if response.status().as_u16() == 404 {
            tracing::warn!("history data not found: {}", meta.save_file_name());
            return Ok(());
        }
        bail!("failed to download history data: {}", response.status());
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
    tracing::info!("download history data success: {}", meta.save_file_name());

    Ok(())
}
