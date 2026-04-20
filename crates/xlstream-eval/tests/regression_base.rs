//! Per-bug regression tests.
//!
//! Each issue gets its own file under `regressions/`. Tests land BEFORE
//! the fix (`#[ignore]`), fix un-ignores them.
//!
//! To run all (including ignored):
//! ```sh
//! cargo test -p xlstream-eval --test regression_base -- --include-ignored
//! ```
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss
)]

mod regressions;
