use destiny_engine::prelude::*;
use pyo3::prelude::*;

#[pymodule]
fn destiny(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<Kline>()?;
    Ok(())
}
