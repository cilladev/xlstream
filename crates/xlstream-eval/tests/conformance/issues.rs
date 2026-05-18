// Issue-specific conformance fixtures go here.
// Each test points at a fixture in fixtures/issues/.

use rust_xlsxwriter::{Formula, Workbook};
use tempfile::NamedTempFile;
use xlstream_eval::{evaluate, EvaluateOptions};

#[test]
fn issue_76_self_referential_formulas() {
    super::conformance::run_conformance("issues/issue-76-self-referential-formulas.xlsx");
}

/// Can't use conformance — oscillating formulas phase-shift when re-seeded.
#[test]
fn issue_76_negation_max_iter_1() {
    use calamine::{open_workbook, Data, Reader as CalReader, Xlsx};

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

/// Cross-sheet simple aggregates must read from the referenced sheet, not main.
#[test]
fn issue_137_cross_sheet_simple_aggregate() {
    use calamine::{open_workbook, Data, Reader as CalReader, Xlsx};

    let input = NamedTempFile::with_suffix(".xlsx").unwrap();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let mut wb = Workbook::new();

    let main = wb.add_worksheet();
    main.set_name("Sheet1").unwrap();
    main.write_string(0, 0, "Value").unwrap();
    main.write_string(0, 1, "Result").unwrap();
    main.write_number(1, 0, 1000.0).unwrap();
    main.write_number(2, 0, 2000.0).unwrap();
    main.write_formula(1, 1, Formula::new("SUM(Sheet2!A2:A4)").set_result("0")).unwrap();
    main.write_formula(2, 1, Formula::new("AVERAGE(Sheet2!A2:A4)").set_result("0")).unwrap();
    main.write_formula(3, 1, Formula::new("COUNT(Sheet2!A2:A4)").set_result("0")).unwrap();
    main.write_formula(4, 1, Formula::new("MAX(Sheet2!A2:A4)").set_result("0")).unwrap();
    main.write_formula(5, 1, Formula::new("MIN(Sheet2!A2:A4)").set_result("0")).unwrap();

    let data = wb.add_worksheet();
    data.set_name("Sheet2").unwrap();
    data.write_string(0, 0, "Data").unwrap();
    data.write_number(1, 0, 10.0).unwrap();
    data.write_number(2, 0, 20.0).unwrap();
    data.write_number(3, 0, 30.0).unwrap();

    wb.save(input.path()).unwrap();

    let opts = EvaluateOptions { workers: Some(1), ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut out: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = out.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();

    let b2 = &rows[1][1];
    assert!(
        matches!(b2, Data::Float(v) if (*v - 60.0).abs() < 1e-6),
        "SUM(Sheet2!A2:A4) expected 60, got {b2:?}"
    );
    let b3 = &rows[2][1];
    assert!(
        matches!(b3, Data::Float(v) if (*v - 20.0).abs() < 1e-6),
        "AVERAGE(Sheet2!A2:A4) expected 20, got {b3:?}"
    );
    let b4 = &rows[3][1];
    assert!(
        matches!(b4, Data::Float(v) if (*v - 3.0).abs() < 1e-6),
        "COUNT(Sheet2!A2:A4) expected 3, got {b4:?}"
    );
    let b5 = &rows[4][1];
    assert!(
        matches!(b5, Data::Float(v) if (*v - 30.0).abs() < 1e-6),
        "MAX(Sheet2!A2:A4) expected 30, got {b5:?}"
    );
    let b6 = &rows[5][1];
    assert!(
        matches!(b6, Data::Float(v) if (*v - 10.0).abs() < 1e-6),
        "MIN(Sheet2!A2:A4) expected 10, got {b6:?}"
    );
}

/// Multiple cross-sheets: SUM(Sheet2!A:A) + MAX(Sheet3!B:B) in same workbook.
#[test]
fn issue_137_multi_sheet_aggregate_isolation() {
    use calamine::{open_workbook, Data, Reader as CalReader, Xlsx};

    let input = NamedTempFile::with_suffix(".xlsx").unwrap();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let mut wb = Workbook::new();

    let main = wb.add_worksheet();
    main.set_name("Sheet1").unwrap();
    main.write_string(0, 0, "X").unwrap();
    main.write_number(1, 0, 999.0).unwrap();
    main.write_formula(1, 1, Formula::new("SUM(Sheet2!A2:A4)").set_result("0")).unwrap();
    main.write_formula(2, 1, Formula::new("MAX(Sheet3!B2:B4)").set_result("0")).unwrap();

    let s2 = wb.add_worksheet();
    s2.set_name("Sheet2").unwrap();
    s2.write_string(0, 0, "V").unwrap();
    s2.write_number(1, 0, 1.0).unwrap();
    s2.write_number(2, 0, 2.0).unwrap();
    s2.write_number(3, 0, 3.0).unwrap();

    let s3 = wb.add_worksheet();
    s3.set_name("Sheet3").unwrap();
    s3.write_string(0, 0, "A").unwrap();
    s3.write_string(0, 1, "B").unwrap();
    s3.write_number(1, 1, 100.0).unwrap();
    s3.write_number(2, 1, 200.0).unwrap();
    s3.write_number(3, 1, 300.0).unwrap();

    wb.save(input.path()).unwrap();

    let opts = EvaluateOptions { workers: Some(1), ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut out: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = out.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();

    let b2 = &rows[1][1];
    assert!(
        matches!(b2, Data::Float(v) if (*v - 6.0).abs() < 1e-6),
        "SUM(Sheet2!A2:A4) expected 6, got {b2:?}"
    );
    let b3 = &rows[2][1];
    assert!(
        matches!(b3, Data::Float(v) if (*v - 300.0).abs() < 1e-6),
        "MAX(Sheet3!B2:B4) expected 300, got {b3:?}"
    );
}

/// Mixed main-sheet + cross-sheet: SUM(A:A) and SUM(Sheet2!A:A) coexist.
#[test]
fn issue_137_mixed_main_and_cross_sheet_aggregate() {
    use calamine::{open_workbook, Data, Reader as CalReader, Xlsx};

    let input = NamedTempFile::with_suffix(".xlsx").unwrap();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let mut wb = Workbook::new();

    let main = wb.add_worksheet();
    main.set_name("Sheet1").unwrap();
    main.write_string(0, 0, "Val").unwrap();
    main.write_string(0, 1, "Res").unwrap();
    main.write_number(1, 0, 100.0).unwrap();
    main.write_number(2, 0, 200.0).unwrap();
    main.write_formula(1, 1, Formula::new("SUM(A2:A3)").set_result("0")).unwrap();
    main.write_formula(2, 1, Formula::new("SUM(Sheet2!A2:A3)").set_result("0")).unwrap();

    let s2 = wb.add_worksheet();
    s2.set_name("Sheet2").unwrap();
    s2.write_string(0, 0, "D").unwrap();
    s2.write_number(1, 0, 5.0).unwrap();
    s2.write_number(2, 0, 7.0).unwrap();

    wb.save(input.path()).unwrap();

    let opts = EvaluateOptions { workers: Some(1), ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut out: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = out.worksheet_range("Sheet1").unwrap();
    let rows: Vec<_> = range.rows().collect();

    let b2 = &rows[1][1];
    assert!(
        matches!(b2, Data::Float(v) if (*v - 300.0).abs() < 1e-6),
        "SUM(A2:A3) expected 300, got {b2:?}"
    );
    let b3 = &rows[2][1];
    assert!(
        matches!(b3, Data::Float(v) if (*v - 12.0).abs() < 1e-6),
        "SUM(Sheet2!A2:A3) expected 12, got {b3:?}"
    );
}

#[test]
fn issue_137_cross_sheet_simple_aggregate_conformance() {
    super::conformance::run_conformance("issues/issue-137-cross-sheet-simple-aggregate.xlsx");
}

/// Can't use conformance — asserts evaluate() returns Err, not cell values.
#[test]
fn issue_136_nonexistent_sheet_is_unsupported() {
    let input = NamedTempFile::with_suffix(".xlsx").unwrap();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let mut wb = Workbook::new();
    let main = wb.add_worksheet();
    main.set_name("Sheet1").unwrap();
    main.write_number(0, 0, 1.0).unwrap();
    main.write_formula(0, 1, Formula::new("NoSuchSheet!A1").set_result("0")).unwrap();
    wb.save(input.path()).unwrap();

    let opts = EvaluateOptions { workers: Some(1), ..Default::default() };
    let result = evaluate(input.path(), output.path(), &opts);
    assert!(result.is_err(), "expected Unsupported error for nonexistent sheet");
}
