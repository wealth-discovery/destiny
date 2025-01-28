use std::io::Write;

use anyhow::Result;
use destiny_engine::history_data;
use destiny_helpers::prelude::*;

#[tokio::test]
async fn test_backtest() -> Result<()> {
    let save_dir = cache_dir()?.join("market_data");
    create_dir(&save_dir)?;
    let save_file = save_dir.join("filemeta.txt");
    let file_list = history_data::get_file_list().await?;
    delete_file(&save_file)?;
    let mut f = create_file(&save_file)?;
    for file_item in file_list {
        writeln!(f, "{}", file_item)?;
    }
    Ok(())
}
