//! Shared fixture generators for xlstream-eval integration tests.
//!
//! Uses raw `rust_xlsxwriter::Workbook` (non-constant-memory mode) so that
//! formula cells and data cells can be written in arbitrary order within a
//! row without triggering the row-order enforcement in `SheetHandle`.
#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

use rust_xlsxwriter::{Formula, Workbook};
use tempfile::NamedTempFile;

/// Generates a fixture with header row `[A, B, C]` plus `n_rows` data rows.
///
/// - Col A: `i * 10.0` (data)
/// - Col B: `i * 20.0` (data)
/// - Col C: formula `=A{excel_row}` (references col A of same row)
///
/// After evaluation col C should equal col A.
#[allow(dead_code)]
pub fn generate_cell_ref_fixture(n_rows: usize) -> NamedTempFile {
    let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();

    // Header row (calamine row 0 = Excel row 1).
    ws.write_string(0, 0, "A").unwrap();
    ws.write_string(0, 1, "B").unwrap();
    ws.write_string(0, 2, "C").unwrap();

    for i in 0..n_rows {
        let row = (i + 1) as u32; // calamine 0-based row index; row 0 is headers
        let a_val = (i + 1) as f64 * 10.0;
        let b_val = (i + 1) as f64 * 20.0;
        // Excel row number = calamine row + 1 (Excel is 1-based).
        // Formula in calamine row `row` should reference Excel row `row+1`.
        let formula = format!("=A{}", row + 1);

        ws.write_number(row, 0, a_val).unwrap();
        ws.write_number(row, 1, b_val).unwrap();
        ws.write_formula(row, 2, Formula::new(&formula).set_result(a_val.to_string())).unwrap();
    }

    wb.save(tmp.path()).unwrap();
    tmp
}

/// Fixture with two chained formula columns.
///
/// Header `[A, B, C, D]`. Col C = `=A{row}`, col D = `=C{row}`.
/// After evaluation: C = A, D = A (D resolves through C).
#[allow(dead_code)]
pub fn generate_chained_formula_fixture(n_rows: usize) -> NamedTempFile {
    let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();

    ws.write_string(0, 0, "A").unwrap();
    ws.write_string(0, 1, "B").unwrap();
    ws.write_string(0, 2, "C").unwrap();
    ws.write_string(0, 3, "D").unwrap();

    for i in 0..n_rows {
        let row = (i + 1) as u32;
        let a_val = (i + 1) as f64 * 10.0;
        let excel_row = row + 1;

        ws.write_number(row, 0, a_val).unwrap();
        ws.write_number(row, 1, a_val * 2.0).unwrap();
        ws.write_formula(
            row,
            2,
            Formula::new(format!("=A{excel_row}")).set_result(a_val.to_string()),
        )
        .unwrap();
        ws.write_formula(
            row,
            3,
            Formula::new(format!("=C{excel_row}")).set_result(a_val.to_string()),
        )
        .unwrap();
    }

    wb.save(tmp.path()).unwrap();
    tmp
}

/// Fixture with no formula cells — plain data only.
///
/// Single sheet, `n_rows` rows, one column: values `1.0, 2.0, ...`.
#[allow(dead_code)]
pub fn generate_no_formula_fixture(n_rows: usize) -> NamedTempFile {
    let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();

    for i in 0..n_rows {
        ws.write_number(i as u32, 0, (i + 1) as f64).unwrap();
    }

    wb.save(tmp.path()).unwrap();
    tmp
}

/// Fixture with an unsupported formula (`OFFSET`, which is refused at
/// classification time).
#[allow(dead_code)]
pub fn generate_unsupported_formula_fixture() -> NamedTempFile {
    let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();

    ws.write_string(0, 0, "A").unwrap();
    ws.write_string(0, 1, "B").unwrap();
    // Row 1: col B has OFFSET, which is unsupported.
    ws.write_number(1, 0, 10.0).unwrap();
    ws.write_formula(1, 1, Formula::new("=OFFSET(A1,0,0)").set_result("10")).unwrap();

    wb.save(tmp.path()).unwrap();
    tmp
}

/// Fixture with conditional formulas.
///
/// Header `[Value, SafeDiv, Tier]`. 5 data rows.
/// - Col A: 0, 150000, 75000, 25000, 5000
/// - Col B: `=IF(A{row}=0, 0, 1/A{row})` — short-circuit test
/// - Col C: `=IFS(A{row}>100000, "Platinum", ..., TRUE, "Bronze")`
#[allow(dead_code)]
pub fn generate_conditional_fixture() -> NamedTempFile {
    let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();

    ws.write_string(0, 0, "Value").unwrap();
    ws.write_string(0, 1, "SafeDiv").unwrap();
    ws.write_string(0, 2, "Tier").unwrap();

    let values = [0.0, 150_000.0, 75_000.0, 25_000.0, 5_000.0];
    let expected_tiers = ["Bronze", "Platinum", "Gold", "Silver", "Bronze"];

    for (i, (&a_val, &tier)) in values.iter().zip(expected_tiers.iter()).enumerate() {
        let row = (i + 1) as u32;
        let excel_row = row + 1;

        ws.write_number(row, 0, a_val).unwrap();

        let cond_formula = format!("=IF(A{excel_row}=0, 0, 1/A{excel_row})");
        let cond_result = if a_val == 0.0 { "0".to_string() } else { (1.0 / a_val).to_string() };
        ws.write_formula(row, 1, Formula::new(&cond_formula).set_result(&cond_result)).unwrap();

        let tier_formula = format!(
            "=IFS(A{excel_row}>100000, \"Platinum\", A{excel_row}>50000, \"Gold\", A{excel_row}>10000, \"Silver\", TRUE, \"Bronze\")",
        );
        ws.write_formula(row, 2, Formula::new(&tier_formula).set_result(tier)).unwrap();
    }

    wb.save(tmp.path()).unwrap();
    tmp
}

/// Fixture with aggregate formulas (pct of total).
///
/// Header `[Region, Deal Value, Pct of Total]`. 4 data rows.
/// Col C: `=B{row}/SUM(B:B)*100` — requires prelude pass to compute SUM(B:B).
#[allow(dead_code)]
pub fn generate_aggregate_fixture() -> NamedTempFile {
    let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();

    ws.write_string(0, 0, "Region").unwrap();
    ws.write_string(0, 1, "Deal Value").unwrap();
    ws.write_string(0, 2, "Pct of Total").unwrap();

    let data = [("EMEA", 100.0), ("APAC", 200.0), ("EMEA", 300.0), ("AMER", 400.0)];

    for (i, (region, value)) in data.iter().enumerate() {
        let row = (i + 1) as u32;
        let excel_row = row + 1;
        ws.write_string(row, 0, *region).unwrap();
        ws.write_number(row, 1, *value).unwrap();
        let pct = *value / 1000.0 * 100.0;
        let formula = format!("=B{excel_row}/SUM(B:B)*100");
        ws.write_formula(row, 2, Formula::new(&formula).set_result(pct.to_string())).unwrap();
    }

    wb.save(tmp.path()).unwrap();
    tmp
}
