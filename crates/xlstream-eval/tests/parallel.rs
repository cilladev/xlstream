#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp,
    clippy::cast_precision_loss
)]

mod helpers;

use xlstream_core::Value;
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

// Task 1.5: Basic parallel determinism
#[test]
fn parallel_output_matches_single_threaded() {
    let input = helpers::generate_arithmetic_fixture(15_000);
    let output_single = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let output_parallel = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();

    let summary_single = evaluate(input.path(), output_single.path(), Some(1)).unwrap();
    let summary_parallel = evaluate(input.path(), output_parallel.path(), Some(4)).unwrap();

    assert_eq!(summary_single.rows_processed, summary_parallel.rows_processed);
    assert_eq!(summary_single.formulas_evaluated, summary_parallel.formulas_evaluated);

    let mut reader_s = Reader::open(output_single.path()).unwrap();
    let mut reader_p = Reader::open(output_parallel.path()).unwrap();
    let rows_s = read_all_rows(&mut reader_s, "Sheet1");
    let rows_p = read_all_rows(&mut reader_p, "Sheet1");

    assert_eq!(rows_s.len(), rows_p.len(), "row count mismatch");
    for (i, (rs, rp)) in rows_s.iter().zip(rows_p.iter()).enumerate() {
        assert_eq!(rs.0, rp.0, "row index mismatch at position {i}");
        assert_eq!(rs.1, rp.1, "row values mismatch at row {}", rs.0);
    }
}

#[test]
fn parallel_workers_2_4_8_produce_identical_output() {
    let input = helpers::generate_arithmetic_fixture(15_000);
    let output_base = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    evaluate(input.path(), output_base.path(), Some(1)).unwrap();

    let mut reader_base = Reader::open(output_base.path()).unwrap();
    let rows_base = read_all_rows(&mut reader_base, "Sheet1");

    for workers in [2, 4, 8] {
        let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
        let summary = evaluate(input.path(), output.path(), Some(workers)).unwrap();
        assert_eq!(
            summary.rows_processed,
            rows_base.len() as u64,
            "rows mismatch for workers={workers}"
        );

        let mut reader = Reader::open(output.path()).unwrap();
        let rows = read_all_rows(&mut reader, "Sheet1");
        assert_eq!(rows.len(), rows_base.len(), "row count mismatch for workers={workers}");
        for (i, (rb, rw)) in rows_base.iter().zip(rows.iter()).enumerate() {
            assert_eq!(rb.0, rw.0, "row index mismatch at {i} for workers={workers}");
            assert_eq!(rb.1, rw.1, "values mismatch at row {} for workers={workers}", rb.0);
        }
    }
}

#[test]
fn parallel_with_one_worker_matches_single_thread() {
    let input = helpers::generate_arithmetic_fixture(15_000);
    let output_a = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let output_b = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();

    let sa = evaluate(input.path(), output_a.path(), Some(1)).unwrap();
    let sb = evaluate(input.path(), output_b.path(), Some(1)).unwrap();

    assert_eq!(sa.rows_processed, sb.rows_processed);
    assert_eq!(sa.formulas_evaluated, sb.formulas_evaluated);
}

// Task 1.6: Threshold fallback
#[test]
fn parallel_small_workload_uses_single_threaded() {
    let input = helpers::generate_cell_ref_fixture(100);
    let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();

    let summary = evaluate(input.path(), output.path(), Some(4)).unwrap();
    assert_eq!(summary.rows_processed, 101);
    assert_eq!(summary.formulas_evaluated, 100);
}

// Task 1.7: Error propagation
#[test]
fn parallel_error_returns_cleanly() {
    let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let err = evaluate(
        std::path::Path::new("nonexistent_file_for_parallel_test.xlsx"),
        output.path(),
        Some(4),
    )
    .unwrap_err();
    assert!(
        matches!(err, xlstream_core::XlStreamError::Xlsx(_)),
        "expected Xlsx error, got {err:?}"
    );
}

#[test]
fn parallel_valid_workload_produces_no_error() {
    let input = helpers::generate_arithmetic_fixture(15_000);
    let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let summary = evaluate(input.path(), output.path(), Some(4)).unwrap();
    assert_eq!(summary.rows_processed, 15_001);
    assert_eq!(summary.formulas_evaluated, 15_000);
}

// Task 1.8: Fuzz-style worker counts
#[test]
fn parallel_fuzz_worker_counts_produce_identical_output() {
    let input = helpers::generate_arithmetic_fixture(15_000);
    let output_base = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    evaluate(input.path(), output_base.path(), Some(1)).unwrap();

    let mut reader_base = Reader::open(output_base.path()).unwrap();
    let rows_base = read_all_rows(&mut reader_base, "Sheet1");

    for workers in [1, 2, 4, 8, 16] {
        let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
        evaluate(input.path(), output.path(), Some(workers)).unwrap();

        let mut reader = Reader::open(output.path()).unwrap();
        let rows = read_all_rows(&mut reader, "Sheet1");
        assert_eq!(rows.len(), rows_base.len(), "row count mismatch for workers={workers}");
        for (i, (rb, rw)) in rows_base.iter().zip(rows.iter()).enumerate() {
            assert_eq!(
                rb.1, rw.1,
                "values mismatch at row {} for workers={workers} (pos {i})",
                rb.0
            );
        }
    }
}

// Task 3.1: Aggregate formulas in parallel
#[test]
fn parallel_with_aggregate_formulas_matches_single() {
    let input = helpers::generate_large_aggregate_fixture(15_000);
    let output_single = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let output_parallel = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();

    let s1 = evaluate(input.path(), output_single.path(), Some(1)).unwrap();
    let s4 = evaluate(input.path(), output_parallel.path(), Some(4)).unwrap();

    assert_eq!(s1.rows_processed, s4.rows_processed);
    assert_eq!(s1.formulas_evaluated, s4.formulas_evaluated);

    let mut r1 = Reader::open(output_single.path()).unwrap();
    let mut r4 = Reader::open(output_parallel.path()).unwrap();
    let rows1 = read_all_rows(&mut r1, "Sheet1");
    let rows4 = read_all_rows(&mut r4, "Sheet1");

    assert_eq!(rows1.len(), rows4.len());
    for (i, (a, b)) in rows1.iter().zip(rows4.iter()).enumerate() {
        assert_eq!(a.0, b.0, "row index mismatch at {i}");
        assert_eq!(a.1, b.1, "values mismatch at row {}", a.0);
    }
}

// Task 3.2: Chained formulas preserve topo order
#[test]
fn parallel_with_chained_formulas_preserves_topo_order() {
    let input = helpers::generate_large_chained_fixture(15_000);
    let output_single = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let output_parallel = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();

    let s1 = evaluate(input.path(), output_single.path(), Some(1)).unwrap();
    let s4 = evaluate(input.path(), output_parallel.path(), Some(4)).unwrap();

    assert_eq!(s1.formulas_evaluated, s4.formulas_evaluated);

    let mut r1 = Reader::open(output_single.path()).unwrap();
    let mut r4 = Reader::open(output_parallel.path()).unwrap();
    let rows1 = read_all_rows(&mut r1, "Sheet1");
    let rows4 = read_all_rows(&mut r4, "Sheet1");

    for (i, (a, b)) in rows1.iter().zip(rows4.iter()).enumerate() {
        assert_eq!(a.1, b.1, "values mismatch at row {} (pos {i})", a.0);
    }
}
