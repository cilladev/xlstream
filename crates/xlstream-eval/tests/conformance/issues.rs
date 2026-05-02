// Issue-specific conformance fixtures go here.
// Each test points at a fixture in fixtures/issues/.

use std::collections::HashSet;
use std::path::Path;

use calamine::{open_workbook, Data, Reader as CalReader, Xlsx};
use rust_xlsxwriter::{Formula, Workbook};
use tempfile::NamedTempFile;
use xlstream_eval::{evaluate, EvaluateOptions};
use xlstream_io::Reader as IoReader;

#[test]
fn issue_76_self_referential_formulas() {
    super::conformance::run_conformance("issues/issue-76-self-referential-formulas.xlsx");
}

/// Proves iterative calc transforms values: seed=500 -> -500 after one negation.
/// Can't use conformance for this — oscillating formulas phase-shift when
/// re-seeded from their own output.
#[test]
fn issue_76_negation_max_iter_1() {
    let input = NamedTempFile::with_suffix(".xlsx").unwrap();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    ws.set_name("Sheet1").unwrap();
    ws.write_string(0, 0, "Flag").unwrap();
    ws.write_string(1, 0, "Risk_KC").unwrap();
    ws.write_formula(1, 1, Formula::new("IF(A2=\"Risk_KC\",B2*-1,B2)").set_result("500")).unwrap();
    wb.save(input.path()).unwrap();

    let opts = EvaluateOptions { workers: Some(1), max_iterations: 1, ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();
    let b2 = &rows[1][1];
    assert!(matches!(b2, Data::Float(v) if (*v + 500.0).abs() < 1e-6), "expected -500, got {b2:?}");
}

fn fixtures_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures")
}

#[test]
fn keep_formulas_default_preserves_formula_text() {
    let fixture = fixtures_dir().join("issues/keep-formulas.xlsx");
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let opts = EvaluateOptions::default();
    assert!(!opts.values_only);
    evaluate(&fixture, output.path(), &opts).unwrap();

    let mut reader = IoReader::open(output.path()).unwrap();
    let formulas = reader.formulas("Sheet1").unwrap();

    let formula_cells: HashSet<(u32, u32)> = formulas.iter().map(|(r, c, _)| (*r, *c)).collect();

    // Formula cols: B(1) and C(2) should have formulas for rows 1..11 (0-indexed)
    // E(4) = IF(..) returns text — skipped (round-trip limitation)
    for r in 1..=10u32 {
        assert!(formula_cells.contains(&(r, 1)), "expected formula at B{}", r + 1);
        assert!(formula_cells.contains(&(r, 2)), "expected formula at C{}", r + 1);
    }

    // Data columns A(0) and D(3) should NOT have formulas
    for r in 0..=10u32 {
        assert!(!formula_cells.contains(&(r, 0)), "unexpected formula at A{}", r + 1);
        assert!(!formula_cells.contains(&(r, 3)), "unexpected formula at D{}", r + 1);
    }

    // Cached values correct
    let mut wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();
    assert!(
        matches!(rows[1][1], Data::Float(v) if (v - 40.0).abs() < 1e-6),
        "B2 expected 40, got {:?}",
        rows[1][1],
    );
    assert!(
        matches!(rows[1][2], Data::Float(v) if (v - 650.0).abs() < 1e-6),
        "C2 expected 650, got {:?}",
        rows[1][2],
    );
    assert_eq!(rows[1][4], Data::String("low".into()));
    assert_eq!(rows[5][4], Data::String("high".into()));
}

#[test]
fn keep_formulas_values_only_strips_formulas() {
    let fixture = fixtures_dir().join("issues/keep-formulas.xlsx");
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let opts = EvaluateOptions { values_only: true, ..Default::default() };
    evaluate(&fixture, output.path(), &opts).unwrap();

    let mut reader = IoReader::open(output.path()).unwrap();
    let formulas = reader.formulas("Sheet1").unwrap();
    assert!(
        formulas.is_empty(),
        "values_only should produce no formula cells, got {}",
        formulas.len()
    );

    let mut wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();
    assert!(
        matches!(rows[1][1], Data::Float(v) if (v - 40.0).abs() < 1e-6),
        "B2 expected 40, got {:?}",
        rows[1][1],
    );
}

#[test]
fn keep_formulas_cross_sheet_preserves_formula_text() {
    let fixture = fixtures_dir().join("issues/keep-formulas-cross-sheet.xlsx");
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    evaluate(&fixture, output.path(), &EvaluateOptions::default()).unwrap();

    let mut reader = IoReader::open(output.path()).unwrap();
    let formulas = reader.formulas("Sheet1").unwrap();

    // B2..B6 should have formulas referencing Sheet2
    for r in 1..=5u32 {
        let f = formulas.iter().find(|(row, col, _)| *row == r && *col == 1);
        assert!(f.is_some(), "expected formula at B{}", r + 1);
        let text = &f.unwrap().2;
        assert!(
            text.to_uppercase().contains("SHEET2"),
            "B{} formula should reference Sheet2, got: {}",
            r + 1,
            text,
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

#[test]
fn keep_formulas_mixed_column_preserves_per_row_text() {
    let fixture = fixtures_dir().join("issues/keep-formulas-mixed-column.xlsx");
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    evaluate(&fixture, output.path(), &EvaluateOptions::default()).unwrap();

    let mut reader = IoReader::open(output.path()).unwrap();
    let formulas = reader.formulas("Sheet1").unwrap();

    // Rows 2-6 (0-indexed 1-5): C(2) should contain "*"
    for r in 1..=5u32 {
        let f = formulas.iter().find(|(row, col, _)| *row == r && *col == 2);
        assert!(f.is_some(), "expected formula at C{}", r + 1);
        let text = &f.unwrap().2;
        assert!(text.contains('*'), "C{} expected multiplication formula, got: {}", r + 1, text);
    }

    // Rows 7-11 (0-indexed 6-10): C(2) should contain "+"
    for r in 6..=10u32 {
        let f = formulas.iter().find(|(row, col, _)| *row == r && *col == 2);
        assert!(f.is_some(), "expected formula at C{}", r + 1);
        let text = &f.unwrap().2;
        assert!(text.contains('+'), "C{} expected addition formula, got: {}", r + 1, text);
    }

    let mut wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();
    assert!(
        matches!(rows[1][2], Data::Float(v) if (v - 40.0).abs() < 1e-6),
        "C2 expected 40, got {:?}",
        rows[1][2],
    );
    assert!(
        matches!(rows[6][2], Data::Float(v) if (v - 77.0).abs() < 1e-6),
        "C7 expected 77, got {:?}",
        rows[6][2],
    );
}
