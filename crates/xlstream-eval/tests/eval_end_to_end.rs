#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::needless_range_loop
)]

mod helpers;

use xlstream_core::{Value, XlStreamError};
use xlstream_eval::evaluate;
use xlstream_io::Reader;

fn read_all_rows(reader: &mut Reader, sheet: &str) -> Vec<(u32, Vec<Value>)> {
    let mut stream = reader.cells(sheet).unwrap();
    let mut rows = Vec::new();
    while let Some(row) = stream.next_row().unwrap() {
        rows.push(row);
    }
    rows
}

// ---------------------------------------------------------------------------
// Cell reference resolution
// ---------------------------------------------------------------------------

#[test]
fn cell_ref_formula_resolves_to_col_a_value() {
    let input = helpers::generate_cell_ref_fixture(3);
    let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();

    let summary = evaluate(input.path(), output.path(), None).unwrap();
    // 1 header + 3 data rows
    assert_eq!(summary.rows_processed, 4, "rows_processed mismatch");
    // 3 rows × 1 formula column
    assert_eq!(summary.formulas_evaluated, 3, "formulas_evaluated mismatch");

    let mut reader = Reader::open(output.path()).unwrap();
    let rows = read_all_rows(&mut reader, "Sheet1");
    assert_eq!(rows.len(), 4);

    // Header row unchanged.
    assert_eq!(rows[0].0, 0);
    assert_eq!(rows[0].1[0], Value::Text("A".into()));
    assert_eq!(rows[0].1[2], Value::Text("C".into()));

    // Data rows: col C (index 2) must equal col A (index 0).
    for i in 1..=3usize {
        assert_eq!(rows[i].1[2], rows[i].1[0], "row {i}: col C should equal col A");
        assert_eq!(rows[i].1[0], Value::Number(i as f64 * 10.0), "row {i}: col A value mismatch");
    }
}

// ---------------------------------------------------------------------------
// Chained formula columns
// ---------------------------------------------------------------------------

#[test]
fn chained_formula_cols_resolve_in_order() {
    let input = helpers::generate_chained_formula_fixture(3);
    let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();

    let summary = evaluate(input.path(), output.path(), None).unwrap();
    assert!(summary.rows_processed > 0);
    // 3 rows × 2 formula columns
    assert_eq!(summary.formulas_evaluated, 6, "formulas_evaluated mismatch");

    let mut reader = Reader::open(output.path()).unwrap();
    let rows = read_all_rows(&mut reader, "Sheet1");

    for i in 1..=3usize {
        let expected = Value::Number(i as f64 * 10.0);
        // col C = =A{row} → should equal col A
        assert_eq!(rows[i].1[2], expected, "row {i}: col C mismatch");
        // col D = =C{row} → should equal col C = col A
        assert_eq!(rows[i].1[3], expected, "row {i}: col D mismatch");
    }
}

// ---------------------------------------------------------------------------
// No-formula passthrough
// ---------------------------------------------------------------------------

#[test]
fn no_formula_sheet_passes_through_unchanged() {
    let input = helpers::generate_no_formula_fixture(3);
    let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();

    evaluate(input.path(), output.path(), None).unwrap();

    let mut reader = Reader::open(output.path()).unwrap();
    let rows = read_all_rows(&mut reader, "Sheet1");
    assert_eq!(rows.len(), 3);
    for (i, (_, row)) in rows.iter().enumerate() {
        assert_eq!(row[0], Value::Number((i + 1) as f64));
    }
}

// ---------------------------------------------------------------------------
// Error cases
// ---------------------------------------------------------------------------

#[test]
fn evaluate_nonexistent_input_returns_xlsx_error() {
    let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let err = evaluate(
        std::path::Path::new("nonexistent_file_that_does_not_exist.xlsx"),
        output.path(),
        None,
    )
    .unwrap_err();
    assert!(matches!(err, XlStreamError::Xlsx(_)), "expected Xlsx error, got {err:?}");
}

