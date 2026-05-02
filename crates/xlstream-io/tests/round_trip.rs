#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[allow(dead_code)]
mod helpers;

use xlstream_core::Value;
use xlstream_io::{Reader, Writer};

/// Read all rows from a sheet into a Vec.
fn read_all_rows(reader: &mut Reader, sheet: &str) -> Vec<(u32, Vec<Value>)> {
    let mut stream = reader.cells(sheet).unwrap();
    let mut rows = Vec::new();
    while let Some(row) = stream.next_row().unwrap() {
        rows.push(row);
    }
    rows
}

/// Round-trip helper: read all sheets from `src`, write them to a new temp
/// file via Writer, return the temp file.
fn round_trip_all_sheets(src: &mut Reader) -> tempfile::NamedTempFile {
    let tmp = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let names = src.sheet_names();

    let mut writer = Writer::create(tmp.path()).unwrap();
    for name in &names {
        let rows = read_all_rows(src, name);
        let mut sh = writer.add_sheet(name).unwrap();
        for (idx, values) in &rows {
            sh.write_row(*idx, values).unwrap();
        }
        drop(sh);
    }
    writer.finish().unwrap();
    tmp
}

// -- Value-type round-trip ---------------------------------------------------

#[test]
fn round_trip_preserves_all_value_types() {
    let fixture = helpers::generate_all_types_fixture();
    let mut reader = Reader::open(fixture.path()).unwrap();
    let out = round_trip_all_sheets(&mut reader);

    let mut re_reader = Reader::open(out.path()).unwrap();
    let mut stream = re_reader.cells("Sheet1").unwrap();
    let (idx, row) = stream.next_row().unwrap().expect("expected row 0");
    assert_eq!(idx, 0);

    // Number
    assert_eq!(row[0], Value::Number(42.0));
    // Text
    assert_eq!(row[1], Value::Text("hello".into()));
    // Bool
    assert_eq!(row[2], Value::Bool(true));
    // Date: serial 45000.0 — may come back as Date or Number depending on
    // calamine's format detection, but the numeric value must match.
    match &row[3] {
        Value::Date(d) => {
            assert!((d.serial - 45000.0).abs() < 0.001, "date serial mismatch: {}", d.serial);
        }
        Value::Number(n) => {
            assert!((*n - 45000.0).abs() < 0.001, "date serial mismatch: {n}");
        }
        other => panic!("expected Date or Number for date cell, got {other:?}"),
    }
    // Error written as string — round-trips as text "#DIV/0!"
    assert_eq!(row[4], Value::Text("#DIV/0!".into()));
    // Empty: trailing empties may be trimmed, so row might be len 5
    assert!(
        row.len() <= 5 || row[5] == Value::Empty,
        "expected Empty or absent trailing cell, got {:?}",
        row.get(5),
    );

    assert!(stream.next_row().unwrap().is_none());
}

/// Integer → Number lossy conversion: xlsx stores all numbers as f64.
#[test]
fn round_trip_integer_becomes_number() {
    let tmp = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    {
        let mut w = Writer::create(tmp.path()).unwrap();
        let mut sh = w.add_sheet("Sheet1").unwrap();
        sh.write_row(0, &[Value::Integer(42)]).unwrap();
        drop(sh);
        w.finish().unwrap();
    }

    let mut reader = Reader::open(tmp.path()).unwrap();
    let mut stream = reader.cells("Sheet1").unwrap();
    let (_, row) = stream.next_row().unwrap().expect("expected row 0");
    // Integer(42) → Number(42.0) after round-trip
    assert_eq!(row[0], Value::Number(42.0));
}

// -- Multi-sheet round-trip --------------------------------------------------

#[test]
fn multi_sheet_round_trip_preserves_all_sheets() {
    let fixture = helpers::generate_three_sheet_fixture();
    let mut reader = Reader::open(fixture.path()).unwrap();
    let out = round_trip_all_sheets(&mut reader);

    let mut re_reader = Reader::open(out.path()).unwrap();
    let names = re_reader.sheet_names();
    assert_eq!(names, vec!["Main", "Lookup1", "Lookup2"]);

    // Main: numbers
    let rows = read_all_rows(&mut re_reader, "Main");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].1[0], Value::Number(1.0));
    assert_eq!(rows[0].1[1], Value::Number(2.0));
    assert_eq!(rows[1].1[0], Value::Number(3.0));
    assert_eq!(rows[1].1[1], Value::Number(4.0));

    // Lookup1: text
    let rows = read_all_rows(&mut re_reader, "Lookup1");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].1[0], Value::Text("apple".into()));
    assert_eq!(rows[0].1[1], Value::Text("banana".into()));
    assert_eq!(rows[1].1[0], Value::Text("cherry".into()));
    assert_eq!(rows[1].1[1], Value::Text("date".into()));

    // Lookup2: mixed
    let rows = read_all_rows(&mut re_reader, "Lookup2");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].1[0], Value::Number(100.0));
    assert_eq!(rows[0].1[1], Value::Text("mixed".into()));
    assert_eq!(rows[0].1[2], Value::Bool(false));
}

#[test]
fn sheet_order_is_preserved() {
    let fixture = helpers::generate_three_sheet_fixture();
    let mut reader = Reader::open(fixture.path()).unwrap();
    let out = round_trip_all_sheets(&mut reader);

    let re_reader = Reader::open(out.path()).unwrap();
    assert_eq!(re_reader.sheet_names(), vec!["Main", "Lookup1", "Lookup2"]);
}

