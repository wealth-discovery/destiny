use crate::path::cache_dir;
use anyhow::Result;
use tracing::Level;
use tracing_subscriber::fmt::time::FormatTime;

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let now =
            chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(8 * 3600).unwrap());
        write!(w, "{}", now.format("%Y-%m-%d %H:%M:%S"))
    }
}

pub fn init_log() -> Result<()> {
    let dir = cache_dir()?.join("logs");
    std::fs::create_dir_all(&dir)?;

    let file_appender = tracing_appender::rolling::daily(dir, "log");
    let (file_appender_non_blocking, file_guard) = tracing_appender::non_blocking(file_appender);
    std::mem::forget(file_guard);

    let (stdout_non_blocking, stdout_guard) = tracing_appender::non_blocking(std::io::stdout());
    std::mem::forget(stdout_guard);

    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_timer(LocalTimer);

    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(stdout_non_blocking)
        .with_writer(file_appender_non_blocking)
        .with_ansi(false)
        .event_format(format)
        .init();

    Ok(())
}
