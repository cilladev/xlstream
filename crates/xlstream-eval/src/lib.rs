//! # xlstream-eval
//!
//! Streaming Excel formula evaluator. Phase 1 ships the public entry
//! point [`evaluate`] and the [`EvaluateSummary`] return type as stubs;
//! the real streaming engine, prelude, and builtin registry land in
//! Phase 4 and beyond.

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
// `clippy::multiple_crate_versions` fires because xlstream-io transitively
// pulls calamine and rust_xlsxwriter, which disagree on `hashbrown` and
// `wit-bindgen` minor versions. Ratified for xlstream-io in PR 2; the
// constraint propagates to any workspace crate that depends on xlstream-io.
#![allow(clippy::multiple_crate_versions)]

mod evaluate;
mod interp;
mod prelude;
mod scope;
pub mod topo;

pub use evaluate::{evaluate, EvaluateSummary};
pub use interp::Interpreter;
pub use prelude::Prelude;
pub use scope::RowScope;
pub use topo::topo_sort;
