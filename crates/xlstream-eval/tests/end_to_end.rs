//! End-to-end integration tests — full evaluate pipeline.
//!
//! Each module tests a specific feature through the complete path:
//! read xlsx → parse → classify → prelude → evaluate → write → verify output.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_lossless,
    clippy::needless_range_loop
)]

#[path = "end_to_end/helpers/mod.rs"]
mod helpers;

#[path = "end_to_end/cross_sheet_aggregates.rs"]
mod cross_sheet_aggregates;
#[path = "end_to_end/grouped_sumif.rs"]
mod grouped_sumif;
#[path = "end_to_end/lookups.rs"]
mod lookups;
#[path = "end_to_end/named_ranges.rs"]
mod named_ranges;
#[path = "end_to_end/pipeline.rs"]
mod pipeline;
#[path = "end_to_end/product_literals.rs"]
mod product_literals;
