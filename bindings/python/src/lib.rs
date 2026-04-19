//! Python bindings for the xlstream streaming Excel evaluator.

#![warn(missing_docs, rust_2018_idioms, clippy::pedantic, clippy::cargo)]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::todo,
    clippy::unimplemented,
    clippy::print_stdout,
    clippy::dbg_macro
)]
#![allow(clippy::module_name_repetitions, clippy::cargo_common_metadata)]

use pyo3::prelude::*;

/// The native extension module for xlstream.
#[pymodule]
#[allow(clippy::unnecessary_wraps)]
fn _xlstream(_m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
