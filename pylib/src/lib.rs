#![feature(custom_attribute)]

use pyo3::prelude::*;


/// This module is a python module implemented in Rust.
#[pymodinit]
fn string_sum(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "sum_string")]
    /// Formats the sum of two numbers as string
    fn sum_as_string(a: i64, b: i64) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    Ok(())
}