#[test]
fn unsupported_formula_returns_unsupported_error() {
    let input = helpers::generate_unsupported_formula_fixture();
    let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let err = evaluate(input.path(), output.path(), None).unwrap_err();
    assert!(
        matches!(err, XlStreamError::Unsupported { .. }),
        "expected Unsupported error, got {err:?}"
    );
}

// ---------------------------------------------------------------------------
// EvaluateSummary
// ---------------------------------------------------------------------------

#[test]
fn summary_duration_is_nonzero_after_real_eval() {
    let input = helpers::generate_cell_ref_fixture(5);
    let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let summary = evaluate(input.path(), output.path(), None).unwrap();
    // Duration should be at least 1 ns for any real I/O.
    assert!(summary.duration.as_nanos() > 0, "duration should be nonzero after evaluation");
}

// ---------------------------------------------------------------------------
// Conditional + logical (Phase 6)
// ---------------------------------------------------------------------------

#[test]
fn conditional_if_short_circuit_and_ifs_tiered_match() {
    let input = helpers::generate_conditional_fixture();
    let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();

    let summary = evaluate(input.path(), output.path(), None).unwrap();
    // 1 header + 5 data rows
    assert_eq!(summary.rows_processed, 6, "rows_processed mismatch");
    // 5 rows * 2 formula columns
    assert_eq!(summary.formulas_evaluated, 10, "formulas_evaluated mismatch");

    let mut reader = Reader::open(output.path()).unwrap();
    let rows = read_all_rows(&mut reader, "Sheet1");
    assert_eq!(rows.len(), 6);

    // Row 1 (A=0): B=0 (IF short-circuit), C="Bronze"
    assert_eq!(rows[1].1[1], Value::Number(0.0), "row 1: IF(0=0, 0, 1/0) should be 0");
    assert_eq!(rows[1].1[2], Value::Text("Bronze".into()), "row 1: tier mismatch");

    // Row 2 (A=150000): C="Platinum"
    assert_eq!(rows[2].1[2], Value::Text("Platinum".into()), "row 2: tier mismatch");

    // Row 3 (A=75000): C="Gold"
    assert_eq!(rows[3].1[2], Value::Text("Gold".into()), "row 3: tier mismatch");

    // Row 4 (A=25000): C="Silver"
    assert_eq!(rows[4].1[2], Value::Text("Silver".into()), "row 4: tier mismatch");

    // Row 5 (A=5000): C="Bronze"
    assert_eq!(rows[5].1[2], Value::Text("Bronze".into()), "row 5: tier mismatch");
}

// ---------------------------------------------------------------------------
// Aggregate prelude — pct of total (Phase 7)
// ---------------------------------------------------------------------------

#[test]
fn aggregate_pct_of_total_via_sum_prelude() {
    let input = helpers::generate_aggregate_fixture();
    let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();

    let summary = evaluate(input.path(), output.path(), None).unwrap();
    assert_eq!(summary.rows_processed, 5, "rows_processed: 1 header + 4 data");
    assert_eq!(summary.formulas_evaluated, 4, "4 rows * 1 formula col");

    let mut reader = Reader::open(output.path()).unwrap();
    let rows = read_all_rows(&mut reader, "Sheet1");
    assert_eq!(rows.len(), 5);

    // Total = 100 + 200 + 300 + 400 = 1000
    let expected_pcts = [10.0, 20.0, 30.0, 40.0];
    for (i, &expected) in expected_pcts.iter().enumerate() {
        let row = &rows[i + 1].1;
        match &row[2] {
            Value::Number(n) => {
                assert!(
                    (n - expected).abs() < 1e-10,
                    "row {}: expected {expected}, got {n}",
                    i + 1
                );
            }
            other => panic!("row {}: expected Number, got {other:?}", i + 1),
        }
    }
}
