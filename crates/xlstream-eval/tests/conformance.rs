#![allow(clippy::all, clippy::pedantic)]

#[path = "conformance/mod.rs"]
mod conformance;

#[path = "conformance/aggregate.rs"]
mod aggregate;
#[path = "conformance/date.rs"]
mod date;
#[path = "conformance/financial.rs"]
mod financial;
#[path = "conformance/info.rs"]
mod info;
#[path = "conformance/issues.rs"]
mod issues;
#[path = "conformance/logical.rs"]
mod logical;
#[path = "conformance/lookup.rs"]
mod lookup;
#[path = "conformance/math.rs"]
mod math;
#[path = "conformance/operators.rs"]
mod operators;
#[path = "conformance/text.rs"]
mod text;
