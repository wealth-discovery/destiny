use anyhow::Result;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion, Region};

pub async fn get_file_list() -> Result<Vec<String>> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(RegionProviderChain::default_provider().or_else(Region::new("us-east-1")))
        .no_credentials()
        .load()
        .await;
    let client = aws_sdk_s3::Client::new(&config);

    let mut result = Vec::new();

    let mut continuation_token: Option<String> = None;

    loop {
        let response = client
            .list_objects_v2()
            .bucket("hyperliquid-archive")
            .prefix("market_data/")
            .set_continuation_token(continuation_token)
            .send()
            .await?;

        for content in response.contents() {
            let key = content.key().unwrap();
            if key.ends_with("BTC.lz4") {
                result.push(key.to_string());
            }
        }
        continuation_token = response.next_continuation_token().map(|s| s.to_string());
        if continuation_token.is_none() {
            break;
        }
    }

    Ok(result)
}
