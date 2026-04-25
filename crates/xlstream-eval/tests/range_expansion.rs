//! End-to-end tests for bounded range expansion through the full evaluate pipeline.
//!
//! Each test builds an xlsx fixture with range arguments (e.g. `A2:A6`),
//! runs it through `xlstream_eval::evaluate()`, reads back the output,
//! and asserts correctness.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss
)]

use std::path::Path;

use rust_xlsxwriter::{Formula, Workbook};
use tempfile::TempDir;
use xlstream_core::Value;
use xlstream_eval::evaluate;
use xlstream_io::Reader;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Read a single cell from the output workbook.
fn read_cell(output: &Path, sheet: &str, row: u32, col: u32) -> Value {
    let mut reader = Reader::open(output).unwrap();
    let mut stream = reader.cells(sheet).unwrap();
    while let Some((r, values)) = stream.next_row().unwrap() {
        if r == row {
            return values.get(col as usize).cloned().unwrap_or(Value::Empty);
        }
    }
    Value::Empty
}

/// Assert a numeric value is within `tolerance` of `expected`.
fn assert_approx(actual: &Value, expected: f64, tolerance: f64) {
    match actual {
        Value::Number(n) => {
            assert!((n - expected).abs() < tolerance, "expected ~{expected}, got {n}");
        }
        other => panic!("expected Number, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn irr_with_bounded_range_produces_correct_result() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet().set_name("Main").unwrap();

    // Header
    ws.write_string(0, 0, "Cashflow").unwrap();
    ws.write_string(0, 1, "IRR").unwrap();

    // Cashflows in A2:A6
    let cashflows = [-1000.0, 300.0, 400.0, 500.0, 200.0];
    for (i, &cf) in cashflows.iter().enumerate() {
        ws.write_number((i + 1) as u32, 0, cf).unwrap();
    }

    // B2 = IRR(A2:A6)
    ws.write_formula(1, 1, Formula::new("IRR(A2:A6)")).unwrap();

    wb.save(&input).unwrap();
    let summary = evaluate(&input, &output, None).unwrap();
    // Formula column B is evaluated for every data row (rows 1-5).
    assert_eq!(summary.formulas_evaluated, 5);

    let result = read_cell(&output, "Main", 1, 1);
    assert_approx(&result, 0.1532, 0.01);
}

#[test]
fn concat_with_bounded_range_joins_all_cells() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet().set_name("Main").unwrap();

    ws.write_string(0, 0, "Word").unwrap();
    ws.write_string(0, 1, "Result").unwrap();

    ws.write_string(1, 0, "Hello").unwrap();
    ws.write_string(2, 0, "World").unwrap();
    ws.write_string(3, 0, "!").unwrap();

    // B2 = CONCAT(A2:A4)
    ws.write_formula(1, 1, Formula::new("CONCAT(A2:A4)")).unwrap();

    wb.save(&input).unwrap();
    let summary = evaluate(&input, &output, None).unwrap();
    // Formula column B is evaluated for every data row (rows 1-3).
    assert_eq!(summary.formulas_evaluated, 3);

    let result = read_cell(&output, "Main", 1, 1);
    assert_eq!(result, Value::Text("HelloWorld!".into()));
}

#[test]
fn textjoin_with_bounded_range_joins_with_delimiter() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet().set_name("Main").unwrap();

    ws.write_string(0, 0, "Item").unwrap();
    ws.write_string(0, 1, "Result").unwrap();

    ws.write_string(1, 0, "a").unwrap();
    ws.write_string(2, 0, "b").unwrap();
    ws.write_string(3, 0, "c").unwrap();

    // B2 = TEXTJOIN(",", TRUE, A2:A4)
    ws.write_formula(1, 1, Formula::new("TEXTJOIN(\",\", TRUE, A2:A4)")).unwrap();

    wb.save(&input).unwrap();
    let summary = evaluate(&input, &output, None).unwrap();
    // Formula column B is evaluated for every data row (rows 1-3).
    assert_eq!(summary.formulas_evaluated, 3);

    let result = read_cell(&output, "Main", 1, 1);
    assert_eq!(result, Value::Text("a,b,c".into()));
}

#[test]
fn npv_with_bounded_range_produces_correct_result() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet().set_name("Main").unwrap();

    ws.write_string(0, 0, "Cashflow").unwrap();
    ws.write_string(0, 1, "NPV").unwrap();

    // Cashflows in A2:A5
    let cashflows = [-1000.0, 300.0, 400.0, 500.0];
    for (i, &cf) in cashflows.iter().enumerate() {
        ws.write_number((i + 1) as u32, 0, cf).unwrap();
    }

    // B2 = NPV(0.1, A2:A5)
    ws.write_formula(1, 1, Formula::new("NPV(0.1, A2:A5)")).unwrap();

    wb.save(&input).unwrap();
    let summary = evaluate(&input, &output, None).unwrap();
    // Formula column B is evaluated for every data row (rows 1-4).
    assert_eq!(summary.formulas_evaluated, 4);

    let result = read_cell(&output, "Main", 1, 1);
    assert_approx(&result, -19.12, 1.0);
}

