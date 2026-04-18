#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod helpers;

use xlstream_core::Value;
use xlstream_io::Reader;

#[test]
fn open_reads_sheet_names() {
    let tmp = helpers::generate_multi_sheet_fixture();
    let reader = Reader::open(tmp.path()).unwrap();
    let names = reader.sheet_names();
    assert_eq!(names, vec!["Alpha".to_string(), "Beta".to_string()]);
}

#[test]
fn cells_yields_dense_rows() {
    let tmp = helpers::generate_simple_fixture();
    let mut reader = Reader::open(tmp.path()).unwrap();
    let mut stream = reader.cells("Sheet1").unwrap();

    let row0 = stream.next_row().unwrap().expect("expected row 0");
    assert_eq!(row0.len(), 3);
    assert_eq!(row0[0], Value::Number(1.0));
    assert_eq!(row0[1], Value::Text("hello".into()));
    assert_eq!(row0[2], Value::Bool(true));

    let row1 = stream.next_row().unwrap().expect("expected row 1");
    assert_eq!(row1.len(), 3);
    assert_eq!(row1[0], Value::Number(2.0));
    assert_eq!(row1[1], Value::Text("world".into()));
    assert_eq!(row1[2], Value::Bool(false));

    assert!(stream.next_row().unwrap().is_none(), "expected EOF");
}

#[test]
fn cells_pads_missing_cells_with_empty() {
    let tmp = helpers::generate_sparse_fixture();
    let mut reader = Reader::open(tmp.path()).unwrap();
    let mut stream = reader.cells("Sheet1").unwrap();

    let row = stream.next_row().unwrap().expect("expected row 0");
    assert_eq!(row.len(), 3);
    assert_eq!(row[0], Value::Number(1.0));
    assert_eq!(row[1], Value::Empty);
    assert_eq!(row[2], Value::Number(3.0));
}

#[test]
fn cells_returns_none_after_last_row() {
    let tmp = helpers::generate_simple_fixture();
    let mut reader = Reader::open(tmp.path()).unwrap();
    let mut stream = reader.cells("Sheet1").unwrap();

    // Drain all rows.
    while stream.next_row().unwrap().is_some() {}

    // Subsequent calls should return None.
    assert!(stream.next_row().unwrap().is_none());
    assert!(stream.next_row().unwrap().is_none());
}

#[test]
fn cells_nonexistent_sheet_returns_error() {
    let tmp = helpers::generate_simple_fixture();
    let mut reader = Reader::open(tmp.path()).unwrap();
    let err = reader.cells("NoSuchSheet").unwrap_err();
    assert!(
        matches!(err, xlstream_core::XlStreamError::Xlsx(_)),
        "expected Xlsx error, got {err:?}",
    );
}

#[test]
fn formulas_returns_formula_text() {
    let tmp = helpers::generate_formula_fixture();
    let mut reader = Reader::open(tmp.path()).unwrap();
    let formulas = reader.formulas("Sheet1").unwrap();

    // Only C1 has a formula.
    assert_eq!(formulas.len(), 1);
    let (row, col, text) = &formulas[0];
    assert_eq!(*row, 0);
    assert_eq!(*col, 2);
    assert_eq!(text, "A1+B1");
}

#[test]
fn cells_reads_formula_cell_cached_value() {
    let tmp = helpers::generate_formula_fixture();
    let mut reader = Reader::open(tmp.path()).unwrap();
    let mut stream = reader.cells("Sheet1").unwrap();

    let row = stream.next_row().unwrap().expect("expected row 0");
    assert_eq!(row.len(), 3);
    assert_eq!(row[0], Value::Number(1.0));
    assert_eq!(row[1], Value::Number(2.0));
    // Formula cell reads the cached value (3.0).
    assert_eq!(row[2], Value::Number(3.0));
}

#[test]
fn cells_multi_sheet_reads_correct_sheet() {
    let tmp = helpers::generate_multi_sheet_fixture();
    let mut reader = Reader::open(tmp.path()).unwrap();

    let mut stream = reader.cells("Alpha").unwrap();
    let row = stream.next_row().unwrap().expect("expected Alpha row 0");
    assert_eq!(row[0], Value::Number(10.0));
    // Must drop stream before opening another (borrow).
    drop(stream);

    let mut stream = reader.cells("Beta").unwrap();
    let row = stream.next_row().unwrap().expect("expected Beta row 0");
    assert_eq!(row[0], Value::Number(20.0));
}
