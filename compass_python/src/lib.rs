use pyo3::prelude::*;

use pyo3_stub_gen::define_stub_info_gatherer;

use pyo3_stub_gen::derive::gen_stub_pyfunction;

/// Formats the sum of two numbers as string.
#[gen_stub_pyfunction(module = "ospl_rcompass._rust_lib")]
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn _rust_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

// Define a function to gather stub information.
define_stub_info_gatherer!(stub_info);