#[test]
fn networkdays_with_holiday_range_from_lookup_sheet() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();

    // Lookup sheet "Holidays": two holidays on weekdays
    // Jan 9, 2026 (Fri) serial=46031, Jan 12, 2026 (Mon) serial=46034
    let ws_holidays = wb.add_worksheet().set_name("Holidays").unwrap();
    ws_holidays.write_number(0, 0, 46031.0).unwrap();
    ws_holidays.write_number(1, 0, 46034.0).unwrap();

    // Main sheet
    let ws = wb.add_worksheet().set_name("Main").unwrap();
    ws.write_string(0, 0, "Start").unwrap();
    ws.write_string(0, 1, "End").unwrap();
    ws.write_string(0, 2, "NetDays").unwrap();

    // Jan 5, 2026 (Mon) serial=46027 to Jan 16, 2026 (Fri) serial=46038
    // 10 weekdays total, minus 2 holidays = 8
    ws.write_number(1, 0, 46027.0).unwrap();
    ws.write_number(1, 1, 46038.0).unwrap();

    // C2 = NETWORKDAYS(A2, B2, Holidays!A1:A2)
    ws.write_formula(1, 2, Formula::new("NETWORKDAYS(A2, B2, Holidays!A1:A2)")).unwrap();

    wb.save(&input).unwrap();
    let summary = evaluate(&input, &output, None).unwrap();
    // Formula column C is evaluated for the single data row.
    assert_eq!(summary.formulas_evaluated, 1);

    let result = read_cell(&output, "Main", 1, 2);
    assert_eq!(result, Value::Number(8.0));
}

#[test]
fn sum_whole_column_regression() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet().set_name("Main").unwrap();

    ws.write_string(0, 0, "Value").unwrap();
    ws.write_string(0, 1, "Total").unwrap();

    // A2:A5 = 10, 20, 30, 40
    ws.write_number(1, 0, 10.0).unwrap();
    ws.write_number(2, 0, 20.0).unwrap();
    ws.write_number(3, 0, 30.0).unwrap();
    ws.write_number(4, 0, 40.0).unwrap();

    // B2 = SUM(A:A) — whole-column aggregate prelude path
    ws.write_formula(1, 1, Formula::new("SUM(A:A)")).unwrap();

    wb.save(&input).unwrap();
    let summary = evaluate(&input, &output, None).unwrap();
    // Formula column B is evaluated for every data row (rows 1-4).
    assert_eq!(summary.formulas_evaluated, 4);

    let result = read_cell(&output, "Main", 1, 1);
    assert_eq!(result, Value::Number(100.0));
}

#[test]
fn irr_with_cell_refs_still_works() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet().set_name("Main").unwrap();

    // Header
    ws.write_string(0, 0, "CF1").unwrap();
    ws.write_string(0, 1, "CF2").unwrap();
    ws.write_string(0, 2, "CF3").unwrap();
    ws.write_string(0, 3, "CF4").unwrap();
    ws.write_string(0, 4, "CF5").unwrap();
    ws.write_string(0, 5, "IRR").unwrap();

    // Cashflows across A2:E2
    ws.write_number(1, 0, -1000.0).unwrap();
    ws.write_number(1, 1, 300.0).unwrap();
    ws.write_number(1, 2, 400.0).unwrap();
    ws.write_number(1, 3, 500.0).unwrap();
    ws.write_number(1, 4, 200.0).unwrap();

    // F2 = IRR(A2, B2, C2, D2, E2) — individual cell refs, not range
    ws.write_formula(1, 5, Formula::new("IRR(A2, B2, C2, D2, E2)")).unwrap();

    wb.save(&input).unwrap();
    let summary = evaluate(&input, &output, None).unwrap();
    // Formula column F is evaluated for the single data row.
    assert_eq!(summary.formulas_evaluated, 1);

    let result = read_cell(&output, "Main", 1, 5);
    assert_approx(&result, 0.1532, 0.01);
}

// ---------------------------------------------------------------------------
// SUMPRODUCT
// ---------------------------------------------------------------------------

#[test]
fn sumproduct_two_ranges_produces_correct_result() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet().set_name("Main").unwrap();

    ws.write_string(0, 0, "A").unwrap();
    ws.write_string(0, 1, "B").unwrap();
    ws.write_string(0, 2, "Result").unwrap();

    ws.write_number(1, 0, 1.0).unwrap();
    ws.write_number(2, 0, 2.0).unwrap();
    ws.write_number(3, 0, 3.0).unwrap();
    ws.write_number(1, 1, 4.0).unwrap();
    ws.write_number(2, 1, 5.0).unwrap();
    ws.write_number(3, 1, 6.0).unwrap();

    // C2 = SUMPRODUCT(A2:A4, B2:B4) -> 1*4 + 2*5 + 3*6 = 32
    ws.write_formula(1, 2, Formula::new("SUMPRODUCT(A2:A4, B2:B4)")).unwrap();

    wb.save(&input).unwrap();
    let summary = evaluate(&input, &output, None).unwrap();
    assert_eq!(summary.formulas_evaluated, 3);

    let result = read_cell(&output, "Main", 1, 2);
    assert_eq!(result, Value::Number(32.0));
}

