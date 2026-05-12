//! # xlstream
//!
//! Streaming Excel formula evaluation engine. Reads `.xlsx` files row-by-row,
//! evaluates formulas in bounded memory, writes results to a new `.xlsx`.
//!
//! This crate re-exports the primary entry points from [`xlstream_eval`]
//! and [`xlstream_core`]. For direct access to internals, depend on those
//! crates instead.
//!
//! # Examples
//!
//! ```no_run
//! use std::path::Path;
//!
//! let summary = xlstream::evaluate(
//!     Path::new("input.xlsx"),
//!     Path::new("output.xlsx"),
//!     &xlstream::EvaluateOptions::default(),
//! )?;
//!
//! assert!(summary.rows_processed > 0);
//! # Ok::<(), xlstream::XlStreamError>(())
//! ```

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
#![allow(clippy::multiple_crate_versions)]

pub use xlstream_core::{
    CellError, EvaluateOptions, ExcelDate, OutputMode, Value, XlStreamError,
    ITERATIVE_CALC_DEFAULT_MAX_CHANGE, ITERATIVE_CALC_DEFAULT_MAX_ITERATIONS,
};
pub use xlstream_eval::{evaluate, EvaluateSummary};
