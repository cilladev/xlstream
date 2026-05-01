//! # xlstream-parse
//!
//! Formula parser adapter. Wraps [`formualizer-parse`] and adds a
//! classification layer that labels each AST with a streaming verdict.
//!
//! Wraps the upstream parser, adds reference extraction, named range
//! resolution, classification, and AST rewriting for prelude refs.
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
mod resolve;
mod resolve_tables;
pub mod rewrite;
pub mod sets;
mod view;

pub use ast::Ast;
pub use classify::{classify, Classification, ClassificationContext, UnsupportedReason};
pub use parser::parse;
pub use references::{extract_references, Reference, References};
pub use resolve::resolve_named_ranges;
pub use resolve_tables::{resolve_table_references, TableInfo};
pub use rewrite::{
    collect_lookup_keys, rewrite, AggKind, AggregateKey, LookupKey, LookupKind, PreludeKey,
};
pub use view::{NodeRef, NodeView};
