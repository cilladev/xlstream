//! Regression: PRODUCT with literal args returns #VALUE!.
//!
//! `PRODUCT(2, 3, 4)` should return 24 but returns #VALUE! because
//! PRODUCT is classified as Aggregate and the evaluator expects a
//! prelude ref. Scalar-only args should classify as `RowLocal`.
//!
//! See: issue.md section 3.
#![allow(clippy::float_cmp)]

use rust_xlsxwriter::{Formula, Workbook};
use tempfile::TempDir;
use xlstream_core::Value;
use xlstream_eval::evaluate;

#[test]
fn product_literal_args() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    ws.set_name("Sheet1").unwrap();
    ws.write_string(0, 0, "A").unwrap();
    ws.write_string(0, 1, "Result").unwrap();
    ws.write_number(1, 0, 1.0).unwrap();
    ws.write_formula(1, 1, Formula::new("=PRODUCT(2,3,4)")).unwrap();

    wb.save(&input).unwrap();
    evaluate(&input, &output, None).unwrap();

    let mut reader = xlstream_io::Reader::open(&output).unwrap();
    let mut stream = reader.cells("Sheet1").unwrap();
    let _ = stream.next_row().unwrap(); // header
    let (_, row) = stream.next_row().unwrap().unwrap();

    assert_eq!(row[1], Value::Number(24.0), "PRODUCT(2,3,4) = 24");
}

#[test]
fn product_cell_ref_args() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    ws.set_name("Sheet1").unwrap();
    ws.write_string(0, 0, "A").unwrap();
    ws.write_string(0, 1, "B").unwrap();
    ws.write_string(0, 2, "Result").unwrap();
    ws.write_number(1, 0, 5.0).unwrap();
    ws.write_number(1, 1, 6.0).unwrap();
    ws.write_formula(1, 2, Formula::new("=PRODUCT(A2,B2)")).unwrap();

    wb.save(&input).unwrap();
    evaluate(&input, &output, None).unwrap();

    let mut reader = xlstream_io::Reader::open(&output).unwrap();
    let mut stream = reader.cells("Sheet1").unwrap();
    let _ = stream.next_row().unwrap(); // header
    let (_, row) = stream.next_row().unwrap().unwrap();

    assert_eq!(row[2], Value::Number(30.0), "PRODUCT(A2,B2) = 30");
}
