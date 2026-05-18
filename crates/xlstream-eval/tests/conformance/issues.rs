// Issue-specific conformance fixtures go here.
// Each test points at a fixture in fixtures/issues/.

use calamine::{open_workbook, Data, Reader as CalReader, Xlsx};
use rust_xlsxwriter::{Formula, Workbook};
use tempfile::NamedTempFile;
use xlstream_eval::{evaluate, EvaluateOptions};

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

#[test]
fn issue_136_cross_sheet_cell_ref() {
    super::conformance::run_conformance("issues/issue-136-cross-sheet-cell-ref.xlsx");
}

#[test]
fn issue_136_cross_sheet_bare_cell_ref() {
    let input = NamedTempFile::with_suffix(".xlsx").unwrap();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let mut wb = Workbook::new();

    // Sheet2: data sheet with known values
    let data = wb.add_worksheet();
    data.set_name("Sheet2").unwrap();
    data.write_number(0, 0, 100.0).unwrap(); // A1 = 100
    data.write_number(0, 1, 200.0).unwrap(); // B1 = 200
    data.write_number(1, 0, 42.0).unwrap(); // A2 = 42
    data.write_number(1, 2, 7.0).unwrap(); // C2 = 7

    // Sheet1: main sheet with formulas referencing Sheet2
    let main = wb.add_worksheet();
    main.set_name("Sheet1").unwrap();

    // A1: data value (not a formula)
    main.write_number(0, 0, 1.0).unwrap();
    // B1: bare cross-sheet ref
    main.write_formula(0, 1, Formula::new("Sheet2!A1").set_result("100")).unwrap();
    // C1: cross-sheet ref inside math
    main.write_formula(0, 2, Formula::new("A1+Sheet2!B1").set_result("201")).unwrap();
    // A2: data
    main.write_number(1, 0, 5.0).unwrap();
    // B2: cross-sheet ref inside function
    main.write_formula(1, 1, Formula::new("EVEN(Sheet2!A2)").set_result("42")).unwrap();
    // C2: cross-sheet ref inside IF
    main.write_formula(1, 2, Formula::new("IF(A2>0,Sheet2!C2,0)").set_result("7")).unwrap();

    wb.save(input.path()).unwrap();

    let opts = EvaluateOptions { workers: Some(1), ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut wb_out: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb_out.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();

    // B1 = Sheet2!A1 = 100
    assert!(
        matches!(&rows[0][1], Data::Float(v) if (*v - 100.0).abs() < 1e-6),
        "B1: expected 100, got {:?}",
        rows[0][1]
    );
    // C1 = A1 + Sheet2!B1 = 1 + 200 = 201
    assert!(
        matches!(&rows[0][2], Data::Float(v) if (*v - 201.0).abs() < 1e-6),
        "C1: expected 201, got {:?}",
        rows[0][2]
    );
    // B2 = EVEN(Sheet2!A2) = EVEN(42) = 42
    assert!(
        matches!(&rows[1][1], Data::Float(v) if (*v - 42.0).abs() < 1e-6),
        "B2: expected 42, got {:?}",
        rows[1][1]
    );
    // C2 = IF(5>0, Sheet2!C2, 0) = 7
    assert!(
        matches!(&rows[1][2], Data::Float(v) if (*v - 7.0).abs() < 1e-6),
        "C2: expected 7, got {:?}",
        rows[1][2]
    );
}

#[test]
fn issue_136_cross_sheet_row_override() {
    let input = NamedTempFile::with_suffix(".xlsx").unwrap();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let mut wb = Workbook::new();

    let data = wb.add_worksheet();
    data.set_name("Rates").unwrap();
    data.write_number(0, 0, 0.05).unwrap(); // A1 = 0.05
    data.write_number(1, 0, 0.10).unwrap(); // A2 = 0.10

    let main = wb.add_worksheet();
    main.set_name("Sheet1").unwrap();

    // Column B: row 1 has template formula, row 2 has a row override referencing Rates sheet
    main.write_number(0, 0, 10.0).unwrap();
    main.write_formula(0, 1, Formula::new("A1*2").set_result("20")).unwrap();
    main.write_number(1, 0, 20.0).unwrap();
    main.write_formula(1, 1, Formula::new("A2*Rates!A2").set_result("2")).unwrap();

    wb.save(input.path()).unwrap();

    let opts = EvaluateOptions { workers: Some(1), ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut wb_out: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb_out.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();

    // B1 = A1*2 = 20
    assert!(
        matches!(&rows[0][1], Data::Float(v) if (*v - 20.0).abs() < 1e-6),
        "B1: expected 20, got {:?}",
        rows[0][1]
    );
    // B2 = A2 * Rates!A2 = 20 * 0.10 = 2
    assert!(
        matches!(&rows[1][1], Data::Float(v) if (*v - 2.0).abs() < 1e-6),
        "B2: expected 2, got {:?}",
        rows[1][1]
    );
}
