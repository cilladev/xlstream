#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[allow(dead_code)]
mod helpers;

use xlstream_core::{CellError, ExcelDate, Value, XlStreamError};
use xlstream_io::{Reader, Writer};

/// Write mixed values, re-read with Reader, verify round-trip.
#[test]
fn write_creates_readable_xlsx() {
    let tmp = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let path = tmp.path();

    let mut w = Writer::create(path).unwrap();
    let mut sh = w.add_sheet("Sheet1").unwrap();
    sh.write_row(0, &[Value::Number(1.0), Value::Text("hello".into()), Value::Bool(true)]).unwrap();
    sh.write_row(1, &[Value::Number(2.0), Value::Text("world".into()), Value::Bool(false)])
        .unwrap();
    drop(sh);
    w.finish().unwrap();

    let mut reader = Reader::open(path).unwrap();
    let names = reader.sheet_names();
    assert_eq!(names, vec!["Sheet1".to_string()]);

    let mut stream = reader.cells("Sheet1").unwrap();
    let (idx, row0) = stream.next_row().unwrap().expect("expected row 0");
    assert_eq!(idx, 0);
    assert_eq!(row0[0], Value::Number(1.0));
    assert_eq!(row0[1], Value::Text("hello".into()));
    assert_eq!(row0[2], Value::Bool(true));

    let (idx, row1) = stream.next_row().unwrap().expect("expected row 1");
    assert_eq!(idx, 1);
    assert_eq!(row1[0], Value::Number(2.0));
    assert_eq!(row1[1], Value::Text("world".into()));
    assert_eq!(row1[2], Value::Bool(false));

    assert!(stream.next_row().unwrap().is_none());
}

/// Writing row 3 after row 5 must produce an error.
#[test]
fn write_row_enforces_increasing_row_index() {
    let tmp = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut w = Writer::create(tmp.path()).unwrap();
    let mut sh = w.add_sheet("Sheet1").unwrap();

    sh.write_row(5, &[Value::Number(1.0)]).unwrap();
    let err = sh.write_row(3, &[Value::Number(2.0)]).unwrap_err();
    assert!(
        matches!(err, XlStreamError::Internal(ref msg) if msg.contains("not strictly greater")),
        "expected Internal error about row order, got {err:?}",
    );
}

/// Writing the same row index twice must also fail.
#[test]
fn write_row_rejects_duplicate_row_index() {
    let tmp = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut w = Writer::create(tmp.path()).unwrap();
    let mut sh = w.add_sheet("Sheet1").unwrap();

    sh.write_row(0, &[Value::Number(1.0)]).unwrap();
    let err = sh.write_row(0, &[Value::Number(2.0)]).unwrap_err();
    assert!(matches!(err, XlStreamError::Internal(_)), "expected Internal error, got {err:?}",);
}

/// Write a formula with cached result, re-read, verify the cached value.
#[test]
fn write_formula_sets_cached_result() {
    let tmp = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let path = tmp.path();

    let mut w = Writer::create(path).unwrap();
    let mut sh = w.add_sheet("Sheet1").unwrap();
    sh.write_row(0, &[Value::Number(1.0), Value::Number(2.0)]).unwrap();
    sh.write_formula(0, 2, "=A1+B1", &Value::Number(3.0)).unwrap();
    drop(sh);
    w.finish().unwrap();

    let mut reader = Reader::open(path).unwrap();
    let mut stream = reader.cells("Sheet1").unwrap();
    let (_, row) = stream.next_row().unwrap().expect("expected row 0");

    assert_eq!(row[0], Value::Number(1.0));
    assert_eq!(row[1], Value::Number(2.0));
    // Cached formula result should read back as the number 3
    assert_eq!(row[2], Value::Number(3.0));
}

/// Write one row containing every Value variant, re-read and verify.
#[test]
fn write_all_value_types() {
    let tmp = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let path = tmp.path();

    let values = vec![
        Value::Number(9.81),
        Value::Integer(42),
        Value::Text("text".into()),
        Value::Bool(true),
        Value::Date(ExcelDate { serial: 44927.0 }),
        Value::Error(CellError::Div0),
        Value::Empty,
    ];

    let mut w = Writer::create(path).unwrap();
    let mut sh = w.add_sheet("Sheet1").unwrap();
    sh.write_row(0, &values).unwrap();
    drop(sh);
    w.finish().unwrap();

    let mut reader = Reader::open(path).unwrap();
    let mut stream = reader.cells("Sheet1").unwrap();
    let (_, row) = stream.next_row().unwrap().expect("expected row 0");

    // Number
    assert_eq!(row[0], Value::Number(9.81));
    // Integer written as f64
    assert_eq!(row[1], Value::Number(42.0));
    // Text
    assert_eq!(row[2], Value::Text("text".into()));
    // Bool
    assert_eq!(row[3], Value::Bool(true));
    // Date — comes back as Number (the serial) since calamine sees the
    // formatted number. The exact type depends on calamine's detection,
    // but the serial value must be preserved.
    match &row[4] {
        Value::Number(n) => assert!((*n - 44927.0).abs() < 0.001, "expected ~44927, got {n}"),
        Value::Date(d) => {
            assert!((d.serial - 44927.0).abs() < 0.001, "expected ~44927, got {}", d.serial);
        }
        other => panic!("expected Number or Date for date cell, got {other:?}"),
    }
    // Error written as string
    assert_eq!(row[5], Value::Text("#DIV/0!".into()));
    // Empty — the row may be shorter (trailing empties trimmed) or padded
    assert!(
        row.len() <= 6 || row[6] == Value::Empty,
        "expected Empty or absent trailing cell, got {:?}",
        row.get(6),
    );
}

