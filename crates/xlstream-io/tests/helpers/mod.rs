//! Shared fixture generators for xlstream-io integration tests.
//! Each function writes a minimal xlsx via `rust_xlsxwriter` and returns
//! a `NamedTempFile` whose path can be passed to `Reader::open`.

use rust_xlsxwriter::{Formula, Workbook};
use tempfile::NamedTempFile;

/// 2 rows x 3 cols: numbers, strings, booleans.
///
/// | A     | B       | C    |
/// |-------|---------|------|
/// | 1.0   | "hello" | true |
/// | 2.0   | "world" | false|
pub fn generate_simple_fixture() -> NamedTempFile {
    let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();

    ws.write_number(0, 0, 1.0).unwrap();
    ws.write_string(0, 1, "hello").unwrap();
    ws.write_boolean(0, 2, true).unwrap();

    ws.write_number(1, 0, 2.0).unwrap();
    ws.write_string(1, 1, "world").unwrap();
    ws.write_boolean(1, 2, false).unwrap();

    wb.save(tmp.path()).unwrap();
    tmp
}

/// Row with gap: A1=1, C1=3 (B1 missing).
///
/// | A   | B     | C   |
/// |-----|-------|-----|
/// | 1.0 | empty | 3.0 |
pub fn generate_sparse_fixture() -> NamedTempFile {
    let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();

    ws.write_number(0, 0, 1.0).unwrap();
    // skip B1
    ws.write_number(0, 2, 3.0).unwrap();

    wb.save(tmp.path()).unwrap();
    tmp
}

/// Row with formula: A1=1, B1=2, C1=formula "=A1+B1" with cached result "3".
///
/// | A   | B   | C        |
/// |-----|-----|----------|
/// | 1.0 | 2.0 | =A1+B1  |
pub fn generate_formula_fixture() -> NamedTempFile {
    let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();

    ws.write_number(0, 0, 1.0).unwrap();
    ws.write_number(0, 1, 2.0).unwrap();
    ws.write_formula(0, 2, Formula::new("=A1+B1").set_result("3")).unwrap();

    wb.save(tmp.path()).unwrap();
    tmp
}

/// Multi-sheet workbook with sheets named "Alpha" and "Beta".
/// Alpha: A1=10, Beta: A1=20.
pub fn generate_multi_sheet_fixture() -> NamedTempFile {
    let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();

    let ws1 = wb.add_worksheet();
    ws1.set_name("Alpha").unwrap();
    ws1.write_number(0, 0, 10.0).unwrap();

    let ws2 = wb.add_worksheet();
    ws2.set_name("Beta").unwrap();
    ws2.write_number(0, 0, 20.0).unwrap();

    wb.save(tmp.path()).unwrap();
    tmp
}
