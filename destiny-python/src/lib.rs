use destiny_engine::prelude::*;
use pyo3::{prelude::*, types::PyTuple};

#[pymodule]
fn destiny(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<TradeType>()?;
    m.add_class::<TradeSide>()?;
    m.add_class::<OrderStatus>()?;
    m.add_class::<Kline>()?;
    m.add_class::<Order>()?;
    m.add_class::<Position>()?;
    m.add_class::<PythonEngine>()?;
    m.add_function(wrap_pyfunction!(init_log, m)?)?;
    m.add_function(wrap_pyfunction!(free_log, m)?)?;
    m.add_function(wrap_pyfunction!(log_trace, m)?)?;
    m.add_function(wrap_pyfunction!(log_debug, m)?)?;
    m.add_function(wrap_pyfunction!(log_info, m)?)?;
    m.add_function(wrap_pyfunction!(log_warn, m)?)?;
    m.add_function(wrap_pyfunction!(log_error, m)?)?;
    m.add_function(wrap_pyfunction!(log_print, m)?)?;
    m.add_function(wrap_pyfunction!(run_backtest, m)?)?;
    Ok(())
}

#[pyfunction]
#[pyo3(
    name = "init_log",
    signature = (
        show_std = true,
        save_file = false,
        targets = vec![],
        level = "info"
    )
)]
fn init_log(show_std: bool, save_file: bool, targets: Vec<String>, level: &str) -> Result<usize> {
    let log_collector = LogConfigBuilder::default()
        .show_std(show_std)
        .save_file(save_file)
        .targets(targets)
        .level(match level {
            "trace" => LogLevel::TRACE,
            "debug" => LogLevel::DEBUG,
            "info" => LogLevel::INFO,
            "warn" => LogLevel::WARN,
            "error" => LogLevel::ERROR,
            _ => LogLevel::INFO,
        })
        .build()?
        .init_log()?;

    info!(
        "\n\n\n{}\t    Author : {}\n\t   Version : {}\n\tRepository : {}\n\n\n",
        LOGO, AUTHOR, VERSION, REPOSITORY
    );

    Ok(Box::into_raw(Box::new(log_collector)) as usize)
}

#[pyfunction]
#[pyo3(
    name = "free_log",
    signature = (log_collector)
)]
fn free_log(log_collector: usize) {
    let ptr = log_collector as *mut LogCollector;
    if ptr.is_null() {
        return;
    }
    let log_collector = unsafe { Box::from_raw(ptr) };
    log_collector.done();
}

#[pyfunction]
#[pyo3(
    name="trace", 
    signature = (*args)
)]
fn log_trace(args: &Bound<'_, PyTuple>) {
    trace!(
        "策略 : {}",
        args.iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

#[pyfunction]
#[pyo3(
    name="debug", 
    signature = (*args)
)]
fn log_debug(args: &Bound<'_, PyTuple>) {
    debug!(
        "策略 : {}",
        args.iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

#[pyfunction]
#[pyo3(
    name="info", 
    signature = (*args)
)]
fn log_info(args: &Bound<'_, PyTuple>) {
    info!(
        "策略 : {}",
        args.iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

#[pyfunction]
#[pyo3(
    name="warn", 
    signature = (*args)
)]
fn log_warn(args: &Bound<'_, PyTuple>) {
    warn!(
        "策略 : {}",
        args.iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

#[pyfunction]
#[pyo3(
    name="error", 
    signature = (*args)
)]
fn log_error(args: &Bound<'_, PyTuple>) {
    error!(
        "策略 : {}",
        args.iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

#[pyfunction]
#[pyo3(
    name="print", 
    signature = (*args)
)]
fn log_print(args: &Bound<'_, PyTuple>) {
    log_debug(args);
}

#[pyclass(name = "API", frozen)]
struct PythonEngine(Arc<dyn Engine>);

#[pymethods]
impl PythonEngine {
    #[pyo3(signature = ())]
    fn time(&self) -> DateTime<Utc> {
        self.0.time()
    }

    #[pyo3(signature = ())]
    fn stop(&self) {
        self.0.stop();
    }

    #[pyo3(signature = (symbol))]
    fn init_symbol(&self, symbol: &str) -> Result<()> {
        self.0.symbol_init(symbol)
    }

