use once_cell::sync::Lazy;
use tokio::runtime::{Builder, Runtime};

pub static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("初始化运行线程池失败")
});
