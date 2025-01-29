use crate::path::cache_dir;
use anyhow::Result;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::time::FormatTime, layer::SubscriberExt, Layer};

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = chrono::Utc::now().with_timezone(
            &chrono::FixedOffset::east_opt(8 * 3600).expect("failed to create timezone offset"),
        );
        write!(w, "{}", now.format("%Y-%m-%d %H:%M:%S"))
    }
}

pub fn init_log() -> Result<()> {
    let dir = cache_dir()?.join("logs");
    std::fs::create_dir_all(&dir)?;

    let appender = tracing_appender::rolling::daily(dir, "log");
    let (writer, file_guard) = tracing_appender::non_blocking(appender);

    let file_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_level(true)
        .with_writer(writer)
        .with_filter(LevelFilter::INFO);

    let (writer, std_guard) = tracing_appender::non_blocking(std::io::stdout());

    let std_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_level(true)
        .with_writer(writer)
        .with_timer(LocalTimer)
        .with_filter(LevelFilter::TRACE);

    let collector = tracing_subscriber::registry()
        .with(file_layer)
        .with(std_layer);

    tracing::subscriber::set_global_default(collector).expect("failed to set global default");

    std::mem::forget(file_guard);
    std::mem::forget(std_guard);

    Ok(())
}
