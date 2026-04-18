//! # xlstream-parse
//!
//! Formula parser adapter. Wraps [`formualizer-parse`] and adds a
//! classification layer that labels each AST with a streaming verdict.
//!
//! This crate exposes stub types in Phase 1; real parse + classify logic
//! lands in Phase 2. See `docs/phases/phase-02-parser.md`.
//!
//! [`formualizer-parse`]: https://docs.rs/formualizer-parse

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

mod ast;
mod classify;
mod parser;
mod references;

pub use ast::Ast;
pub use classify::{classify, Classification, ClassificationContext};
pub use parser::parse;
pub use references::{extract_references, Reference, References};
