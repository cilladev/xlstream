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

#[test]
fn issue_142_sumif_wildcard_text_only() {
    super::conformance::run_conformance("issues/issue-142-sumif-wildcard-text-only.xlsx");
}

#[test]
fn issue_143_lookup_range_bounds() {
    super::conformance::run_conformance("issues/issue-143-lookup-range-bounds.xlsx");
}

#[test]
fn issue_144_date_year_adjust() {
    super::conformance::run_conformance("issues/issue-144-date-year-adjust.xlsx");
}

#[test]
fn issue_161_wildcard_blank() {
    super::conformance::run_conformance("issues/issue-161-wildcard-blank.xlsx");
}

#[test]
fn issue_175_countif_text_compare() {
    super::conformance::run_conformance("issues/issue-175-countif-text-compare.xlsx");
}

#[test]
fn issue_138_bounded_conditional_aggregate() {
    super::conformance::run_conformance("issues/issue-138-bounded-conditional-aggregate.xlsx");
}

/// Can't use conformance — this is a deliberate divergence from Excel.
/// Excel resizes an offset sum range (`B5:B14`) to the criteria shape and
/// pairs rows with an offset; same-row streaming cannot express that, so
/// xlstream refuses the formula with #VALUE! instead of silently
/// mis-pairing rows. An Excel-saved fixture would cache the offset result.
#[test]
fn issue_138_offset_sumif_value_range_returns_value_error() {
    use calamine::{open_workbook, Data, Reader as CalReader, Xlsx};

    let input = NamedTempFile::with_suffix(".xlsx").unwrap();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    ws.set_name("Sheet1").unwrap();
    ws.write_string(0, 0, "cat").unwrap();
    ws.write_string(0, 1, "val").unwrap();
    for row in 1..=10 {
        ws.write_string(row, 0, "x").unwrap();
        ws.write_number(row, 1, f64::from(row)).unwrap();
    }
    ws.write_formula(1, 2, Formula::new("SUMIF(A2:A11,\"x\",B5:B14)").set_result("0")).unwrap();
    wb.save(input.path()).unwrap();

    let opts = EvaluateOptions { workers: Some(1), ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = wb.worksheet_range("Sheet1").unwrap();
    let c2 = range.get_value((1, 2)).cloned().unwrap_or(Data::Empty);
    assert!(
        matches!(&c2, Data::String(s) if s == "#VALUE!")
            || matches!(&c2, Data::Error(calamine::CellErrorType::Value)),
        "expected #VALUE! for offset sum range, got {c2:?}"
    );
}

/// Can't use conformance — the runner compares cell values, not sheet
/// order, and the bug only triggers on the parallel path (workers > 1,
/// main sheet with formulas, >= 10,000 data rows). Issue #197.
#[test]
fn issue_197_parallel_path_preserves_sheet_order() {
    use calamine::{open_workbook, Reader as CalReader, Xlsx};

    let input = NamedTempFile::with_suffix(".xlsx").unwrap();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let mut wb = Workbook::new();
    let main = wb.add_worksheet();
    main.set_name("Sheet1").unwrap();
    main.write_string(0, 0, "region").unwrap();
    main.write_string(0, 1, "factor").unwrap();
    for row in 1..=12_000u32 {
        main.write_string(row, 0, "north").unwrap();
        main.write_formula(
            row,
            1,
            Formula::new(format!(
                "INDEX(RegionFactors!B2:B4,MATCH(A{},RegionFactors!A2:A4,0))",
                row + 1
            ))
            .set_result("0"),
        )
        .unwrap();
    }
    for name in ["RegionFactors", "CategoryWeights", "RegionCodes"] {
        let ws = wb.add_worksheet();
        ws.set_name(name).unwrap();
        ws.write_string(0, 0, "region").unwrap();
        for (i, region) in ["north", "south", "east"].iter().enumerate() {
            let row = (i + 1) as u32;
            ws.write_string(row, 0, *region).unwrap();
            ws.write_number(row, 1, f64::from(row) * 1.1).unwrap();
        }
    }
    wb.save(input.path()).unwrap();

    let opts = EvaluateOptions { workers: Some(2), ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let names = wb.sheet_names();
    let order: Vec<&str> = names.iter().map(String::as_str).collect();
    assert_eq!(
        order,
        vec!["Sheet1", "RegionFactors", "CategoryWeights", "RegionCodes"],
        "output must preserve input sheet order"
    );
}
