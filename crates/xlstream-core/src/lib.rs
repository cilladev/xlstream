//! # xlstream-core
//!
//! Foundation types for the xlstream streaming Excel evaluator. Owns the
//! [`Value`], [`CellError`], [`ExcelDate`], and [`XlStreamError`] types
//! that every other crate in the workspace depends on.
//!
//! See [`docs/architecture/crate-layout.md`] in the workspace for the
//! bigger picture.
//!
//! [`docs/architecture/crate-layout.md`]: https://github.com/cilladev/xlstream/blob/main/docs/architecture/crate-layout.md

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

mod address;
mod cell_error;
pub mod coerce;
mod date;
mod error;
mod value;

pub use address::col_row_to_a1;
pub use cell_error::CellError;
pub use date::ExcelDate;
pub use error::XlStreamError;
pub use value::Value;

/// Maximum row count in an Excel xlsx worksheet (2^20).
pub const EXCEL_MAX_ROWS: u64 = 1_048_576;
/// Maximum column count in an Excel xlsx worksheet (2^14).
pub const EXCEL_MAX_COLS: u16 = 16_384;

/// Minimum data rows on the main sheet before the evaluator spawns parallel
/// workers. Below this threshold single-threaded evaluation is faster.
pub const PARALLEL_ROW_THRESHOLD: u32 = 10_000;
