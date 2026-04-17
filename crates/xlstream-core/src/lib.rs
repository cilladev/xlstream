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

mod cell_error;
mod date;
mod error;
mod value;

pub use cell_error::CellError;
pub use date::ExcelDate;
pub use error::XlStreamError;
pub use value::Value;
