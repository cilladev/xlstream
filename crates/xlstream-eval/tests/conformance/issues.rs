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

#[test]
fn issue_137_cross_sheet_simple_aggregate() {
    super::conformance::run_conformance("issues/issue-137-cross-sheet-simple-aggregate.xlsx");
}

#[test]
fn issue_138_bounded_aggregate_row_bounds() {
    super::conformance::run_conformance("issues/issue-138-bounded-aggregate-row-bounds.xlsx");
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

#[test]
fn issue_139_prelude_formula_eval() {
    super::conformance::run_conformance("issues/issue-139-prelude-formula-eval.xlsx");
}

#[test]
fn issue_140_aggregate_literal_args() {
    super::conformance::run_conformance("issues/issue-140-aggregate-literal-args.xlsx");
}

#[test]
fn issue_141_countif_equals_criteria() {
    super::conformance::run_conformance("issues/issue-141-countif-equals-criteria.xlsx");
}
