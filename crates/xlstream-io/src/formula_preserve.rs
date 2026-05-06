//! Copy-and-replace engine for preserving formulas in output xlsx.
//!
//! Streams worksheet XML through `quick_xml`, replacing only the `<v>`
//! (cached value) content for formula cells. The `<f>` (formula text)
//! passes through untouched.

use std::collections::HashMap;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Reader as XmlReader;
use quick_xml::Writer as XmlWriter;
use xlstream_core::{a1_to_col_row, XlStreamError};
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

use crate::convert::CellResult;

/// State machine for tracking position within `<c>` elements.
enum CellState {
    Normal,
    /// Inside a `<c>` whose (row, col) is in the results map.
    FormulaCell {
        row: u32,
        col: u16,
        has_value: bool,
    },
    /// Inside the `<v>` of a formula cell.
    InValue {
        row: u32,
        col: u16,
    },
}

/// Stream worksheet XML, replacing `<v>` content for cells in `results`.
///
/// The `<f>` element passes through byte-for-byte. Only `<v>` text and
/// the `t` attribute on `<c>` are modified.
///
/// `results` is keyed by 0-based `(row, col)`.
///
/// # Errors
///
/// Returns [`XlStreamError::Internal`] on XML parse or write failures.
#[allow(clippy::implicit_hasher)]
pub fn replace_sheet_values(
    xml_in: &[u8],
    results: &HashMap<(u32, u16), CellResult>,
) -> Result<Vec<u8>, XlStreamError> {
    let mut reader = XmlReader::from_reader(xml_in);
    reader.config_mut().trim_text_start = false;
    reader.config_mut().trim_text_end = false;

    let mut writer = XmlWriter::new(Vec::with_capacity(xml_in.len()));
    let mut state = CellState::Normal;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"c" => {
                if let Some((row, col)) = parse_cell_position(e) {
                    if let Some(result) = results.get(&(row, col)) {
                        let modified = rewrite_cell_start(e, result.cell_type);
                        write_event(&mut writer, Event::Start(modified))?;
                        state = CellState::FormulaCell { row, col, has_value: false };
                        buf.clear();
                        continue;
                    }
                }
                write_event(&mut writer, Event::Start(e.clone().into_owned()))?;
                state = CellState::Normal;
            }

            Ok(Event::Start(ref e)) if e.name().as_ref() == b"v" => {
                write_event(&mut writer, Event::Start(e.clone().into_owned()))?;
                if let CellState::FormulaCell { row, col, ref mut has_value } = state {
                    *has_value = true;
                    state = CellState::InValue { row, col };
                }
            }

            Ok(Event::Text(ref _t)) if matches!(state, CellState::InValue { .. }) => {
                let CellState::InValue { row, col } = state else {
                    return Err(XlStreamError::Internal("unexpected state".into()));
                };
                let result = &results[&(row, col)];
                write_event(&mut writer, Event::Text(BytesText::new(&result.value)))?;
                state = CellState::FormulaCell { row, col, has_value: true };
            }

            Ok(Event::End(ref e))
                if e.name().as_ref() == b"v" && matches!(state, CellState::InValue { .. }) =>
            {
                let CellState::InValue { row, col } = state else {
                    return Err(XlStreamError::Internal("unexpected state".into()));
                };
                write_event(&mut writer, Event::End(e.clone().into_owned()))?;
                state = CellState::FormulaCell { row, col, has_value: true };
            }

            Ok(Event::Empty(ref e))
                if e.name().as_ref() == b"v" && matches!(state, CellState::FormulaCell { .. }) =>
            {
                if let CellState::FormulaCell { row, col, ref mut has_value } = state {
                    *has_value = true;
                    let result = &results[&(row, col)];
                    write_event(&mut writer, Event::Start(BytesStart::new("v")))?;
                    write_event(&mut writer, Event::Text(BytesText::new(&result.value)))?;
                    write_event(&mut writer, Event::End(BytesEnd::new("v")))?;
                }
            }

            Ok(Event::End(ref e)) if e.name().as_ref() == b"c" => {
                if let CellState::FormulaCell { row, col, has_value: false } = state {
                    let result = &results[&(row, col)];
                    write_event(&mut writer, Event::Start(BytesStart::new("v")))?;
                    write_event(&mut writer, Event::Text(BytesText::new(&result.value)))?;
                    write_event(&mut writer, Event::End(BytesEnd::new("v")))?;
                }
                write_event(&mut writer, Event::End(e.clone().into_owned()))?;
                state = CellState::Normal;
            }

            Ok(Event::Eof) => break,

            Ok(event) => {
                write_event(&mut writer, event.into_owned())?;
            }

            Err(e) => return Err(XlStreamError::Internal(format!("XML parse error: {e}"))),
        }
        buf.clear();
    }

    Ok(writer.into_inner())
}

