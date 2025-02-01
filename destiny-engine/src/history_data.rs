use anyhow::{anyhow, bail, Result};
use async_zip::base::read::seek::ZipFileReader;
use chrono::{DateTime, Datelike, Duration, Months, Utc};
use destiny_helpers::prelude::*;
use destiny_types::prelude::*;
use futures::{stream::StreamExt, AsyncReadExt};
use std::{cmp::Ordering, path::PathBuf};
use strum::IntoEnumIterator;
use tokio::{
    fs::{create_dir_all, File},
    io::{AsyncWriteExt, BufReader},
    sync::mpsc::{channel, Receiver},
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
            } => format!("日期({year}-{month:02}),交易对({symbol}),类型(聚合交易)"),
            SyncHistoryMeta::BookTicker {
                symbol,
                year,
                month,
            } => format!("日期({year}-{month:02}),交易对({symbol}),类型(盘口)"),
            SyncHistoryMeta::FundingRate {
                symbol,
                year,
                month,
            } => format!("日期({year}-{month:02}),交易对({symbol}),类型(资金费率)"),
            SyncHistoryMeta::IndexPriceKlines {
                symbol,
                interval,
                year,
                month,
            } => {
                format!("日期({year}-{month:02}),交易对({symbol}),类型(指数价格),周期({interval})")
            }
            SyncHistoryMeta::Klines {
                symbol,
                interval,
                year,
                month,
            } => format!("日期({year}-{month:02}),交易对({symbol}),类型(K线),周期({interval})"),
            SyncHistoryMeta::MarkPriceKlines {
                symbol,
                interval,
                year,
                month,
            } => {
                format!("日期({year}-{month:02}),交易对({symbol}),类型(标记价格),周期({interval})")
            }
            SyncHistoryMeta::PremiumIndexKlines {
                symbol,
                interval,
                year,
                month,
            } => {
                format!("日期({year}-{month:02}),交易对({symbol}),类型(溢价指数),周期({interval})")
            }
            SyncHistoryMeta::Trades {
                symbol,
                year,
                month,
            } => format!("日期({year}-{month:02}),交易对({symbol}),类型(交易)"),
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
        tracing::info!("同步信息: {}", self.desc());

        let save_path = PathBuf::cache()?
            .join("history_data")
            .join(self.save_path());
        if !save_path.exists() {
            create_dir_all(&save_path).await?;
        }

        let save_file_path = save_path.join(self.save_file_name());
        if save_file_path.exists() {
            tracing::debug!("本地数据已存在");
            return Ok(());
        }

        tracing::info!("开始下载...");

        let request_url = self.url();
        let response = reqwest::ClientBuilder::default()
            .connect_timeout(std::time::Duration::from_secs(5))
            .pool_idle_timeout(std::time::Duration::from_secs(5))
            .build()?
            .get(request_url)
            .send()
            .await?;
        if !response.status().is_success() {
            if response.status().as_u16() == 404 {
                tracing::warn!("状态码: 404");
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

        tracing::info!("下载成功");

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
                // if matches!(
                //     interval,
                //     KlineInterval::D3 | KlineInterval::W1 | KlineInterval::Mo1
                // ) {
                //     continue;
                // }

                if interval != KlineInterval::M1 {
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

                // SyncHistoryMeta::premium_index_klines(
                //     symbol,
                //     interval,
                //     start.year() as i64,
                //     start.month() as i64,
                // )
                // .sync()
                // .await;
            }
            start = start + Months::new(1);
        }

        Ok(())
    }
}

pub trait DecodeCsvRecordItem {
    fn datetime(&self) -> DateTime<Utc>;
}

pub trait DecodeCsvRecord {
    type T: DecodeCsvRecordItem + Clone + Send + 'static;
    fn decode(record: &csv_async::StringRecord) -> Result<Self::T>;
}

pub struct HistoryData;

impl HistoryData {
    pub async fn csv_read<D>(path: &PathBuf) -> Result<Vec<D::T>>
    where
        D: DecodeCsvRecord,
    {
        if !path.exists() {
            return Ok(vec![]);
        }

        let mut reader = csv_async::AsyncReader::from_reader(File::open(path).await?);
        let mut records = reader.records();

        let mut result = vec![];

        while let Some(record) = records.next().await {
            let record = record?;
            result.push(D::decode(&record)?);
        }

        Ok(result)
    }
}

impl DecodeCsvRecordItem for FundingRateHistory {
    fn datetime(&self) -> DateTime<Utc> {
        self.time
    }
}

impl DecodeCsvRecord for FundingRateHistory {
    type T = Self;

    fn decode(record: &csv_async::StringRecord) -> Result<Self::T> {
        let time = record
            .get(0)
            .ok_or(anyhow!("结算时间不存在"))?
            .parse::<i64>()?
            .to_date()?
            .truncate_hour()?;

        let rate = record
            .get(2)
            .ok_or(anyhow!("资金费率不存在"))?
            .parse::<f64>()?
            .to_safe();

        Ok(Self {
            symbol: Default::default(),
            mark_price: Default::default(),
            rate,
            time,
        })
    }
}

