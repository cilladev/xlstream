//! End-to-end tests for named range resolution through the full evaluate pipeline.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_lossless
)]

use rust_xlsxwriter::{Formula, Workbook};
use tempfile::TempDir;
use xlstream_core::Value;
use xlstream_eval::evaluate;
use xlstream_io::Reader;

// ---------------------------------------------------------------------------
// Named range -> range (aggregate)
// ---------------------------------------------------------------------------

#[test]
fn named_range_aggregate_pct_of_total() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet().set_name("Data").unwrap();

    ws.write_string(0, 0, "Value").unwrap();
    ws.write_string(0, 1, "Pct").unwrap();

    let values = [100.0, 200.0, 300.0, 400.0];
    let total: f64 = values.iter().sum(); // 1000.0

    for (i, &v) in values.iter().enumerate() {
        let row = (i + 1) as u32;
        let excel_row = row + 1;
        ws.write_number(row, 0, v).unwrap();
        let pct = v / total * 100.0;
        ws.write_formula(
            row,
            1,
            Formula::new(format!("=A{excel_row}/SUM(SalesData)*100")).set_result(pct.to_string()),
        )
        .unwrap();
    }

    wb.define_name("SalesData", "=Data!$A:$A").unwrap();
    wb.save(&input).unwrap();

    let summary = evaluate(&input, &output, None).unwrap();
    assert_eq!(summary.formulas_evaluated, 4);

    let mut reader = Reader::open(&output).unwrap();
    let mut stream = reader.cells("Data").unwrap();
    let _ = stream.next_row().unwrap(); // skip header

    let expected_pcts = [10.0, 20.0, 30.0, 40.0];
    for (i, &expected) in expected_pcts.iter().enumerate() {
        let (_, row) = stream.next_row().unwrap().unwrap();
        match &row[1] {
            Value::Number(n) => {
                assert!(
                    (n - expected).abs() < 1e-10,
                    "row {}: expected {expected}, got {n}",
                    i + 1
                );
            }
            other => panic!("row {}: expected Number, got {other:?}", i + 1),
        }
    }
}

// ---------------------------------------------------------------------------
// Named range -> constant (row-local)
// ---------------------------------------------------------------------------

#[test]
fn named_range_constant_in_formula() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet().set_name("Sheet1").unwrap();

    ws.write_string(0, 0, "Amount").unwrap();
    ws.write_string(0, 1, "Tax").unwrap();

    let amounts = [100.0, 200.0, 500.0];
    for (i, &amt) in amounts.iter().enumerate() {
        let row = (i + 1) as u32;
        let excel_row = row + 1;
        ws.write_number(row, 0, amt).unwrap();
        let tax = amt * 0.15;
        ws.write_formula(
            row,
            1,
            Formula::new(format!("=A{excel_row}*TaxRate")).set_result(tax.to_string()),
        )
        .unwrap();
    }

    wb.define_name("TaxRate", "=0.15").unwrap();
    wb.save(&input).unwrap();

    let summary = evaluate(&input, &output, None).unwrap();
    assert_eq!(summary.formulas_evaluated, 3);

    let mut reader = Reader::open(&output).unwrap();
    let mut stream = reader.cells("Sheet1").unwrap();
    let _ = stream.next_row().unwrap(); // skip header

    for (i, &amt) in amounts.iter().enumerate() {
        let (_, row) = stream.next_row().unwrap().unwrap();
        let expected = amt * 0.15;
        match &row[1] {
            Value::Number(n) => {
                assert!(
                    (n - expected).abs() < 1e-10,
                    "row {}: expected {expected}, got {n}",
                    i + 1
                );
            }
            other => panic!("row {}: expected Number, got {other:?}", i + 1),
        }
    }
}

// ---------------------------------------------------------------------------
// Named range -> lookup table_array
// ---------------------------------------------------------------------------