    #[pyo3(signature = (symbol, id))]
    fn order(&self, symbol: &str, id: &str) -> Option<Order> {
        self.0.order(symbol, id)
    }

    #[pyo3(signature = (symbol))]
    fn orders(&self, symbol: &str) -> Vec<Order> {
        self.0.orders(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn orders_long(&self, symbol: &str) -> Vec<Order> {
        self.0.orders_long(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn orders_long_open(&self, symbol: &str) -> Vec<Order> {
        self.0.orders_long_open(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn orders_long_close(&self, symbol: &str) -> Vec<Order> {
        self.0.orders_long_close(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn orders_short(&self, symbol: &str) -> Vec<Order> {
        self.0.orders_short(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn orders_short_open(&self, symbol: &str) -> Vec<Order> {
        self.0.orders_short_open(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn orders_short_close(&self, symbol: &str) -> Vec<Order> {
        self.0.orders_short_close(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn leverage(&self, symbol: &str) -> u32 {
        self.0.leverage(symbol)
    }

    #[pyo3(signature = ())]
    fn cash(&self) -> Decimal {
        self.0.cash()
    }

    #[pyo3(signature = ())]
    fn cash_available(&self) -> Decimal {
        self.0.cash_available()
    }

    #[pyo3(signature = ())]
    fn cash_frozen(&self) -> Decimal {
        self.0.cash_frozen()
    }

    #[pyo3(signature = ())]
    fn margin(&self) -> Decimal {
        self.0.margin()
    }

    #[pyo3(signature = ())]
    fn pnl(&self) -> Decimal {
        self.0.pnl()
    }

    #[pyo3(signature = (symbol))]
    fn long_price(&self, symbol: &str) -> Decimal {
        self.0.long_price(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn long_size(&self, symbol: &str) -> Decimal {
        self.0.long_size(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn long_size_available(&self, symbol: &str) -> Decimal {
        self.0.long_size_available(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn long_size_frozen(&self, symbol: &str) -> Decimal {
        self.0.long_size_frozen(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn long_margin(&self, symbol: &str) -> Decimal {
        self.0.long_margin(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn long_pnl(&self, symbol: &str) -> Decimal {
        self.0.long_pnl(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn short_price(&self, symbol: &str) -> Decimal {
        self.0.short_price(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn short_size(&self, symbol: &str) -> Decimal {
        self.0.short_size(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn short_size_available(&self, symbol: &str) -> Decimal {
        self.0.short_size_available(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn short_size_frozen(&self, symbol: &str) -> Decimal {
        self.0.short_size_frozen(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn short_margin(&self, symbol: &str) -> Decimal {
        self.0.short_margin(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn short_pnl(&self, symbol: &str) -> Decimal {
        self.0.short_pnl(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn symbol_pnl(&self, symbol: &str) -> Decimal {
        self.0.symbol_pnl(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn symbol_margin(&self, symbol: &str) -> Decimal {
        self.0.symbol_margin(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn price_mark(&self, symbol: &str) -> Decimal {
        self.0.price_mark(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn price_last(&self, symbol: &str) -> Decimal {
        self.0.price_last(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn price_index(&self, symbol: &str) -> Decimal {
        self.0.price_index(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn price_settlement(&self, symbol: &str) -> Decimal {
        self.0.price_settlement(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn time_settlement(&self, symbol: &str) -> DateTime<Utc> {
        self.0.time_settlement(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn rule_price_min(&self, symbol: &str) -> Decimal {
        self.0.rule_price_min(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn rule_price_max(&self, symbol: &str) -> Decimal {
        self.0.rule_price_max(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn rule_price_tick(&self, symbol: &str) -> Decimal {
        self.0.rule_price_tick(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn rule_size_min(&self, symbol: &str) -> Decimal {
        self.0.rule_size_min(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn rule_size_max(&self, symbol: &str) -> Decimal {
        self.0.rule_size_max(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn rule_size_tick(&self, symbol: &str) -> Decimal {
        self.0.rule_size_tick(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn rule_amount_min(&self, symbol: &str) -> Decimal {
        self.0.rule_amount_min(symbol)
    }

    #[pyo3(signature = (symbol))]
    fn rule_order_max(&self, symbol: &str) -> i64 {
        self.0.rule_order_max(symbol)
    }

    #[pyo3(signature = (symbol, size))]
    fn long_market_open(&self, symbol: &str, size: Decimal) -> Result<String> {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current()
                .block_on(async move { self.0.long_market_open(symbol, size).await })
        })
    }

    #[pyo3(signature = (symbol, size, price))]
    fn long_limit_open(&self, symbol: &str, size: Decimal, price: Decimal) -> Result<String> {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current()
                .block_on(async move { self.0.long_limit_open(symbol, size, price).await })
        })
    }

    #[pyo3(signature = (symbol, size))]
    fn long_market_close(&self, symbol: &str, size: Decimal) -> Result<String> {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current()
                .block_on(async move { self.0.long_market_close(symbol, size).await })
        })
    }

    #[pyo3(signature = (symbol, size, price))]
    fn long_limit_close(&self, symbol: &str, size: Decimal, price: Decimal) -> Result<String> {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current()
                .block_on(async move { self.0.long_limit_close(symbol, size, price).await })
        })
    }

    #[pyo3(signature = (symbol, size))]
    fn short_market_open(&self, symbol: &str, size: Decimal) -> Result<String> {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current()
                .block_on(async move { self.0.short_market_open(symbol, size).await })
        })
    }

    #[pyo3(signature = (symbol, size, price))]
    fn short_limit_open(&self, symbol: &str, size: Decimal, price: Decimal) -> Result<String> {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current()
                .block_on(async move { self.0.short_limit_open(symbol, size, price).await })
        })
    }

    #[pyo3(signature = (symbol, size))]
    fn short_market_close(&self, symbol: &str, size: Decimal) -> Result<String> {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current()
                .block_on(async move { self.0.short_market_close(symbol, size).await })
        })
    }

    #[pyo3(signature = (symbol, size, price))]
    fn short_limit_close(&self, symbol: &str, size: Decimal, price: Decimal) -> Result<String> {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current()
                .block_on(async move { self.0.short_limit_close(symbol, size, price).await })
        })
    }

    #[pyo3(signature = (symbol, id))]
    fn order_close(&self, symbol: &str, id: &str) -> Result<()> {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current()
                .block_on(async move { self.0.order_close(symbol, id).await })
        })
    }

    #[pyo3(signature = (symbol, ids))]
    fn order_cancel_many(&self, symbol: &str, ids: Vec<String>) -> Result<()> {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current()
                .block_on(async move { self.0.order_cancel_many(symbol, &ids).await })
        })
    }

    #[pyo3(signature = (symbol, leverage))]
    fn leverage_set(&self, symbol: &str, leverage: u32) -> Result<()> {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current()
                .block_on(async move { self.0.leverage_set(symbol, leverage).await })
        })
    }
}

struct PythonStrategy {
    on_init: Option<Py<PyAny>>,
    on_start: Option<Py<PyAny>>,
    on_stop: Option<Py<PyAny>>,
    on_daily: Option<Py<PyAny>>,
    on_hourly: Option<Py<PyAny>>,
    on_minutely: Option<Py<PyAny>>,
    on_kline: Option<Py<PyAny>>,
    on_order: Option<Py<PyAny>>,
    on_position: Option<Py<PyAny>>,
}

impl PythonStrategy {
    #[allow(clippy::too_many_arguments)]
    fn new(
        on_init: Option<Py<PyAny>>,
        on_start: Option<Py<PyAny>>,
        on_stop: Option<Py<PyAny>>,
        on_daily: Option<Py<PyAny>>,
        on_hourly: Option<Py<PyAny>>,
        on_minutely: Option<Py<PyAny>>,
        on_kline: Option<Py<PyAny>>,
        on_order: Option<Py<PyAny>>,
        on_position: Option<Py<PyAny>>,
    ) -> Self {
        Self {
            on_init,
            on_start,
            on_stop,
            on_daily,
            on_hourly,
            on_minutely,
            on_kline,
            on_order,
            on_position,
        }
    }
}

#[async_trait]
impl Strategy for PythonStrategy {
    async fn on_init(&self, engine: Arc<dyn Engine>) -> Result<()> {
        if let Some(callback) = &self.on_init {
            Python::with_gil(|py| {
                callback.call1(py, (PythonEngine(engine),))?;
                anyhow::Ok(())
            })?;
        }
        Ok(())
    }
    async fn on_start(&self, engine: Arc<dyn Engine>) -> Result<()> {
        if let Some(callback) = &self.on_start {
            Python::with_gil(|py| {
                callback.call1(py, (PythonEngine(engine),))?;
                anyhow::Ok(())
            })?;
        }
        Ok(())
    }
    async fn on_stop(&self, engine: Arc<dyn Engine>) -> Result<()> {
        if let Some(callback) = &self.on_stop {
            Python::with_gil(|py| {
                callback.call1(py, (PythonEngine(engine),))?;
                anyhow::Ok(())
            })?;
        }
        Ok(())
    }
    async fn on_daily(&self, engine: Arc<dyn Engine>) -> Result<()> {
        if let Some(callback) = &self.on_daily {
            Python::with_gil(|py| {
                callback.call1(py, (PythonEngine(engine),))?;
                anyhow::Ok(())
            })?;
        }
        Ok(())
    }
    async fn on_hourly(&self, engine: Arc<dyn Engine>) -> Result<()> {
        if let Some(callback) = &self.on_hourly {
            Python::with_gil(|py| {
                callback.call1(py, (PythonEngine(engine),))?;
                anyhow::Ok(())
            })?;
        }
        Ok(())
    }
    async fn on_minutely(&self, engine: Arc<dyn Engine>) -> Result<()> {
        if let Some(callback) = &self.on_minutely {
            Python::with_gil(|py| {
                callback.call1(py, (PythonEngine(engine),))?;
                anyhow::Ok(())
            })?;
        }
        Ok(())
    }
    async fn on_kline(&self, engine: Arc<dyn Engine>, kline: Kline) -> Result<()> {
        if let Some(callback) = &self.on_kline {
            Python::with_gil(|py| {
                callback.call1(py, (PythonEngine(engine), kline))?;
                anyhow::Ok(())
            })?;
        }
        Ok(())
    }
    async fn on_order(&self, engine: Arc<dyn Engine>, order: Order) -> Result<()> {
        if let Some(callback) = &self.on_order {
            Python::with_gil(|py| {
                callback.call1(py, (PythonEngine(engine), order))?;
                anyhow::Ok(())
            })?;
        }
        Ok(())
    }
    async fn on_position(&self, engine: Arc<dyn Engine>, position: Position) -> Result<()> {
        if let Some(callback) = &self.on_position {
            Python::with_gil(|py| {
                callback.call1(py, (PythonEngine(engine), position))?;
                anyhow::Ok(())
            })?;
        }
        Ok(())
    }
}

#[pyfunction]
#[pyo3(
    name="run_backtest", 
    signature = (
        begin,
        end,
        cash = dec!(1000),
        fee_rate_taker = dec!(0.0005),
        fee_rate_maker = dec!(0.0005),
        slippage_rate = dec!(0.01),
        on_init = None,
        on_start = None,
        on_stop = None,
        on_daily = None,
        on_hourly = None,
        on_minutely = None,
        on_kline = None,
        on_order = None,
        on_position = None,
    )
)]
#[allow(clippy::too_many_arguments)]
fn run_backtest(
    py: Python<'_>,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
    cash: Decimal,
    fee_rate_taker: Decimal,
    fee_rate_maker: Decimal,
    slippage_rate: Decimal,
    on_init: Option<Py<PyAny>>,
    on_start: Option<Py<PyAny>>,
    on_stop: Option<Py<PyAny>>,
    on_daily: Option<Py<PyAny>>,
    on_hourly: Option<Py<PyAny>>,
    on_minutely: Option<Py<PyAny>>,
    on_kline: Option<Py<PyAny>>,
    on_order: Option<Py<PyAny>>,
    on_position: Option<Py<PyAny>>,
) -> Result<()> {
    py.allow_threads(|| {
        RUNTIME.block_on(async move {
            Backtest::run(
                BacktestConfigBuilder::default()
                    .begin(begin)
                    .end(end)
                    .cash(cash)
                    .fee_rate_taker(fee_rate_taker)
                    .fee_rate_maker(fee_rate_maker)
                    .slippage_rate(slippage_rate)
                    .build()?,
                Arc::new(PythonStrategy::new(
                    on_init,
                    on_start,
                    on_stop,
                    on_daily,
                    on_hourly,
                    on_minutely,
                    on_kline,
                    on_order,
                    on_position,
                )),
            )
            .await?;
            anyhow::Ok(())
        })
    })
}
