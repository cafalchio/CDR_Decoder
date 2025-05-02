use pyo3::prelude::*;
pub mod core;
pub mod data_blocks;
pub mod datatypes;

#[pyfunction]
fn sum_as_string_rs(a: usize, b: usize) -> PyResult<String> {
    println!("running from rust");
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn cdr_decoder(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string_rs, m)?)?;
    Ok(())
}