/// Write a row with Empty values, verify they read back as Empty.
#[test]
fn write_empty_cells_are_skipped() {
    let tmp = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let path = tmp.path();

    let mut w = Writer::create(path).unwrap();
    let mut sh = w.add_sheet("Sheet1").unwrap();
    // A=1.0, B=Empty, C=3.0
    sh.write_row(0, &[Value::Number(1.0), Value::Empty, Value::Number(3.0)]).unwrap();
    drop(sh);
    w.finish().unwrap();

    let mut reader = Reader::open(path).unwrap();
    let mut stream = reader.cells("Sheet1").unwrap();
    let (_, row) = stream.next_row().unwrap().expect("expected row 0");

    assert_eq!(row[0], Value::Number(1.0));
    assert_eq!(row[1], Value::Empty);
    assert_eq!(row[2], Value::Number(3.0));
}

/// Add 2 sheets, write to both, verify round-trip.
#[test]
fn multi_sheet_write() {
    let tmp = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let path = tmp.path();

    let mut w = Writer::create(path).unwrap();

    let mut sh1 = w.add_sheet("Alpha").unwrap();
    sh1.write_row(0, &[Value::Number(10.0)]).unwrap();
    drop(sh1);

    let mut sh2 = w.add_sheet("Beta").unwrap();
    sh2.write_row(0, &[Value::Number(20.0)]).unwrap();
    drop(sh2);

    w.finish().unwrap();

    let mut reader = Reader::open(path).unwrap();
    let names = reader.sheet_names();
    assert_eq!(names, vec!["Alpha".to_string(), "Beta".to_string()]);

    let mut stream = reader.cells("Alpha").unwrap();
    let (_, row) = stream.next_row().unwrap().expect("Alpha row 0");
    assert_eq!(row[0], Value::Number(10.0));
    drop(stream);

    let mut stream = reader.cells("Beta").unwrap();
    let (_, row) = stream.next_row().unwrap().expect("Beta row 0");
    assert_eq!(row[0], Value::Number(20.0));
}

/// Write a row with mixed data and formula cells, re-read, verify both
/// cached values and formula text survive.
#[test]
fn write_row_with_formulas_preserves_formulas_and_values() {
    use std::collections::HashMap;

    let tmp = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let path = tmp.path();

    let mut w = Writer::create(path).unwrap();
    let mut sh = w.add_sheet("Sheet1").unwrap();

    let values = vec![Value::Number(10.0), Value::Number(20.0), Value::Number(30.0)];
    let mut formulas = HashMap::new();
    formulas.insert(2u16, "=A1+B1");
    sh.write_row_with_formulas(0, &values, &formulas).unwrap();
    drop(sh);
    w.finish().unwrap();

    let mut reader = Reader::open(path).unwrap();
    {
        let mut stream = reader.cells("Sheet1").unwrap();
        let (_, row) = stream.next_row().unwrap().expect("expected row 0");
        assert_eq!(row[0], Value::Number(10.0));
        assert_eq!(row[1], Value::Number(20.0));
        assert_eq!(row[2], Value::Number(30.0));
    }

    let formulas_read = reader.formulas("Sheet1").unwrap();
    assert_eq!(formulas_read.len(), 1);
    assert_eq!(formulas_read[0].0, 0);
    assert_eq!(formulas_read[0].1, 2);
    assert!(
        formulas_read[0].2.contains("A1") && formulas_read[0].2.contains("B1"),
        "formula should reference A1 and B1, got: {}",
        formulas_read[0].2,
    );
}

#[test]
fn write_row_with_formulas_rejects_duplicate_row() {
    let tmp = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut w = Writer::create(tmp.path()).unwrap();
    let mut sh = w.add_sheet("Sheet1").unwrap();

    let values = vec![Value::Number(1.0)];
    let formulas = std::collections::HashMap::new();
    sh.write_row_with_formulas(0, &values, &formulas).unwrap();
    let err = sh.write_row_with_formulas(0, &values, &formulas).unwrap_err();
    assert!(matches!(err, XlStreamError::Internal(_)), "expected Internal error, got {err:?}",);
}