/// Map sheet names to their zip entry paths by parsing `xl/workbook.xml`
/// and `xl/_rels/workbook.xml.rels`.
///
/// # Errors
///
/// Returns [`XlStreamError::Internal`] if the archive lacks the
/// expected workbook XML entries.
pub fn resolve_sheet_paths<R: Read + std::io::Seek>(
    archive: &mut ZipArchive<R>,
) -> Result<HashMap<String, String>, XlStreamError> {
    let name_to_rid = parse_workbook_sheets(archive)?;
    let rid_to_target = parse_workbook_rels(archive)?;

    let mut result = HashMap::new();
    for (rid, sheet_name) in &name_to_rid {
        if let Some(target) = rid_to_target.get(rid) {
            let full_path = if let Some(stripped) = target.strip_prefix('/') {
                stripped.to_string()
            } else {
                format!("xl/{target}")
            };
            result.insert(sheet_name.clone(), full_path);
        }
    }
    Ok(result)
}

/// Copy an xlsx from `input` to `output`, replacing `<v>` values for
/// formula cells according to `results`.
///
/// Non-worksheet entries (styles, shared strings, images, etc.) are
/// copied byte-for-byte from the input archive.
///
/// # Errors
///
/// - [`XlStreamError::Io`] on filesystem errors.
/// - [`XlStreamError::Internal`] on zip or XML parse failures.
#[allow(clippy::implicit_hasher)]
pub fn copy_and_replace(
    input: &Path,
    output: &Path,
    results: &HashMap<String, HashMap<(u32, u16), CellResult>>,
) -> Result<(), XlStreamError> {
    let input_file = std::fs::File::open(input)
        .map_err(|e| XlStreamError::Io { path: input.to_path_buf(), source: e })?;
    let mut archive = ZipArchive::new(BufReader::new(input_file))
        .map_err(|e| XlStreamError::Internal(format!("zip open: {e}")))?;

    let sheet_paths = resolve_sheet_paths(&mut archive)?;
    let path_to_sheet: HashMap<String, String> =
        sheet_paths.into_iter().map(|(name, path)| (path, name)).collect();

    let output_file = std::fs::File::create(output)
        .map_err(|e| XlStreamError::Io { path: output.to_path_buf(), source: e })?;
    let mut zip_writer = ZipWriter::new(BufWriter::new(output_file));

    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| XlStreamError::Internal(format!("zip entry {i}: {e}")))?;
        let entry_name = entry.name().to_string();

        let sheet_results = path_to_sheet.get(&entry_name).and_then(|name| results.get(name));

        if let Some(sr) = sheet_results {
            if !sr.is_empty() {
                let mut xml_in = Vec::new();
                entry
                    .read_to_end(&mut xml_in)
                    .map_err(|e| XlStreamError::Internal(format!("read {entry_name}: {e}")))?;

                let xml_out = replace_sheet_values(&xml_in, sr)?;

                let options = SimpleFileOptions::default().compression_method(entry.compression());
                zip_writer
                    .start_file(&entry_name, options)
                    .map_err(|e| XlStreamError::Internal(format!("zip write {entry_name}: {e}")))?;
                zip_writer
                    .write_all(&xml_out)
                    .map_err(|e| XlStreamError::Internal(e.to_string()))?;
                continue;
            }
        }

        zip_writer
            .raw_copy_file(entry)
            .map_err(|e| XlStreamError::Internal(format!("zip copy {entry_name}: {e}")))?;
    }

    zip_writer.finish().map_err(|e| XlStreamError::Internal(format!("zip finish: {e}")))?;

    Ok(())
}

// -- helpers --

