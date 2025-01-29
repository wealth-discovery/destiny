use anyhow::{ensure, Result};
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

enum DownloadMeta {
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

impl DownloadMeta {
    pub fn url(&self) -> String {
        match self {
            DownloadMeta::AggTrades {
                symbol,
                year,
                month,
            } => format!("{DOWNLOAD_PREFIX}/{symbol}/aggTrades/{symbol}/{symbol}-aggTrades-{year}-{month:02}.zip"),  
            DownloadMeta::BookTicker {
                symbol,
                year,
                month,
            } => format!("{DOWNLOAD_PREFIX}/{symbol}/bookTicker/{symbol}/{symbol}-bookTicker-{year}-{month:02}.zip"),  
            DownloadMeta::FundingRate {
                symbol,
                year,
                month,
            } => format!("{DOWNLOAD_PREFIX}/{symbol}/fundingRate/{symbol}/{symbol}-fundingRate-{year}-{month:02}.zip"),  
            DownloadMeta::IndexPriceKlines {
                symbol,
                interval,
                year,
                month,
            } => format!("{DOWNLOAD_PREFIX}/{symbol}/indexPriceKlines/{symbol}/{interval}/{symbol}-{interval}-{year}-{month:02}.zip"),
            DownloadMeta::Klines {
                symbol,
                interval,
                year,
                month,
            } =>  format!("{DOWNLOAD_PREFIX}/{symbol}/klines/{symbol}/{interval}/{symbol}-{interval}-{year}-{month:02}.zip"),
            DownloadMeta::MarkPriceKlines {
                symbol,
                interval,
                year,
                month,
            } =>  format!("{DOWNLOAD_PREFIX}/{symbol}/markPriceKlines/{symbol}/{interval}/{symbol}-{interval}-{year}-{month:02}.zip"),
            DownloadMeta::PremiumIndexKlines {
                symbol,
                interval,
                year,
                month,
            } =>  format!("{DOWNLOAD_PREFIX}/{symbol}/premiumIndexKlines/{symbol}/{interval}/{symbol}-{interval}-{year}-{month:02}.zip"), 
            DownloadMeta::Trades {
                symbol,
                year,
                month,
            } => format!("{DOWNLOAD_PREFIX}/{symbol}/trades/{symbol}/{symbol}-trades-{year}-{month:02}.zip"),  
        }
    }

    pub fn save_path(&self) -> PathBuf {
        match self {
            DownloadMeta::AggTrades {
                symbol,
                year,
                month,
            } => PathBuf::new().join(symbol).join("aggTrades"),
            DownloadMeta::BookTicker {
                symbol,
                year,
                month,
            } => PathBuf::new().join(symbol).join("bookTicker"),
            DownloadMeta::FundingRate {
                symbol,
                year,
                month,
            } => PathBuf::new().join(symbol).join("fundingRate"),
            DownloadMeta::IndexPriceKlines {
                symbol,
                interval,
                year,
                month,
            } => PathBuf::new()
                .join(symbol)
                .join("indexPriceKlines")
                .join(interval.to_string()),
            DownloadMeta::Klines {
                symbol,
                interval,
                year,
                month,
            } => PathBuf::new()
                .join(symbol)
                .join("klines")
                .join(interval.to_string()),
            DownloadMeta::MarkPriceKlines {
                symbol,
                interval,
                year,
                month,
            } => PathBuf::new()
                .join(symbol)
                .join("markPriceKlines")
                .join(interval.to_string()),
            DownloadMeta::PremiumIndexKlines {
                symbol,
                interval,
                year,
                month,
            } => PathBuf::new()
                .join(symbol)
                .join("premiumIndexKlines")
                .join(interval.to_string()),
            DownloadMeta::Trades {
                symbol,
                year,
                month,
            } => PathBuf::new().join(symbol).join("trades"),
        }
    }

    pub fn save_file_name(&self) -> String {
        match self {
            DownloadMeta::AggTrades {
                symbol: _,
                year,
                month,
            } => format!("{year}{month:02}.csv"),
            DownloadMeta::BookTicker {
                symbol: _,
                year,
                month,
            } => format!("{year}{month:02}.csv"),
            DownloadMeta::FundingRate {
                symbol: _,
                year,
                month,
            } => format!("{year}{month:02}.csv"),
            DownloadMeta::IndexPriceKlines {
                symbol: _,
                interval: _,
                year,
                month,
            } => format!("{year}{month:02}.csv"),
            DownloadMeta::Klines {
                symbol: _,
                interval: _,
                year,
                month,
            } => format!("{year}{month:02}.csv"),
            DownloadMeta::MarkPriceKlines {
                symbol: _,
                interval: _,
                year,
                month,
            } => format!("{year}{month:02}.csv"),
            DownloadMeta::PremiumIndexKlines {
                symbol: _,
                interval: _,
                year,
                month,
            } => format!("{year}{month:02}.csv"),
            DownloadMeta::Trades {
                symbol: _,
                year,
                month,
            } => format!("{year}{month:02}.csv"),
        }
    }
}

#[instrument(name = "DownloadHistoryData", skip_all)]
async fn download_history_data(download_meta: DownloadMeta) -> Result<()> {
    let save_path = cache_dir()?
        .join("history_data")
        .join(download_meta.save_path());
    if !save_path.exists() {
        create_dir_all(&save_path).await?;
    }

    let save_file_path = save_path.join(download_meta.save_file_name());
    if save_file_path.exists() {
        tracing::info!(
            "history data already exists: {}",
            download_meta.save_file_name()
        );
        return Ok(());
    }

    let response = reqwest::get(download_meta.url()).await?;
    ensure!(
        response.status().is_success(),
        "failed to download history data: {}",
        response.status()
    );
    let bytes = response.bytes().await?;
    let reader = BufReader::new(std::io::Cursor::new(bytes));
    let mut zip = ZipFileReader::with_tokio(reader).await?;
    let mut csv_reader = zip.reader_with_entry(0).await?;
    let mut csv_data = Vec::new();
    csv_reader.read_to_end(&mut csv_data).await?;

    let mut csv_file = File::create(save_file_path).await?;
    csv_file.write_all(&csv_data).await?;
    csv_file.shutdown().await?;
    tracing::info!(
        "download history data success: {}",
        download_meta.save_file_name()
    );

    Ok(())
}
