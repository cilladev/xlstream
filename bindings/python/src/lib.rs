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
#![allow(clippy::used_underscore_binding)]

use std::path::PathBuf;

use pyo3::exceptions::{PyOSError, PyRuntimeError};
use pyo3::prelude::*;
use pyo3::types::PyDict;

use xlstream_core::XlStreamError as RustXlStreamError;

pyo3::create_exception!(
    xlstream,
    XlStreamError,
    pyo3::exceptions::PyException,
    "Base class for all xlstream errors."
);
pyo3::create_exception!(
    xlstream,
    UnsupportedFormula,
    XlStreamError,
    "A formula cannot be evaluated in streaming mode."
);
pyo3::create_exception!(
    xlstream,
    FormulaParseError,
    XlStreamError,
    "A formula could not be parsed."
);
pyo3::create_exception!(
    xlstream,
    ClassificationError,
    XlStreamError,
    "A formula could not be classified."
);
pyo3::create_exception!(
    xlstream,
    CircularReferenceError,
    XlStreamError,
    "Formula columns form a dependency cycle."
);

#[allow(clippy::needless_pass_by_value)]
fn to_pyerr(e: RustXlStreamError) -> PyErr {
    let msg = e.to_string();
    match e {
        RustXlStreamError::Io { .. }
        | RustXlStreamError::Xlsx(_)
        | RustXlStreamError::XlsxWrite(_) => PyOSError::new_err(msg),
        RustXlStreamError::Unsupported { .. } => UnsupportedFormula::new_err(msg),
        RustXlStreamError::FormulaParse { .. } => FormulaParseError::new_err(msg),
        RustXlStreamError::Classification { .. } => ClassificationError::new_err(msg),
        RustXlStreamError::CircularReference { .. } => CircularReferenceError::new_err(msg),
        RustXlStreamError::Internal(_) => PyRuntimeError::new_err(msg),
    }
}

/// Evaluate formulas in an xlsx workbook and write the results.
///
/// The GIL is released for the entire Rust evaluation so other Python
/// threads can run concurrently.
#[pyfunction]
#[pyo3(signature = (input_path, output_path, *, workers=None, iterative_calc=true, max_iterations=100, max_change=0.001))]
fn evaluate(
    py: Python<'_>,
    input_path: &str,
    output_path: &str,
    workers: Option<usize>,
    iterative_calc: bool,
    max_iterations: u32,
    max_change: f64,
) -> PyResult<Py<PyDict>> {
    let input = PathBuf::from(input_path);
    let output = PathBuf::from(output_path);
    let options = xlstream_eval::EvaluateOptions {
        workers,
        iterative_calc,
        max_iterations,
        max_change,
        values_only: false,
    };

    let summary =
        py.detach(move || xlstream_eval::evaluate(&input, &output, &options)).map_err(to_pyerr)?;

    let dict = PyDict::new(py);
    dict.set_item("rows_processed", summary.rows_processed)?;
    dict.set_item("formulas_evaluated", summary.formulas_evaluated)?;
    #[allow(clippy::cast_possible_truncation)]
    dict.set_item("duration_ms", summary.duration.as_millis() as u64)?;
    Ok(dict.unbind())
}

/// The native extension module for xlstream.
#[pymodule]
fn _xlstream(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(evaluate, m)?)?;
    m.add("XlStreamError", m.py().get_type::<XlStreamError>())?;
    m.add("UnsupportedFormula", m.py().get_type::<UnsupportedFormula>())?;
    m.add("FormulaParseError", m.py().get_type::<FormulaParseError>())?;
    m.add("ClassificationError", m.py().get_type::<ClassificationError>())?;
    m.add("CircularReferenceError", m.py().get_type::<CircularReferenceError>())?;
    Ok(())
}
