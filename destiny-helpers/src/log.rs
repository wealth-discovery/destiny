use crate::path::PathBufSupport;
use anyhow::Result;
use derive_builder::Builder;
use nu_ansi_term::Color;
use std::{io::Write, path::PathBuf};
use tokio::{
    fs::create_dir_all,
    sync::mpsc::{unbounded_channel, UnboundedSender},
    task::JoinHandle,
};
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
    /// 可显示的包名,默认显示[`destiny_`]开头的包
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
    show_std: bool,
    file_writer: Option<tracing_appender::non_blocking::NonBlocking>,
    std_tx: UnboundedSender<Option<String>>,
}

impl LogLayer {
    pub async fn new(
        show_std: bool,
        save_file: bool,
        std_tx: UnboundedSender<Option<String>>,
    ) -> Result<Self> {
        let mut file_writer = None;
        if save_file {
            let dir = PathBuf::cache()?.join("logs");
            create_dir_all(&dir).await?;
            let appender = tracing_appender::rolling::daily(dir, "log");
            let (writer, guard) = tracing_appender::non_blocking(appender);
            file_writer = Some(writer);
            std::mem::forget(guard);
        }

        Ok(Self {
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
        if !self.show_std && self.file_writer.is_none() {
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

        let level = *event.metadata().level();

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
    pub async fn done(self) -> Result<()> {
        self.tx.send(None).ok();
        self.handle.await?;
        Ok(())
    }
}

impl LogConfig {
    /// 初始化日志,将设置全局的日志配置.
    /// <br> 重复初始化会报错.
    pub async fn init_log(self) -> Result<LogCollector> {
        let mut targets =
            tracing_subscriber::filter::Targets::new().with_target("destiny_", LevelFilter::TRACE);
        for target in self.targets {
            targets = targets.with_target(target, LevelFilter::TRACE);
        }

        let (tx, mut rx) = unbounded_channel::<Option<String>>();

        let handle = tokio::spawn(async move {
            while let Some(Some(msg)) = rx.recv().await {
                std::io::stdout().write_all(msg.as_bytes()).ok();
            }
        });
        let log_collector = LogCollector::new(tx.clone(), handle);

        let layer = LogLayer::new(self.show_std, self.save_file, tx).await?;

        let collector = tracing_subscriber::registry().with(targets).with(layer);

        tracing::subscriber::set_global_default(collector)?;

        Ok(log_collector)
    }
}
