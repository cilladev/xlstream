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

/// Every Value variant in a single row: Number, Text, Bool, Date, Error, Empty.
///
/// | A    | B       | C    | D (date serial 45000) | E (#DIV/0!) | F (empty) |
/// |------|---------|------|-----------------------|-------------|-----------|
/// | 42.0 | "hello" | true | 45000.0               | "#DIV/0!"   |           |
pub fn generate_all_types_fixture() -> NamedTempFile {
    let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();

    ws.write_number(0, 0, 42.0).unwrap();
    ws.write_string(0, 1, "hello").unwrap();
    ws.write_boolean(0, 2, true).unwrap();
    let date_fmt = rust_xlsxwriter::Format::new().set_num_format("yyyy-mm-dd");
    ws.write_number_with_format(0, 3, 45000.0, &date_fmt).unwrap();
    // Error: written as string since rust_xlsxwriter has no error-cell API
    ws.write_string(0, 4, "#DIV/0!").unwrap();
    // Column F left empty

    wb.save(tmp.path()).unwrap();
    tmp
}

/// Three sheets with distinct data: Main (numbers), Lookup1 (text), Lookup2 (mixed).
pub fn generate_three_sheet_fixture() -> NamedTempFile {
    let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();

    let ws1 = wb.add_worksheet();
    ws1.set_name("Main").unwrap();
    ws1.write_number(0, 0, 1.0).unwrap();
    ws1.write_number(0, 1, 2.0).unwrap();
    ws1.write_number(1, 0, 3.0).unwrap();
    ws1.write_number(1, 1, 4.0).unwrap();

    let ws2 = wb.add_worksheet();
    ws2.set_name("Lookup1").unwrap();
    ws2.write_string(0, 0, "apple").unwrap();
    ws2.write_string(0, 1, "banana").unwrap();
    ws2.write_string(1, 0, "cherry").unwrap();
    ws2.write_string(1, 1, "date").unwrap();

    let ws3 = wb.add_worksheet();
    ws3.set_name("Lookup2").unwrap();
    ws3.write_number(0, 0, 100.0).unwrap();
    ws3.write_string(0, 1, "mixed").unwrap();
    ws3.write_boolean(0, 2, false).unwrap();

    wb.save(tmp.path()).unwrap();
    tmp
}

/// Single cell containing a date serial value with date formatting.
pub fn generate_date_fixture(serial: f64) -> NamedTempFile {
    let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    let date_fmt = rust_xlsxwriter::Format::new().set_num_format("yyyy-mm-dd");
    ws.write_number_with_format(0, 0, serial, &date_fmt).unwrap();
    wb.save(tmp.path()).unwrap();
    tmp
}

/// Empty sheet (no data written).
pub fn generate_empty_sheet_fixture() -> NamedTempFile {
    let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    ws.set_name("Empty").unwrap();
    wb.save(tmp.path()).unwrap();
    tmp
}
