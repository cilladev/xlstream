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