fn write_event<W: std::io::Write>(
    writer: &mut XmlWriter<W>,
    event: Event<'_>,
) -> Result<(), XlStreamError> {
    writer.write_event(event).map_err(|e| XlStreamError::Internal(e.to_string()))
}

fn parse_cell_position(elem: &BytesStart<'_>) -> Option<(u32, u16)> {
    for attr in elem.attributes().filter_map(Result::ok) {
        if attr.key.as_ref() == b"r" {
            let ref_str = std::str::from_utf8(&attr.value).ok()?;
            return a1_to_col_row(ref_str);
        }
    }
    None
}

fn rewrite_cell_start(
    elem: &BytesStart<'_>,
    cell_type: Option<&'static str>,
) -> BytesStart<'static> {
    let name = std::str::from_utf8(elem.name().as_ref()).unwrap_or("c").to_owned();
    let mut new_elem = BytesStart::new(name);
    for attr in elem.attributes().filter_map(Result::ok) {
        if attr.key.as_ref() != b"t" {
            new_elem.push_attribute((
                std::str::from_utf8(attr.key.as_ref()).unwrap_or(""),
                std::str::from_utf8(&attr.value).unwrap_or(""),
            ));
        }
    }
    if let Some(t) = cell_type {
        new_elem.push_attribute(("t", t));
    }
    new_elem
}

