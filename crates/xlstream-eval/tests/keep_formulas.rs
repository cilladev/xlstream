//! Integration tests for formula preservation in output.
//!
//! These test that `<f>formula</f><v>cached</v>` is written (or not) based
//! on `values_only`. They do NOT test value correctness (that's conformance).

#![allow(clippy::all, clippy::pedantic)]

use calamine::{open_workbook, Data, Reader as CalReader, Xlsx};
use rust_xlsxwriter::{Formula, Workbook};
use tempfile::NamedTempFile;
use xlstream_eval::{evaluate, EvaluateOptions};
use xlstream_io::Reader as IoReader;

fn make_simple_fixture() -> NamedTempFile {
    let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    ws.set_name("Sheet1").unwrap();
    ws.write_string(0, 0, "Val").unwrap();
    ws.write_string(0, 1, "Double").unwrap();
    ws.write_string(0, 2, "Label").unwrap();
    for r in 1..=5u32 {
        ws.write_number(r, 0, f64::from(r * 10)).unwrap();
        ws.write_formula(r, 1, Formula::new(format!("A{}*2", r + 1)).set_result("0")).unwrap();
        // IF returns text — tests the discard path
        ws.write_formula(
            r,
            2,
            Formula::new(format!("IF(A{}>30,\"high\",\"low\")", r + 1)).set_result("low"),
        )
        .unwrap();
    }
    wb.save(tmp.path()).unwrap();
    tmp
}

fn make_cross_sheet_fixture() -> NamedTempFile {
    let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let ws1 = wb.add_worksheet();
    ws1.set_name("Sheet1").unwrap();
    ws1.write_string(0, 0, "Key").unwrap();
    ws1.write_string(0, 1, "Lookup").unwrap();
    for r in 1..=3u32 {
        ws1.write_string(r, 0, &format!("item{r}")).unwrap();
        let cached = (r * 100).to_string();
        ws1.write_formula(r, 1, Formula::new(format!("Sheet2!B{}", r + 1)).set_result(&cached))
            .unwrap();
    }
    let ws2 = wb.add_worksheet();
    ws2.set_name("Sheet2").unwrap();
    ws2.write_string(0, 0, "Key").unwrap();
    ws2.write_string(0, 1, "Price").unwrap();
    for r in 1..=3u32 {
        ws2.write_string(r, 0, &format!("item{r}")).unwrap();
        ws2.write_number(r, 1, f64::from(r * 100)).unwrap();
    }
    wb.save(tmp.path()).unwrap();
    tmp
}

fn make_mixed_column_fixture() -> NamedTempFile {
    let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    ws.set_name("Sheet1").unwrap();
    ws.write_string(0, 0, "X").unwrap();
    ws.write_string(0, 1, "Y").unwrap();
    ws.write_string(0, 2, "Result").unwrap();
    // Rows 2-4: multiplication
    for r in 1..=3u32 {
        ws.write_number(r, 0, f64::from(r)).unwrap();
        ws.write_number(r, 1, f64::from(r * 10)).unwrap();
        ws.write_formula(r, 2, Formula::new(format!("A{}*B{}", r + 1, r + 1)).set_result("0"))
            .unwrap();
    }
    // Rows 5-7: addition (different formula structure, same column)
    for r in 4..=6u32 {
        ws.write_number(r, 0, f64::from(r)).unwrap();
        ws.write_number(r, 1, f64::from(r * 10)).unwrap();
        ws.write_formula(r, 2, Formula::new(format!("A{}+B{}", r + 1, r + 1)).set_result("0"))
            .unwrap();
    }
    wb.save(tmp.path()).unwrap();
    tmp
}

fn make_parallel_fixture(row_count: u32) -> NamedTempFile {
    let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    ws.set_name("Sheet1").unwrap();
    ws.write_string(0, 0, "Val").unwrap();
    ws.write_string(0, 1, "Double").unwrap();
    for r in 1..=row_count {
        ws.write_number(r, 0, f64::from(r)).unwrap();
        ws.write_formula(r, 1, Formula::new(format!("A{}*2", r + 1)).set_result("0")).unwrap();
    }
    wb.save(tmp.path()).unwrap();
    tmp
}

// -- Default mode: formulas preserved -----------------------------------------

#[test]
fn default_preserves_numeric_formula_cells() {
    let input = make_simple_fixture();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    evaluate(input.path(), output.path(), &EvaluateOptions::default()).unwrap();

    let mut reader = IoReader::open(output.path()).unwrap();
    let formulas = reader.formulas("Sheet1").unwrap();

    // B column (col 1) has numeric results — formulas preserved
    for r in 1..=5u32 {
        assert!(
            formulas.iter().any(|(row, col, _)| *row == r && *col == 1),
            "expected formula at B{}",
            r + 1,
        );
    }
}

#[test]
fn default_discards_text_result_formula_cells() {
    let input = make_simple_fixture();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    evaluate(input.path(), output.path(), &EvaluateOptions::default()).unwrap();

    let mut reader = IoReader::open(output.path()).unwrap();
    let formulas = reader.formulas("Sheet1").unwrap();

    // C column (col 2) has text results — formulas NOT preserved
    for r in 1..=5u32 {
        assert!(
            !formulas.iter().any(|(row, col, _)| *row == r && *col == 2),
            "C{} should not have formula (text result)",
            r + 1,
        );
    }

    // But the VALUES should still be correct
    let mut wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();
    assert_eq!(rows[1][2], Data::String("low".into()));
    assert_eq!(rows[4][2], Data::String("high".into()));
}

