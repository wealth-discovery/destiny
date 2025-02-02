use destiny_engine::prelude::*;

#[tokio::test]
async fn test_csv() -> Result<()> {
    if bool::has_github_action() {
        return Ok(());
    }

    let log_collector = LogConfigBuilder::default()
        .show_std(true)
        .targets(vec!["csv".to_string()])
        .build()?
        .init_log()?;

    {
        let path = PathBuf::cache()?
            .join("history_data")
            .join("ETHUSDT")
            .join("fundingRate")
            .join("202001.csv");

        let datas = HistoryData::csv_read::<FundingRateHistory>(&path).await?;
        for data in datas {
            tracing::info!("{:?}", data);
        }
    }

    {
        let path = PathBuf::cache()?
            .join("history_data")
            .join("ETHUSDT")
            .join("klines")
            .join("1m")
            .join("202412.csv");

        let datas = HistoryData::csv_read::<Kline>(&path).await?;
        for data in datas {
            tracing::info!("{:?}", data);
        }
    }

    log_collector.done();

    Ok(())
}
