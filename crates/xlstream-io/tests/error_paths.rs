#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

//! Error-path integration tests for Reader and Writer.

use std::path::Path;

use xlstream_core::XlStreamError;
use xlstream_io::Reader;

#[test]
fn open_nonexistent_file_returns_xlsx_error() {
    let err = Reader::open(Path::new("/tmp/xlstream_does_not_exist_9999.xlsx")).unwrap_err();
    assert!(
        matches!(err, XlStreamError::Xlsx(_)),
        "expected Xlsx error for missing file, got {err:?}",
    );
}

#[test]
fn open_malformed_file_returns_xlsx_error() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(tmp.path(), b"this is not a valid xlsx file").unwrap();
    let err = Reader::open(tmp.path()).unwrap_err();
    assert!(
        matches!(err, XlStreamError::Xlsx(_)),
        "expected Xlsx error for malformed file, got {err:?}",
    );
}

#[test]
fn open_empty_file_returns_xlsx_error() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    let err = Reader::open(tmp.path()).unwrap_err();
    assert!(
        matches!(err, XlStreamError::Xlsx(_)),
        "expected Xlsx error for empty file, got {err:?}",
    );
}

#[test]
fn cells_nonexistent_sheet_returns_xlsx_error() {
    let tmp = {
        let mut wb = rust_xlsxwriter::Workbook::new();
        let ws = wb.add_worksheet();
        ws.write_number(0, 0, 1.0).unwrap();
        let f = tempfile::NamedTempFile::new().unwrap();
        wb.save(f.path()).unwrap();
        f
    };
    let mut reader = Reader::open(tmp.path()).unwrap();
    let err = reader.cells("NoSuchSheet").unwrap_err();
    assert!(
        matches!(err, XlStreamError::Xlsx(_)),
        "expected Xlsx error for nonexistent sheet, got {err:?}",
    );
}