#[test]
fn data_columns_never_get_formulas() {
    let input = make_simple_fixture();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    evaluate(input.path(), output.path(), &EvaluateOptions::default()).unwrap();

    let mut reader = IoReader::open(output.path()).unwrap();
    let formulas = reader.formulas("Sheet1").unwrap();

    // A column (col 0) is data — never has formulas
    assert!(!formulas.iter().any(|(_, col, _)| *col == 0), "data column A should have no formulas",);
}

// -- values_only mode ---------------------------------------------------------

#[test]
fn values_only_strips_all_formulas() {
    let input = make_simple_fixture();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let opts = EvaluateOptions { values_only: true, ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut reader = IoReader::open(output.path()).unwrap();
    let formulas = reader.formulas("Sheet1").unwrap();
    assert!(formulas.is_empty(), "values_only should produce no formulas, got {}", formulas.len());
}

#[test]
fn values_only_multi_sheet_strips_all_formulas() {
    let input = make_cross_sheet_fixture();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let opts = EvaluateOptions { values_only: true, ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut reader = IoReader::open(output.path()).unwrap();
    let formulas = reader.formulas("Sheet1").unwrap();
    assert!(formulas.is_empty(), "Sheet1 should have no formulas with values_only");
}

#[test]
fn values_only_preserves_correct_values() {
    let input = make_simple_fixture();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let opts = EvaluateOptions { values_only: true, ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();
    // B2 = 10*2 = 20
    assert!(
        matches!(rows[1][1], Data::Float(v) if (v - 20.0).abs() < 1e-6),
        "B2 expected 20, got {:?}",
        rows[1][1],
    );
}

// -- Cross-sheet formulas -----------------------------------------------------

#[test]
fn cross_sheet_formulas_preserved() {
    let input = make_cross_sheet_fixture();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    evaluate(input.path(), output.path(), &EvaluateOptions::default()).unwrap();

    let mut reader = IoReader::open(output.path()).unwrap();
    let formulas = reader.formulas("Sheet1").unwrap();

    for r in 1..=3u32 {
        let f = formulas.iter().find(|(row, col, _)| *row == r && *col == 1);
        assert!(f.is_some(), "expected formula at B{}", r + 1);
        assert!(
            f.unwrap().2.to_uppercase().contains("SHEET2"),
            "B{} should reference Sheet2, got: {}",
            r + 1,
            f.unwrap().2,
        );
    }

    let mut wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();
    assert!(
        matches!(rows[1][1], Data::Float(v) if (v - 100.0).abs() < 1e-6),
        "B2 expected 100, got {:?}",
        rows[1][1],
    );
}

// -- Mixed-column (row override) formulas -------------------------------------

#[test]
fn mixed_column_preserves_per_row_formula_text() {
    let input = make_mixed_column_fixture();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    evaluate(input.path(), output.path(), &EvaluateOptions::default()).unwrap();

    let mut reader = IoReader::open(output.path()).unwrap();
    let formulas = reader.formulas("Sheet1").unwrap();

    // Rows 2-4 (idx 1-3): C should contain "*"
    for r in 1..=3u32 {
        let f = formulas.iter().find(|(row, col, _)| *row == r && *col == 2);
        assert!(f.is_some(), "expected formula at C{}", r + 1);
        assert!(f.unwrap().2.contains('*'), "C{} expected *, got: {}", r + 1, f.unwrap().2);
    }

    // Rows 5-7 (idx 4-6): C should contain "+"
    for r in 4..=6u32 {
        let f = formulas.iter().find(|(row, col, _)| *row == r && *col == 2);
        assert!(f.is_some(), "expected formula at C{}", r + 1);
        assert!(f.unwrap().2.contains('+'), "C{} expected +, got: {}", r + 1, f.unwrap().2);
    }

    let mut wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();
    // C2 = 1*10 = 10
    assert!(
        matches!(rows[1][2], Data::Float(v) if (v - 10.0).abs() < 1e-6),
        "C2 expected 10, got {:?}",
        rows[1][2],
    );
    // C5 = 4+40 = 44
    assert!(
        matches!(rows[4][2], Data::Float(v) if (v - 44.0).abs() < 1e-6),
        "C5 expected 44, got {:?}",
        rows[4][2],
    );
}

// -- Parallel path ------------------------------------------------------------

#[test]
fn parallel_path_preserves_formulas() {
    let input = make_parallel_fixture(10_500);
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let opts = EvaluateOptions { workers: Some(2), ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut reader = IoReader::open(output.path()).unwrap();
    let formulas = reader.formulas("Sheet1").unwrap();

    assert!(formulas.len() >= 10_000, "expected >= 10k formula cells, got {}", formulas.len(),);

    // Spot-check first and last
    assert!(formulas.iter().any(|(r, c, _)| *r == 1 && *c == 1), "expected formula at B2");
    assert!(formulas.iter().any(|(r, c, _)| *r == 10_500 && *c == 1), "expected formula at B10501",);

    let mut wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();
    // B2 = 1*2 = 2
    assert!(
        matches!(rows[1][1], Data::Float(v) if (v - 2.0).abs() < 1e-6),
        "B2 expected 2, got {:?}",
        rows[1][1],
    );
}
