#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[allow(dead_code)]
mod helpers;

use xlstream_core::Value;
use xlstream_io::{Reader, Writer};

/// Debug builds are ~4-5x slower than release. Apply a multiplier so the
/// smoke tests pass in both profiles. The *real* budget is the release number;
/// the debug multiplier just keeps `cargo test` green.
const fn time_limit(release_secs: u64) -> u64 {
    if cfg!(debug_assertions) {
        release_secs * 5
    } else {
        release_secs
    }
}

fn generate_large_fixture(rows: u32, cols: u16) -> tempfile::NamedTempFile {
    let f = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = rust_xlsxwriter::Workbook::new();
    let ws = wb.add_worksheet_with_constant_memory();
    ws.set_name("Data").unwrap();
    for r in 0..rows {
        for c in 0..cols {
            ws.write_number(r, c, f64::from(r * u32::from(c.wrapping_add(1)))).unwrap();
        }
    }
    wb.save(f.path()).unwrap();
    f
}

#[test]
fn read_100k_rows_under_5_seconds() {
    let f = generate_large_fixture(100_000, 20);
    let limit = time_limit(5);
    let start = std::time::Instant::now();
    let mut reader = Reader::open(f.path()).unwrap();
    let mut stream = reader.cells("Data").unwrap();
    let mut count = 0u32;
    while stream.next_row().unwrap().is_some() {
        count += 1;
    }
    let elapsed = start.elapsed();
    assert_eq!(count, 100_000);
    assert!(elapsed.as_secs() < limit, "read took {elapsed:?}, expected < {limit}s");
    eprintln!("read_100k: {elapsed:?}");
}

#[test]
fn write_100k_rows_under_3_seconds() {
    let limit = time_limit(3);
    let f = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let start = std::time::Instant::now();
    let mut writer = Writer::create(f.path()).unwrap();
    let mut sheet = writer.add_sheet("Data").unwrap();
    let row_data: Vec<Value> = (0..20).map(|i| Value::Number(f64::from(i))).collect();
    for r in 0..100_000u32 {
        sheet.write_row(r, &row_data).unwrap();
    }
    drop(sheet);
    writer.finish().unwrap();
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < limit, "write took {elapsed:?}, expected < {limit}s");
    eprintln!("write_100k: {elapsed:?}");
}

#[test]
fn round_trip_100k_under_10_seconds() {
    let limit = time_limit(10);
    let input = generate_large_fixture(100_000, 20);
    let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let start = std::time::Instant::now();

    // Read
    let mut reader = Reader::open(input.path()).unwrap();
    let mut stream = reader.cells("Data").unwrap();
    let mut writer = Writer::create(output.path()).unwrap();
    let mut sheet = writer.add_sheet("Data").unwrap();
    while let Some((row_idx, row)) = stream.next_row().unwrap() {
        sheet.write_row(row_idx, &row).unwrap();
    }
    drop(sheet);
    writer.finish().unwrap();

    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < limit, "round-trip took {elapsed:?}, expected < {limit}s");
    eprintln!("round_trip_100k: {elapsed:?}");
}
