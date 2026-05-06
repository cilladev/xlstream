//! # xlstream-io
//!
//! xlsx I/O. Calamine-backed [`Reader`], `rust_xlsxwriter`-backed [`Writer`],
//! and a row-oriented [`CellStream`] abstraction.

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
// `clippy::multiple_crate_versions` fires because calamine and
// rust_xlsxwriter each pull different minor versions of transitive deps
// (`hashbrown`, `wit-bindgen`). We don't control upstream's dep graph; the
// Phase-1 stub doesn't use either crate's API so we can't avoid the pull-in.
#![allow(clippy::multiple_crate_versions)]

pub mod convert;
mod reader;
mod sheet_handle;
mod stream;
mod table_meta;
mod writer;

pub use reader::Reader;
pub use sheet_handle::SheetHandle;
pub use stream::CellStream;
pub use table_meta::TableMeta;
pub use writer::Writer;