impl DecodeCsvRecordItem for Kline {
    fn datetime(&self) -> DateTime<Utc> {
        self.open_time
    }
}

impl DecodeCsvRecord for Kline {
    type T = Self;

    fn decode(record: &csv_async::StringRecord) -> Result<Self::T> {
        let open_time = record
            .get(0)
            .ok_or(anyhow!("开盘时间不存在"))?
            .parse::<i64>()?
            .to_date()?
            .truncate_minute()?;

        let open = record
            .get(1)
            .ok_or(anyhow!("开盘价不存在"))?
            .parse::<f64>()?
            .to_safe();

        let high = record
            .get(2)
            .ok_or(anyhow!("最高价不存在"))?
            .parse::<f64>()?
            .to_safe();

        let low = record
            .get(3)
            .ok_or(anyhow!("最低价不存在"))?
            .parse::<f64>()?
            .to_safe();

        let close = record
            .get(4)
            .ok_or(anyhow!("收盘价不存在"))?
            .parse::<f64>()?
            .to_safe();

        let size = record
            .get(5)
            .ok_or(anyhow!("成交量不存在"))?
            .parse::<f64>()?
            .to_safe();

        let cash = record
            .get(7)
            .ok_or(anyhow!("成交额不存在"))?
            .parse::<f64>()?
            .to_safe();

        let trades = record
            .get(8)
            .ok_or(anyhow!("交易笔数不存在"))?
            .parse::<i64>()?;

        let buy_size = record
            .get(9)
            .ok_or(anyhow!("买方成交量不存在"))?
            .parse::<f64>()?
            .to_safe();

        let buy_cash = record
            .get(10)
            .ok_or(anyhow!("买方成交额不存在"))?
            .parse::<f64>()?
            .to_safe();

        Ok(Self {
            symbol: Default::default(),
            interval: KlineInterval::M1,
            open_time,
            open,
            high,
            low,
            close,
            size,
            cash,
            buy_size,
            buy_cash,
            trades,
            time: Default::default(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryDataStreamType {
    FundingRate,
    IndexPriceKlines,
    Klines,
    MarkPriceKlines,
    PremiumIndexKlines,
}

pub struct HistoryDataStream<D>
where
    D: DecodeCsvRecord,
{
    data_rx: Receiver<D::T>,
    curr_data: Option<D::T>,
}

impl<D> HistoryDataStream<D>
where
    D: DecodeCsvRecord,
{
    pub fn new(
        symbol: String,
        r#type: HistoryDataStreamType,
        begin: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Self {
        let (tx, data_rx) = channel(10000);
        tokio::spawn(async move {
            let base_path = PathBuf::cache()?.join("history_data").join(symbol);
            let base_path = match r#type {
                HistoryDataStreamType::FundingRate => base_path.join("fundingRate"),
                HistoryDataStreamType::IndexPriceKlines => {
                    base_path.join("indexPriceKlines").join("1m")
                }
                HistoryDataStreamType::Klines => base_path.join("klines").join("1m"),
                HistoryDataStreamType::MarkPriceKlines => {
                    base_path.join("markPriceKlines").join("1m")
                }
                HistoryDataStreamType::PremiumIndexKlines => {
                    base_path.join("premiumIndexKlines").join("1m")
                }
            };
            let mut begin_month = begin.truncate_month()?;
            let end_month = end.truncate_month()?;

            while begin_month <= end_month {
                let path = base_path.join(format!("{}.csv", begin_month.str_ym()));
                tracing::trace!("加载历史数据: {}", path.display());
                let data = HistoryData::csv_read::<D>(&path).await?;
                tracing::trace!("加载完成: {} 数量大小({})", path.display(), data.len());
                for item in data {
                    if item.datetime() >= begin && item.datetime() <= end {
                        tx.send(item).await.expect("发送数据失败");
                        tokio::task::yield_now().await;
                    }
                }

                begin_month = begin_month + Months::new(1);
            }

            anyhow::Ok(())
        });

        Self {
            data_rx,
            curr_data: None,
        }
    }

    pub async fn take(&mut self, date: DateTime<Utc>) -> Result<Option<D::T>> {
        if let Some(curr_data) = &self.curr_data {
            match curr_data.datetime().cmp(&date) {
                Ordering::Equal => return Ok(Some(curr_data.to_owned())),
                Ordering::Greater => return Ok(None),
                _ => {}
            }
        }

        while let Some(data) = self.data_rx.recv().await {
            self.curr_data = Some(data);
            if let Some(curr_data) = &self.curr_data {
                match curr_data.datetime().cmp(&date) {
                    Ordering::Equal => return Ok(Some(curr_data.to_owned())),
                    Ordering::Greater => return Ok(None),
                    _ => {}
                }
            }
        }

        Ok(None)
    }
}