// -- Date round-trip ---------------------------------------------------------

#[test]
fn date_round_trip_contemporary() {
    // Serial 45000 ~ 2023-03-15
    let fixture = helpers::generate_date_fixture(45000.0);
    let mut reader = Reader::open(fixture.path()).unwrap();
    let out = round_trip_all_sheets(&mut reader);

    let mut re_reader = Reader::open(out.path()).unwrap();
    let mut stream = re_reader.cells("Sheet1").unwrap();
    let (_, row) = stream.next_row().unwrap().expect("expected row 0");

    match &row[0] {
        Value::Date(d) => {
            assert!((d.serial - 45000.0).abs() < 0.001, "serial mismatch: {}", d.serial);
        }
        Value::Number(n) => {
            assert!((*n - 45000.0).abs() < 0.001, "serial mismatch: {n}");
        }
        other => panic!("expected Date or Number, got {other:?}"),
    }
}

#[test]
fn date_round_trip_serial_60() {
    // Serial 60 = 1900-02-29 (the Lotus leap bug date)
    let fixture = helpers::generate_date_fixture(60.0);
    let mut reader = Reader::open(fixture.path()).unwrap();
    let out = round_trip_all_sheets(&mut reader);

    let mut re_reader = Reader::open(out.path()).unwrap();
    let mut stream = re_reader.cells("Sheet1").unwrap();
    let (_, row) = stream.next_row().unwrap().expect("expected row 0");

    match &row[0] {
        Value::Date(d) => {
            assert!((d.serial - 60.0).abs() < 0.001, "serial mismatch: {}", d.serial);
        }
        Value::Number(n) => {
            assert!((*n - 60.0).abs() < 0.001, "serial mismatch: {n}");
        }
        other => panic!("expected Date or Number, got {other:?}"),
    }
}

// -- Formula round-trip ------------------------------------------------------

#[test]
fn formula_round_trip() {
    let fixture = helpers::generate_formula_fixture();
    let mut reader = Reader::open(fixture.path()).unwrap();

    // Read cached values
    let rows = read_all_rows(&mut reader, "Sheet1");
    assert_eq!(rows.len(), 1);
    let (_, ref vals) = rows[0];
    assert_eq!(vals[0], Value::Number(1.0));
    assert_eq!(vals[1], Value::Number(2.0));
    assert_eq!(vals[2], Value::Number(3.0));

    // Read formula text
    let formulas = reader.formulas("Sheet1").unwrap();
    assert_eq!(formulas.len(), 1);
    let (frow, fcol, ref ftext) = formulas[0];
    assert_eq!(frow, 0);
    assert_eq!(fcol, 2);

    // Write: data cells + formula with cached value
    let tmp = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    {
        let mut w = Writer::create(tmp.path()).unwrap();
        let mut sh = w.add_sheet("Sheet1").unwrap();
        sh.write_row(0, &[Value::Number(1.0), Value::Number(2.0)]).unwrap();
        sh.write_formula(0, 2, &format!("={ftext}"), &Value::Number(3.0)).unwrap();
        drop(sh);
        w.finish().unwrap();
    }

    // Re-read and verify cached value
    let mut re_reader = Reader::open(tmp.path()).unwrap();
    {
        let mut stream = re_reader.cells("Sheet1").unwrap();
        let (_, row) = stream.next_row().unwrap().expect("expected row 0");
        assert_eq!(row[0], Value::Number(1.0));
        assert_eq!(row[1], Value::Number(2.0));
        assert_eq!(row[2], Value::Number(3.0));
    }

    // Verify formula text survived
    let re_formulas = re_reader.formulas("Sheet1").unwrap();
    assert_eq!(re_formulas.len(), 1);
    // calamine strips the leading '=' from formula text
    assert!(
        re_formulas[0].2.contains("A1") && re_formulas[0].2.contains("B1"),
        "formula text should reference A1 and B1, got: {}",
        re_formulas[0].2,
    );
}

// -- Edge cases --------------------------------------------------------------

#[test]
fn empty_sheet_round_trip() {
    let fixture = helpers::generate_empty_sheet_fixture();
    let mut reader = Reader::open(fixture.path()).unwrap();
    let out = round_trip_all_sheets(&mut reader);

    let mut re_reader = Reader::open(out.path()).unwrap();
    let names = re_reader.sheet_names();
    assert_eq!(names, vec!["Empty"]);

    let mut stream = re_reader.cells("Empty").unwrap();
    assert!(stream.next_row().unwrap().is_none(), "empty sheet should yield no rows");
}

/// Verify calamine's `formulas()` returns text WITHOUT leading `=`.
/// If this fails after a calamine upgrade, update the formula text
/// storage in xlstream-eval to handle the new format.
#[test]
fn calamine_formulas_returns_text_without_equals_prefix() {
    let fixture = helpers::generate_formula_fixture();
    let mut reader = Reader::open(fixture.path()).unwrap();
    let formulas = reader.formulas("Sheet1").unwrap();
    assert!(!formulas.is_empty(), "fixture should have formulas");
    for (_, _, text) in &formulas {
        assert!(!text.starts_with('='), "expected formula text without '=' prefix, got: {text}");
    }
}
