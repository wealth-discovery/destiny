use anyhow::Result;
use destiny_engine::prelude::*;
use pyo3::{
    prelude::*,
    types::{PyCapsule, PyTuple},
};

#[pymodule]
fn destiny(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Kline>()?;
    m.add_function(wrap_pyfunction!(init_log, m)?)?;
    m.add_function(wrap_pyfunction!(free_log, m)?)?;
    m.add_function(wrap_pyfunction!(log_trace, m)?)?;
    m.add_function(wrap_pyfunction!(log_debug, m)?)?;
    m.add_function(wrap_pyfunction!(log_info, m)?)?;
    m.add_function(wrap_pyfunction!(log_warn, m)?)?;
    m.add_function(wrap_pyfunction!(log_error, m)?)?;
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
fn init_log(
    py: Python<'_>,
    show_std: bool,
    save_file: bool,
    targets: Vec<String>,
    level: &str,
) -> Result<Py<PyCapsule>> {
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

    Ok(PyCapsule::new(py, log_collector, None)?.unbind())
}

#[pyfunction]
#[pyo3(
    name = "free_log",
    signature = (log_collector)
)]
fn free_log(log_collector: Bound<PyCapsule>) {
    let ptr = log_collector.pointer();
    if ptr.is_null() {
        return;
    }
    let log_collector = unsafe { Box::from_raw(ptr as *mut LogCollector) };
    log_collector.done();
}

#[pyfunction]
#[pyo3(
    name="trace", 
    signature = (*args)
)]
pub fn log_trace(args: &Bound<'_, PyTuple>) {
    trace!(
        "{}",
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
pub fn log_debug(args: &Bound<'_, PyTuple>) {
    debug!(
        "{}",
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
pub fn log_info(args: &Bound<'_, PyTuple>) {
    info!(
        "{}",
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
pub fn log_warn(args: &Bound<'_, PyTuple>) {
    warn!(
        "{}",
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
pub fn log_error(args: &Bound<'_, PyTuple>) {
    error!(
        "{}",
        args.iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
