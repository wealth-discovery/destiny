use crate::path::cache_dir;
use anyhow::Result;
use chrono::{Datelike, Timelike};
use derive_builder::Builder;
use std::io::Write;
use tokio::fs::create_dir_all;
use tracing::{field::Visit, level_filters::LevelFilter, Level};
use tracing_subscriber::{layer::SubscriberExt, Layer};

/// 日志配置
#[derive(Builder)]
#[builder(setter(into))]
pub struct LogConfig {
    /// 是否在控制台输出
    #[builder(default = true)]
    pub show_std: bool,
    /// 是否写入文件
    #[builder(default = true)]
    pub save_file: bool,
    /// 可显示的包名, 默认显示 [`destiny_`] 开头的包
    #[builder(default = vec![])]
    pub targets: Vec<String>,
}

struct LogVisitor(Option<String>);

impl Visit for LogVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.0 = Some(format!("{:?}", value))
        }
    }
}

struct LogLayer {
    writers: Vec<tracing_appender::non_blocking::NonBlocking>,
}

impl LogLayer {
    pub async fn new(show_std: bool, save_file: bool) -> Result<Self> {
        let mut writers = vec![];
        if show_std {
            let (writer, guard) = tracing_appender::non_blocking(std::io::stdout());
            writers.push(writer);
            std::mem::forget(guard);
        }

        if save_file {
            let dir = cache_dir()?.join("logs");
            create_dir_all(&dir).await?;
            let appender = tracing_appender::rolling::daily(dir, "log");
            let (writer, guard) = tracing_appender::non_blocking(appender);
            writers.push(writer);
            std::mem::forget(guard);
        }

        Ok(Self { writers })
    }
}

impl<S> Layer<S> for LogLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if self.writers.is_empty() {
            return;
        }

        let mut visitor = LogVisitor(None);
        event.record(&mut visitor);
        let message = visitor.0.unwrap_or_default();

        static FIXED_OFFSET: chrono::FixedOffset =
            chrono::FixedOffset::east_opt(8 * 3600).expect("创建时区偏移失败");
        let now = chrono::Utc::now().with_timezone(&FIXED_OFFSET);
        let year = now.year();
        let month = now.month();
        let day = now.day();
        let hour = now.hour();
        let minute = now.minute();
        let second = now.second();
        let millis = now.timestamp_subsec_millis();
        let micros = now.timestamp_subsec_micros() % 1000;

        let icon = match *event.metadata().level() {
            Level::TRACE => "🧬",
            Level::DEBUG => "🔍",
            Level::INFO => "💬",
            Level::WARN => "🚨",
            Level::ERROR => "💥",
        };

        let msg = format!("{icon} [{year}{month:02}{day:02}][{hour:02}{minute:02}{second:02}][{millis:03}][{micros:03}] - {message}\n");

        for out in self.writers.iter() {
            let mut write = out.clone();
            write.write_all(msg.as_bytes()).ok();
        }
    }
}

/// 初始化日志, 将设置全局的日志配置.
/// <br> 重复初始化会报错.
pub async fn init_log(config: LogConfig) -> Result<()> {
    let mut targets =
        tracing_subscriber::filter::Targets::new().with_target("destiny_", LevelFilter::TRACE);
    for target in config.targets {
        targets = targets.with_target(target, LevelFilter::TRACE);
    }

    let layer = LogLayer::new(config.show_std, config.save_file).await?;

    let collector = tracing_subscriber::registry().with(targets).with(layer);

    tracing::subscriber::set_global_default(collector)?;

    Ok(())
}
