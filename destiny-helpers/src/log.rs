use crate::path::PathBufSupport;
use anyhow::Result;
use derive_builder::Builder;
use nu_ansi_term::Color;
use std::{fs::create_dir_all, io::Write, path::PathBuf, thread::JoinHandle};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tracing::{field::Visit, level_filters::LevelFilter, Level};
use tracing_subscriber::{layer::SubscriberExt, Layer};

pub type LogLevel = LevelFilter;

/// 日志配置
#[derive(Builder)]
#[builder(setter(into))]
pub struct LogConfig {
    /// 是否在控制台输出
    #[builder(default = false)]
    pub show_std: bool,
    /// 是否写入文件
    #[builder(default = false)]
    pub save_file: bool,
    /// 可显示的包名,默认显示[`destiny_`]开头的包
    #[builder(default = vec![])]
    pub targets: Vec<String>,
    /// 日志级别
    #[builder(default = LogLevel::INFO)]
    pub level: LogLevel,
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
    level: LogLevel,
    show_std: bool,
    file_writer: Option<tracing_appender::non_blocking::NonBlocking>,
    std_tx: UnboundedSender<Option<String>>,
}

impl LogLayer {
    pub fn new(
        level: LogLevel,
        show_std: bool,
        save_file: bool,
        std_tx: UnboundedSender<Option<String>>,
    ) -> Result<Self> {
        let mut file_writer = None;
        if level != LogLevel::OFF && save_file {
            let dir = PathBuf::cache()?.join("logs");
            create_dir_all(&dir)?;
            let appender = tracing_appender::rolling::daily(dir, "log");
            let (writer, guard) = tracing_appender::non_blocking(appender);
            file_writer = Some(writer);
            std::mem::forget(guard);
        }

        Ok(Self {
            level,
            show_std,
            file_writer,
            std_tx,
        })
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
        if self.level == LogLevel::OFF {
            return;
        }

        if !self.show_std && self.file_writer.is_none() {
            return;
        }

        let level = *event.metadata().level();

        if self.level < level {
            return;
        }

        let mut visitor = LogVisitor(None);
        event.record(&mut visitor);
        let message = visitor.0.unwrap_or_default();

        static FIXED_OFFSET: chrono::FixedOffset =
            chrono::FixedOffset::east_opt(8 * 3600).expect("创建时区偏移失败");
        let now = chrono::Utc::now()
            .with_timezone(&FIXED_OFFSET)
            .format("%Y%m%d_%H%M%S_%6f")
            .to_string();

        let topic = match level {
            Level::TRACE => "轨迹",
            Level::DEBUG => "调试",
            Level::INFO => "消息",
            Level::WARN => "警告",
            Level::ERROR => "错误",
        };

        let target = event
            .metadata()
            .target()
            .split("::")
            .last()
            .unwrap_or_default();
        let line = event.metadata().line().unwrap_or(0);

        let thread_id = std::thread::current().id().as_u64();

        let msg = format!("[{topic}][{now}][{thread_id:02}][{target}:{line:04}]> {message}\n");

        if self.show_std {
            self.std_tx
                .send(Some(
                    match level {
                        Level::TRACE => Color::DarkGray.paint(&msg),
                        Level::DEBUG => Color::Blue.paint(&msg),
                        Level::INFO => Color::Green.paint(&msg),
                        Level::WARN => Color::Purple.paint(&msg),
                        Level::ERROR => Color::Red.paint(&msg),
                    }
                    .to_string(),
                ))
                .ok();
        }

        if let Some(writer) = &self.file_writer {
            let mut writer = writer.clone();
            writer.write_all(msg.as_bytes()).ok();
        }
    }
}

pub struct LogCollector {
    tx: UnboundedSender<Option<String>>,
    handle: JoinHandle<()>,
}

impl LogCollector {
    fn new(tx: UnboundedSender<Option<String>>, handle: JoinHandle<()>) -> Self {
        Self { tx, handle }
    }
    pub fn done(self) {
        self.tx.send(None).expect("日志收集器关闭信号发送失败");
        self.handle.join().expect("日志收集器线程未完成");
    }
}

impl LogConfig {
    /// 初始化日志,将设置全局的日志配置.
    /// <br> 重复初始化会报错.
    pub fn init_log(self) -> Result<LogCollector> {
        let mut targets =
            tracing_subscriber::filter::Targets::new().with_target("destiny", LevelFilter::TRACE);

        for target in self.targets {
            targets = targets.with_target(target, LevelFilter::TRACE);
        }

        let (tx, mut rx) = unbounded_channel::<Option<String>>();

        let handle = std::thread::spawn(move || {
            while let Some(Some(msg)) = rx.blocking_recv() {
                std::io::stdout().write_all(msg.as_bytes()).ok();
            }
        });
        let log_collector = LogCollector::new(tx.clone(), handle);

        let layer = LogLayer::new(self.level, self.show_std, self.save_file, tx)?;

        let collector = tracing_subscriber::registry().with(targets).with(layer);

        tracing::subscriber::set_global_default(collector)?;

        Ok(log_collector)
    }
}