#[test]
fn sumproduct_single_range_sums_values() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet().set_name("Main").unwrap();

    ws.write_string(0, 0, "Value").unwrap();
    ws.write_string(0, 1, "Result").unwrap();

    ws.write_number(1, 0, 10.0).unwrap();
    ws.write_number(2, 0, 20.0).unwrap();
    ws.write_number(3, 0, 30.0).unwrap();

    // B2 = SUMPRODUCT(A2:A4) -> 10 + 20 + 30 = 60
    ws.write_formula(1, 1, Formula::new("SUMPRODUCT(A2:A4)")).unwrap();

    wb.save(&input).unwrap();
    evaluate(&input, &output, None).unwrap();

    let result = read_cell(&output, "Main", 1, 1);
    assert_eq!(result, Value::Number(60.0));
}

#[test]
fn sumproduct_three_ranges_triple_product() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet().set_name("Main").unwrap();

    ws.write_string(0, 0, "A").unwrap();
    ws.write_string(0, 1, "B").unwrap();
    ws.write_string(0, 2, "C").unwrap();
    ws.write_string(0, 3, "Result").unwrap();

    ws.write_number(1, 0, 1.0).unwrap();
    ws.write_number(2, 0, 2.0).unwrap();
    ws.write_number(1, 1, 3.0).unwrap();
    ws.write_number(2, 1, 4.0).unwrap();
    ws.write_number(1, 2, 5.0).unwrap();
    ws.write_number(2, 2, 6.0).unwrap();

    // D2 = SUMPRODUCT(A2:A3, B2:B3, C2:C3) -> 1*3*5 + 2*4*6 = 15 + 48 = 63
    ws.write_formula(1, 3, Formula::new("SUMPRODUCT(A2:A3, B2:B3, C2:C3)")).unwrap();

    wb.save(&input).unwrap();
    evaluate(&input, &output, None).unwrap();

    let result = read_cell(&output, "Main", 1, 3);
    assert_eq!(result, Value::Number(63.0));
}

#[test]
fn sumproduct_with_boolean_conditional_idiom() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet().set_name("Main").unwrap();

    ws.write_string(0, 0, "Value").unwrap();
    ws.write_string(0, 1, "Flag").unwrap();
    ws.write_string(0, 2, "Result").unwrap();

    ws.write_number(1, 0, 10.0).unwrap();
    ws.write_number(2, 0, 20.0).unwrap();
    ws.write_number(3, 0, 30.0).unwrap();
    ws.write_boolean(1, 1, true).unwrap();
    ws.write_boolean(2, 1, false).unwrap();
    ws.write_boolean(3, 1, true).unwrap();

    // C2 = SUMPRODUCT(A2:A4, B2:B4) -> 10*1 + 20*0 + 30*1 = 40
    ws.write_formula(1, 2, Formula::new("SUMPRODUCT(A2:A4, B2:B4)")).unwrap();

    wb.save(&input).unwrap();
    evaluate(&input, &output, None).unwrap();

    let result = read_cell(&output, "Main", 1, 2);
    assert_eq!(result, Value::Number(40.0));
}

#[test]
fn sumproduct_cross_sheet_bounded_range() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.xlsx");
    let output = dir.path().join("output.xlsx");

    let mut wb = Workbook::new();

    // Lookup sheet with weights
    let ws_weights = wb.add_worksheet().set_name("Weights").unwrap();
    ws_weights.write_number(0, 0, 2.0).unwrap();
    ws_weights.write_number(1, 0, 3.0).unwrap();
    ws_weights.write_number(2, 0, 4.0).unwrap();

    // Main sheet
    let ws = wb.add_worksheet().set_name("Main").unwrap();
    ws.write_string(0, 0, "Value").unwrap();
    ws.write_string(0, 1, "Result").unwrap();

    ws.write_number(1, 0, 10.0).unwrap();
    ws.write_number(2, 0, 20.0).unwrap();
    ws.write_number(3, 0, 30.0).unwrap();

    // B2 = SUMPRODUCT(A2:A4, Weights!A1:A3) -> 10*2 + 20*3 + 30*4 = 200
    ws.write_formula(1, 1, Formula::new("SUMPRODUCT(A2:A4, Weights!A1:A3)")).unwrap();

    wb.save(&input).unwrap();
    evaluate(&input, &output, None).unwrap();

    let result = read_cell(&output, "Main", 1, 1);
    assert_eq!(result, Value::Number(200.0));
}