#[test]
fn named_range_vlookup_table_array() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();

    // Lookup sheet
    let ws_lookup = wb.add_worksheet().set_name("Rates").unwrap();
    ws_lookup.write_string(0, 0, "EMEA").unwrap();
    ws_lookup.write_number(0, 1, 0.10).unwrap();
    ws_lookup.write_string(1, 0, "APAC").unwrap();
    ws_lookup.write_number(1, 1, 0.12).unwrap();
    ws_lookup.write_string(2, 0, "AMER").unwrap();
    ws_lookup.write_number(2, 1, 0.08).unwrap();

    // Main sheet
    let ws_main = wb.add_worksheet().set_name("Main").unwrap();
    ws_main.write_string(0, 0, "Region").unwrap();
    ws_main.write_string(0, 1, "Rate").unwrap();

    let regions = ["EMEA", "APAC", "AMER"];
    for (i, &region) in regions.iter().enumerate() {
        let row = (i + 1) as u32;
        let excel_row = row + 1;
        ws_main.write_string(row, 0, region).unwrap();
        ws_main
            .write_formula(
                row,
                1,
                Formula::new(format!("VLOOKUP(A{excel_row}, RateTable, 2, FALSE)")),
            )
            .unwrap();
    }

    wb.define_name("RateTable", "=Rates!$A:$B").unwrap();
    wb.save(&input).unwrap();

    let summary = evaluate(&input, &output, None).unwrap();
    assert_eq!(summary.formulas_evaluated, 3);

    let mut reader = Reader::open(&output).unwrap();
    let mut stream = reader.cells("Main").unwrap();
    let _ = stream.next_row().unwrap(); // skip header

    let expected_rates = [0.10, 0.12, 0.08];
    for (i, &expected) in expected_rates.iter().enumerate() {
        let (_, row) = stream.next_row().unwrap().unwrap();
        match &row[1] {
            Value::Number(n) => {
                assert!(
                    (n - expected).abs() < 1e-10,
                    "row {}: expected {expected}, got {n}",
                    i + 1
                );
            }
            other => panic!("row {}: expected Number, got {other:?}", i + 1),
        }
    }
}

// ---------------------------------------------------------------------------
// Named range in SUMIF (aggregate with named range args)
// ---------------------------------------------------------------------------

#[test]
fn named_range_sumif_both_args() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet().set_name("Data").unwrap();

    ws.write_string(0, 0, "Region").unwrap();
    ws.write_string(0, 1, "Amount").unwrap();
    ws.write_string(0, 2, "EMEA Total").unwrap();

    let data: &[(&str, f64)] =
        &[("EMEA", 100.0), ("APAC", 200.0), ("EMEA", 300.0), ("AMER", 400.0)];
    // EMEA total = 100 + 300 = 400
    for (i, &(region, amount)) in data.iter().enumerate() {
        let row = (i + 1) as u32;
        ws.write_string(row, 0, region).unwrap();
        ws.write_number(row, 1, amount).unwrap();
        ws.write_formula(
            row,
            2,
            Formula::new("SUMIF(RegionCol, \"EMEA\", AmountCol)").set_result("400"),
        )
        .unwrap();
    }

    wb.define_name("RegionCol", "=Data!$A:$A").unwrap();
    wb.define_name("AmountCol", "=Data!$B:$B").unwrap();
    wb.save(&input).unwrap();

    let summary = evaluate(&input, &output, None).unwrap();
    assert_eq!(summary.formulas_evaluated, 4);

    let mut reader = Reader::open(&output).unwrap();
    let mut stream = reader.cells("Data").unwrap();
    let _ = stream.next_row().unwrap(); // skip header

    for i in 0..4 {
        let (_, row) = stream.next_row().unwrap().unwrap();
        match &row[2] {
            Value::Number(n) => {
                assert!((n - 400.0).abs() < 1e-10, "row {}: expected 400, got {n}", i + 1);
            }
            other => panic!("row {}: expected Number, got {other:?}", i + 1),
        }
    }
}

// ---------------------------------------------------------------------------
// Regression: no named ranges -> unchanged behavior
// ---------------------------------------------------------------------------

#[test]
fn no_named_ranges_regression_guard() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet().set_name("Sheet1").unwrap();

    ws.write_string(0, 0, "A").unwrap();
    ws.write_string(0, 1, "B").unwrap();
    ws.write_string(0, 2, "C").unwrap();

    for i in 0..3u32 {
        let row = i + 1;
        let excel_row = row + 1;
        let a_val = (i + 1) as f64 * 10.0;
        let b_val = (i + 1) as f64 * 20.0;
        ws.write_number(row, 0, a_val).unwrap();
        ws.write_number(row, 1, b_val).unwrap();
        ws.write_formula(
            row,
            2,
            Formula::new(format!("=A{excel_row}+B{excel_row}"))
                .set_result((a_val + b_val).to_string()),
        )
        .unwrap();
    }

    // No define_name calls -- workbook has zero named ranges.
    wb.save(&input).unwrap();

    let summary = evaluate(&input, &output, None).unwrap();
    assert_eq!(summary.rows_processed, 4);
    assert_eq!(summary.formulas_evaluated, 3);

    let mut reader = Reader::open(&output).unwrap();
    let mut stream = reader.cells("Sheet1").unwrap();
    let _ = stream.next_row().unwrap(); // skip header

    for i in 1..=3u32 {
        let (_, row) = stream.next_row().unwrap().unwrap();
        let expected = i as f64 * 10.0 + i as f64 * 20.0;
        assert_eq!(row[2], Value::Number(expected), "row {i}");
    }
}