fn parse_workbook_sheets<R: Read + std::io::Seek>(
    archive: &mut ZipArchive<R>,
) -> Result<HashMap<String, String>, XlStreamError> {
    let mut map = HashMap::new();
    let entry = archive
        .by_name("xl/workbook.xml")
        .map_err(|e| XlStreamError::Internal(format!("missing workbook.xml: {e}")))?;
    let mut xml = String::new();
    BufReader::new(entry)
        .read_to_string(&mut xml)
        .map_err(|e| XlStreamError::Internal(e.to_string()))?;

    let mut reader = XmlReader::from_str(&xml);
    loop {
        match reader.read_event() {
            Ok(Event::Empty(ref e) | Event::Start(ref e)) if e.name().as_ref() == b"sheet" => {
                let mut name = None;
                let mut rid = None;
                for attr in e.attributes().filter_map(Result::ok) {
                    match attr.key.as_ref() {
                        b"name" => {
                            name = std::str::from_utf8(&attr.value).ok().map(String::from);
                        }
                        // The namespace-prefixed form (r:id) is the standard.
                        k if k.ends_with(b"id") && k != b"sheetId" => {
                            rid = std::str::from_utf8(&attr.value).ok().map(String::from);
                        }
                        _ => {}
                    }
                }
                if let (Some(n), Some(r)) = (name, rid) {
                    map.insert(r, n);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(XlStreamError::Internal(format!("workbook.xml parse: {e}")));
            }
            _ => {}
        }
    }
    Ok(map)
}

fn parse_workbook_rels<R: Read + std::io::Seek>(
    archive: &mut ZipArchive<R>,
) -> Result<HashMap<String, String>, XlStreamError> {
    let mut map = HashMap::new();
    let entry = archive
        .by_name("xl/_rels/workbook.xml.rels")
        .map_err(|e| XlStreamError::Internal(format!("missing workbook.xml.rels: {e}")))?;
    let mut xml = String::new();
    BufReader::new(entry)
        .read_to_string(&mut xml)
        .map_err(|e| XlStreamError::Internal(e.to_string()))?;

    let mut reader = XmlReader::from_str(&xml);
    loop {
        match reader.read_event() {
            Ok(Event::Empty(ref e) | Event::Start(ref e))
                if e.name().as_ref() == b"Relationship" =>
            {
                let mut rid = None;
                let mut target = None;
                for attr in e.attributes().filter_map(Result::ok) {
                    match attr.key.as_ref() {
                        b"Id" => {
                            rid = std::str::from_utf8(&attr.value).ok().map(String::from);
                        }
                        b"Target" => {
                            target = std::str::from_utf8(&attr.value).ok().map(String::from);
                        }
                        _ => {}
                    }
                }
                if let (Some(r), Some(t)) = (rid, target) {
                    map.insert(r, t);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(XlStreamError::Internal(format!("rels parse: {e}")));
            }
            _ => {}
        }
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use std::collections::HashMap;

    use calamine::Reader as CalReader;

    use super::*;
    use crate::convert::CellResult;

    fn make_results(
        entries: &[(u32, u16, &str, Option<&'static str>)],
    ) -> HashMap<(u32, u16), CellResult> {
        entries
            .iter()
            .map(|(r, c, v, t)| ((*r, *c), CellResult { value: (*v).to_string(), cell_type: *t }))
            .collect()
    }

    #[test]
    fn replaces_number_value_in_formula_cell() {
        let xml = br#"<?xml version="1.0"?><worksheet><sheetData><row r="2"><c r="A2"><v>10</v></c><c r="B2"><f>A2*2</f><v>20</v></c></row></sheetData></worksheet>"#;
        let results = make_results(&[(1, 1, "42", None)]);
        let out = replace_sheet_values(xml, &results).unwrap();
        let out_str = std::str::from_utf8(&out).unwrap();
        assert!(out_str.contains("<f>A2*2</f>"), "formula must be preserved");
        assert!(out_str.contains("<v>42</v>"), "value must be replaced, got: {out_str}");
        assert!(!out_str.contains("<v>20</v>"), "old value must not appear");
    }

    #[test]
    fn preserves_data_cells_unchanged() {
        let xml = br#"<?xml version="1.0"?><worksheet><sheetData><row r="2"><c r="A2"><v>10</v></c></row></sheetData></worksheet>"#;
        let results = HashMap::new();
        let out = replace_sheet_values(xml, &results).unwrap();
        let out_str = std::str::from_utf8(&out).unwrap();
        assert!(out_str.contains("<v>10</v>"), "data cell must be unchanged");
    }

    #[test]
    fn updates_cell_type_for_string_result() {
        let xml = br#"<?xml version="1.0"?><worksheet><sheetData><row r="2"><c r="B2"><f>A2&amp;"!"</f><v>hi</v></c></row></sheetData></worksheet>"#;
        let results = make_results(&[(1, 1, "hello", Some("str"))]);
        let out = replace_sheet_values(xml, &results).unwrap();
        let out_str = std::str::from_utf8(&out).unwrap();
        assert!(out_str.contains(r#"t="str""#), "t must be set to str: {out_str}");
        assert!(out_str.contains("<v>hello</v>"), "value must be replaced");
    }

    #[test]
    fn updates_cell_type_for_boolean_result() {
        let xml = br#"<?xml version="1.0"?><worksheet><sheetData><row r="2"><c r="B2"><f>A2&gt;0</f><v>0</v></c></row></sheetData></worksheet>"#;
        let results = make_results(&[(1, 1, "1", Some("b"))]);
        let out = replace_sheet_values(xml, &results).unwrap();
        let out_str = std::str::from_utf8(&out).unwrap();
        assert!(out_str.contains(r#"t="b""#), "t must be set to b: {out_str}");
        assert!(out_str.contains("<v>1</v>"));
    }

    #[test]
    fn updates_cell_type_for_error_result() {
        let xml = br#"<?xml version="1.0"?><worksheet><sheetData><row r="2"><c r="B2"><f>1/0</f><v>0</v></c></row></sheetData></worksheet>"#;
        let results = make_results(&[(1, 1, "#DIV/0!", Some("e"))]);
        let out = replace_sheet_values(xml, &results).unwrap();
        let out_str = std::str::from_utf8(&out).unwrap();
        assert!(out_str.contains(r#"t="e""#));
        assert!(out_str.contains("<v>#DIV/0!</v>"));
    }

    #[test]
    fn inserts_value_when_no_v_element_exists() {
        let xml = br#"<?xml version="1.0"?><worksheet><sheetData><row r="2"><c r="B2"><f>A2*2</f></c></row></sheetData></worksheet>"#;
        let results = make_results(&[(1, 1, "42", None)]);
        let out = replace_sheet_values(xml, &results).unwrap();
        let out_str = std::str::from_utf8(&out).unwrap();
        assert!(out_str.contains("<v>42</v>"), "must insert <v>: {out_str}");
    }

    #[test]
    fn removes_old_type_for_number_result() {
        let xml = br#"<?xml version="1.0"?><worksheet><sheetData><row r="2"><c r="B2" t="e"><f>1/0</f><v>#DIV/0!</v></c></row></sheetData></worksheet>"#;
        let results = make_results(&[(1, 1, "42", None)]);
        let out = replace_sheet_values(xml, &results).unwrap();
        let out_str = std::str::from_utf8(&out).unwrap();
        assert!(!out_str.contains(r#"t="e""#), "old t must be removed: {out_str}");
        assert!(out_str.contains("<v>42</v>"));
    }

    #[test]
    fn preserves_style_attribute() {
        let xml = br#"<?xml version="1.0"?><worksheet><sheetData><row r="2"><c r="B2" s="4"><f>A2*2</f><v>20</v></c></row></sheetData></worksheet>"#;
        let results = make_results(&[(1, 1, "42", None)]);
        let out = replace_sheet_values(xml, &results).unwrap();
        let out_str = std::str::from_utf8(&out).unwrap();
        assert!(out_str.contains(r#"s="4""#), "style must be preserved: {out_str}");
    }

    #[test]
    fn handles_mixed_formula_and_data_rows() {
        let xml = br#"<?xml version="1.0"?><worksheet><sheetData><row r="1"><c r="A1" t="s"><v>0</v></c><c r="B1" t="s"><v>1</v></c></row><row r="2"><c r="A2"><v>10</v></c><c r="B2"><f>A2*2</f><v>20</v></c></row><row r="3"><c r="A3"><v>30</v></c><c r="B3"><f>A3*2</f><v>60</v></c></row></sheetData></worksheet>"#;
        let results = make_results(&[(1, 1, "42", None), (2, 1, "99", None)]);
        let out = replace_sheet_values(xml, &results).unwrap();
        let out_str = std::str::from_utf8(&out).unwrap();
        assert!(out_str.contains(r#"<c r="A1" t="s"><v>0</v></c>"#));
        assert!(out_str.contains("<v>42</v>"));
        assert!(out_str.contains("<v>99</v>"));
    }

    #[test]
    fn resolve_sheet_paths_parses_workbook_xml() {
        let tmp = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
        {
            let mut wb = rust_xlsxwriter::Workbook::new();
            let ws1 = wb.add_worksheet();
            ws1.set_name("Data").unwrap();
            ws1.write_number(0, 0, 1.0).unwrap();
            let ws2 = wb.add_worksheet();
            ws2.set_name("Summary").unwrap();
            ws2.write_number(0, 0, 2.0).unwrap();
            wb.save(tmp.path()).unwrap();
        }
        let file = std::fs::File::open(tmp.path()).unwrap();
        let mut archive = ZipArchive::new(BufReader::new(file)).unwrap();
        let paths = resolve_sheet_paths(&mut archive).unwrap();
        assert!(paths.contains_key("Data"), "missing Data: {paths:?}");
        assert!(paths.contains_key("Summary"), "missing Summary: {paths:?}");
        for path in paths.values() {
            assert!(path.contains("worksheets/"), "bad path: {path}");
        }
    }

    #[test]
    fn copy_and_replace_round_trips_simple_workbook() {
        let input = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
        {
            let mut wb = rust_xlsxwriter::Workbook::new();
            let ws = wb.add_worksheet();
            ws.set_name("Sheet1").unwrap();
            ws.write_number(0, 0, 10.0).unwrap();
            ws.write_formula(0, 1, "=A1*2").unwrap();
            ws.write_number(1, 0, 30.0).unwrap();
            ws.write_formula(1, 1, "=A2*2").unwrap();
            wb.save(input.path()).unwrap();
        }

        let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();

        let mut sheet_results = HashMap::new();
        let mut cells = HashMap::new();
        cells.insert((0u32, 1u16), CellResult { value: "42".into(), cell_type: None });
        cells.insert((1, 1), CellResult { value: "99".into(), cell_type: None });
        sheet_results.insert("Sheet1".to_string(), cells);

        copy_and_replace(input.path(), output.path(), &sheet_results).unwrap();

        let mut wb: calamine::Xlsx<_> = calamine::open_workbook(output.path()).unwrap();
        let range = wb.worksheet_range("Sheet1").unwrap();
        let rows: Vec<_> = range.rows().collect();

        assert_eq!(rows[0][1], calamine::Data::Float(42.0));
        assert_eq!(rows[1][1], calamine::Data::Float(99.0));
        assert_eq!(rows[0][0], calamine::Data::Float(10.0));
    }
}
