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

/// Issue 139 v2: aggregates over formula columns containing VLOOKUP
/// produce correct results when lookup sheets are loaded before prelude.
#[test]
fn issue_139_v2_prelude_formula_eval() {
    use calamine::{open_workbook, Data, Reader as CalReader, Xlsx};

    let input = NamedTempFile::with_suffix(".xlsx").unwrap();
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();

    let mut wb = Workbook::new();

    // Sheet "Data" — lookup reference
    let data = wb.add_worksheet();
    data.set_name("Data").unwrap();
    data.write_string(0, 0, "Key").unwrap();
    data.write_string(0, 1, "Rate").unwrap();
    data.write_string(1, 0, "X").unwrap();
    data.write_number(1, 1, 1.5).unwrap();
    data.write_string(2, 0, "Y").unwrap();
    data.write_number(2, 1, 2.0).unwrap();
    data.write_string(3, 0, "Z").unwrap();
    data.write_number(3, 1, 0.5).unwrap();

    // Sheet "Main" — formula columns + aggregates
    let main = wb.add_worksheet();
    main.set_name("Main").unwrap();
    main.write_string(0, 0, "Category").unwrap();
    main.write_string(0, 1, "Amount").unwrap();
    main.write_string(0, 2, "Double").unwrap(); // C = B*2
    main.write_string(0, 3, "Rate").unwrap(); // D = VLOOKUP
    main.write_string(0, 4, "Product").unwrap(); // E = D*B

    // 6 data rows cycling X/Y/Z
    let cats = ["X", "Y", "Z", "X", "Y", "Z"];
    let amounts = [10.0f64, 20.0, 30.0, 40.0, 50.0, 60.0];
    let rates = [1.5f64, 2.0, 0.5, 1.5, 2.0, 0.5];

    for i in 0..6u32 {
        let row = i + 1;
        let row_1based = row + 1; // Excel 1-based for formula refs
        let idx = i as usize;
        let double = amounts[idx] * 2.0;
        let product = amounts[idx] * rates[idx];

        main.write_string(row, 0, cats[idx]).unwrap();
        main.write_number(row, 1, amounts[idx]).unwrap();
        main.write_formula(
            row,
            2,
            Formula::new(format!("B{row_1based}*2")).set_result(&double.to_string()),
        )
        .unwrap();
        main.write_formula(
            row,
            3,
            Formula::new(format!("VLOOKUP(A{row_1based},Data!A:B,2,FALSE)"))
                .set_result(&rates[idx].to_string()),
        )
        .unwrap();
        main.write_formula(
            row,
            4,
            Formula::new(format!("D{row_1based}*B{row_1based}")).set_result(&product.to_string()),
        )
        .unwrap();
    }

    // Compute expected aggregate values
    let sum_c: f64 = amounts.iter().map(|a| a * 2.0).sum();
    let sum_d: f64 = rates.iter().sum();
    let sum_e: f64 = amounts.iter().zip(rates.iter()).map(|(a, r)| a * r).sum();

    // Aggregate formulas in columns G-H
    main.write_string(0, 6, "Test").unwrap();
    main.write_string(0, 7, "Result").unwrap();

    main.write_string(1, 6, "SUM(C:C)").unwrap();
    main.write_formula(1, 7, Formula::new("SUM(C:C)").set_result(&sum_c.to_string())).unwrap();

    main.write_string(2, 6, "SUM(D:D)").unwrap();
    main.write_formula(2, 7, Formula::new("SUM(D:D)").set_result(&sum_d.to_string())).unwrap();

    main.write_string(3, 6, "SUM(E:E)").unwrap();
    main.write_formula(3, 7, Formula::new("SUM(E:E)").set_result(&sum_e.to_string())).unwrap();

    main.write_string(4, 6, "COUNT(D:D)").unwrap();
    main.write_formula(4, 7, Formula::new("COUNT(D:D)").set_result("6")).unwrap();

    wb.save(input.path()).unwrap();

    let opts = EvaluateOptions { workers: Some(1), ..Default::default() };
    evaluate(input.path(), output.path(), &opts).unwrap();

    let mut out_wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let range = out_wb.worksheet_range("Main").unwrap();
    let rows: Vec<_> = range.rows().collect();

    // SUM(C:C) — row-local formula aggregate (baseline)
    let sum_c_actual = &rows[1][7];
    assert!(
        matches!(sum_c_actual, Data::Float(v) if (*v - sum_c).abs() < 1e-6),
        "SUM(C:C): expected {sum_c}, got {sum_c_actual:?}"
    );

    // SUM(D:D) — THIS IS THE KEY TEST: aggregate over VLOOKUP formula column
    let sum_d_actual = &rows[2][7];
    assert!(
        matches!(sum_d_actual, Data::Float(v) if (*v - sum_d).abs() < 1e-6),
        "SUM(D:D): expected {sum_d}, got {sum_d_actual:?}"
    );

    // SUM(E:E) — aggregate over chained formula column (D*B)
    let sum_e_actual = &rows[3][7];
    assert!(
        matches!(sum_e_actual, Data::Float(v) if (*v - sum_e).abs() < 1e-6),
        "SUM(E:E): expected {sum_e}, got {sum_e_actual:?}"
    );

    // COUNT(D:D) — count of VLOOKUP formula column
    let count_d_actual = &rows[4][7];
    assert!(
        matches!(count_d_actual, Data::Float(v) if (*v - 6.0).abs() < 1e-6),
        "COUNT(D:D): expected 6, got {count_d_actual:?}"
    );
}
