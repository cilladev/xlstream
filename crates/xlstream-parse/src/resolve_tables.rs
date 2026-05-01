//! Table reference resolution — rewrites `Reference::Table` nodes in the AST
//! by looking up table metadata and resolving specifiers to cell/range refs.

use std::collections::HashMap;
use std::hash::BuildHasher;

use crate::ast::{Ast, Node};
use crate::references::Reference;

/// Minimal table metadata needed for resolution. Mirrors the IO layer's
/// `TableMeta` to avoid a crate dependency.
///
/// # Examples
///
/// ```
/// use xlstream_parse::TableInfo;
/// let info = TableInfo {
///     sheet_name: "Sheet1".into(),
///     columns: vec!["A".into(), "B".into()],
///     header_row: 0,
///     data_start_row: 1,
///     data_end_row: 10,
///     start_col: 0,
/// };
/// assert_eq!(info.column_offset("b"), Some(1));
/// ```
#[derive(Debug, Clone)]
pub struct TableInfo {
    /// Sheet the table lives on.
    pub sheet_name: String,
    /// Column header names, in order.
    pub columns: Vec<String>,
    /// 0-based row index of the header row.
    pub header_row: u32,
    /// 0-based row index of the first data row.
    pub data_start_row: u32,
    /// 0-based row index of the last data row (inclusive).
    pub data_end_row: u32,
    /// 0-based column index of the first table column.
    pub start_col: u32,
}

impl TableInfo {
    /// Find a column's 0-based offset within this table (case-insensitive).
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_parse::TableInfo;
    /// let info = TableInfo {
    ///     sheet_name: "S".into(),
    ///     columns: vec!["Price".into(), "Qty".into()],
    ///     header_row: 0,
    ///     data_start_row: 1,
    ///     data_end_row: 5,
    ///     start_col: 0,
    /// };
    /// assert_eq!(info.column_offset("price"), Some(0));
    /// assert_eq!(info.column_offset("Missing"), None);
    /// ```
    #[must_use]
    pub fn column_offset(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|c| c.eq_ignore_ascii_case(name))
    }

    /// 1-based absolute column number for a named column.
    #[allow(clippy::cast_possible_truncation)]
    fn absolute_col_1based(&self, name: &str) -> Option<u32> {
        self.column_offset(name).map(|i| self.start_col + i as u32 + 1)
    }
}

/// Resolve all table references in `ast` using the provided metadata.
///
/// The `tables` map must use **lowercase** keys. Table names and column
/// names are matched case-insensitively. `cell_sheet` is used to infer
/// the table for bare `[@Column]` references (empty table name).
/// `current_row` and `current_col` are 1-based cell coordinates used
/// for `[@Column]` resolution — the classifier checks
/// `row == ctx.current_row()`, so `current_row` must match. `current_col`
/// is used for positional table inference on bare `[@Column]` refs.
///
/// Unresolvable references (unknown table, unknown column) are left as
/// `Reference::Table` for the classifier to reject.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use xlstream_parse::{parse, resolve_table_references, TableInfo};
///
/// let ast = parse("SUM(Table1[Amount])").unwrap();
/// let mut tables = HashMap::new();
/// tables.insert("table1".to_string(), TableInfo {
///     sheet_name: "Sheet1".into(),
///     columns: vec!["Region".into(), "Amount".into()],
///     header_row: 0,
///     data_start_row: 1,
///     data_end_row: 10,
///     start_col: 0,
/// });
/// let resolved = resolve_table_references(ast, &tables, Some("Sheet1"), 2, 1);
/// let dbg = format!("{resolved:?}");
/// assert!(dbg.contains("Range"), "expected Range in: {dbg}");
/// ```
#[must_use]
pub fn resolve_table_references<S: BuildHasher>(
    ast: Ast,
    tables: &HashMap<String, TableInfo, S>,
    cell_sheet: Option<&str>,
    current_row: u32,
    current_col: u32,
) -> Ast {
    if tables.is_empty() {
        return ast;
    }
    let new_root = resolve_node(ast.root, tables, cell_sheet, current_row, current_col);
    Ast { upstream: ast.upstream, root: new_root }
}

fn resolve_node<S: BuildHasher>(
    node: Node,
    tables: &HashMap<String, TableInfo, S>,
    cell_sheet: Option<&str>,
    current_row: u32,
    current_col: u32,
) -> Node {
    match node {
        Node::Reference(Reference::Table { ref name, ref specifier }) => resolve_table_ref(
            name,
            specifier.as_deref(),
            tables,
            cell_sheet,
            current_row,
            current_col,
        )
        .unwrap_or(node),
        Node::Function { name, args } => Node::Function {
            name,
            args: args
                .into_iter()
                .map(|a| resolve_node(a, tables, cell_sheet, current_row, current_col))
                .collect(),
        },
        Node::BinaryOp { op, left, right } => Node::BinaryOp {
            op,
            left: Box::new(resolve_node(*left, tables, cell_sheet, current_row, current_col)),
            right: Box::new(resolve_node(*right, tables, cell_sheet, current_row, current_col)),
        },
        Node::UnaryOp { op, expr } => Node::UnaryOp {
            op,
            expr: Box::new(resolve_node(*expr, tables, cell_sheet, current_row, current_col)),
        },
        Node::Array(rows) => Node::Array(
            rows.into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|n| resolve_node(n, tables, cell_sheet, current_row, current_col))
                        .collect()
                })
                .collect(),
        ),
        other @ (Node::Literal(_)
        | Node::Text(_)
        | Node::Error(_)
        | Node::Reference(_)
        | Node::PreludeRef(_)) => other,
    }
}

#[allow(clippy::cast_possible_truncation)]
fn resolve_table_ref<S: BuildHasher>(
    table_name: &str,
    specifier: Option<&str>,
    tables: &HashMap<String, TableInfo, S>,
    cell_sheet: Option<&str>,
    current_row: u32,
    current_col: u32,
) -> Option<Node> {
    let info = if table_name.is_empty() {
        let sheet = cell_sheet?;
        tables.values().find(|t| {
            t.sheet_name.eq_ignore_ascii_case(sheet)
                && current_row > t.header_row
                && current_row <= t.data_end_row + 1
                && current_col > t.start_col
                && current_col <= t.start_col + t.columns.len() as u32
        })?
    } else {
        tables.get(&table_name.to_ascii_lowercase())?
    };

    let spec = specifier.unwrap_or("");
    resolve_specifier(info, spec, current_row)
}

/// Resolve a specifier string against table metadata.
///
/// Specifier string format (from formualizer-parse 2.0.0 Display):
/// - `""` or absent → whole data range
/// - `"ColumnName"` → bounded column data range
/// - `"#Data"` → whole data range (same as empty)
/// - `"#Headers"` → header row range
/// - `"#All"` → header row through last data row
/// - `"#Totals"` → totals row (unsupported — calamine doesn't expose it)
/// - `"Col1:Col3"` → column range
/// - `"[@],[ColumnName]"` → current row, named column
/// - `"@"` → current row, all columns (bare @)
#[allow(clippy::cast_possible_truncation)]
fn resolve_specifier(info: &TableInfo, spec: &str, current_row: u32) -> Option<Node> {
    if let Some(col_part) = parse_this_row_column(spec) {
        let abs_col = info.absolute_col_1based(col_part)?;
        return Some(Node::Reference(Reference::Cell {
            sheet: None,
            row: current_row,
            col: abs_col,
        }));
    }

    if spec == "@" {
        return None;
    }

    match spec {
        "" | "#Data" => {
            return Some(whole_data_range(info));
        }
        "#Headers" => {
            let header_row_1based = info.header_row + 1;
            let start_col = info.start_col + 1;
            let end_col = info.start_col + info.columns.len() as u32;
            return Some(Node::Reference(Reference::Range {
                sheet: Some(info.sheet_name.clone()),
                start_row: Some(header_row_1based),
                end_row: Some(header_row_1based),
                start_col: Some(start_col),
                end_col: Some(end_col),
            }));
        }
        "#All" => {
            let header_row_1based = info.header_row + 1;
            let end_row_1based = info.data_end_row + 1;
            let start_col = info.start_col + 1;
            let end_col = info.start_col + info.columns.len() as u32;
            return Some(Node::Reference(Reference::Range {
                sheet: Some(info.sheet_name.clone()),
                start_row: Some(header_row_1based),
                end_row: Some(end_row_1based),
                start_col: Some(start_col),
                end_col: Some(end_col),
            }));
        }
        "#Totals" => {
            return None;
        }
        _ => {}
    }

    if let Some((start_name, end_name)) = spec.split_once(':') {
        let start_abs = info.absolute_col_1based(start_name)?;
        let end_abs = info.absolute_col_1based(end_name)?;
        let start_row_1based = info.data_start_row + 1;
        let end_row_1based = info.data_end_row + 1;
        return Some(Node::Reference(Reference::Range {
            sheet: Some(info.sheet_name.clone()),
            start_row: Some(start_row_1based),
            end_row: Some(end_row_1based),
            start_col: Some(start_abs),
            end_col: Some(end_abs),
        }));
    }

    let abs_col = info.absolute_col_1based(spec)?;
    let start_row_1based = info.data_start_row + 1;
    let end_row_1based = info.data_end_row + 1;
    Some(Node::Reference(Reference::Range {
        sheet: Some(info.sheet_name.clone()),
        start_row: Some(start_row_1based),
        end_row: Some(end_row_1based),
        start_col: Some(abs_col),
        end_col: Some(abs_col),
    }))
}

fn parse_this_row_column(spec: &str) -> Option<&str> {
    let rest = spec.strip_prefix("[@],")?;
    let col = rest.strip_prefix('[')?.strip_suffix(']')?;
    if col.is_empty() {
        return None;
    }
    Some(col)
}

#[allow(clippy::cast_possible_truncation)]
fn whole_data_range(info: &TableInfo) -> Node {
    let start_col = info.start_col + 1;
    let end_col = info.start_col + info.columns.len() as u32;
    let start_row_1based = info.data_start_row + 1;
    let end_row_1based = info.data_end_row + 1;
    Node::Reference(Reference::Range {
        sheet: Some(info.sheet_name.clone()),
        start_row: Some(start_row_1based),
        end_row: Some(end_row_1based),
        start_col: Some(start_col),
        end_col: Some(end_col),
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    fn test_tables() -> HashMap<String, TableInfo> {
        let mut m = HashMap::new();
        m.insert(
            "table1".to_string(),
            TableInfo {
                sheet_name: "Sheet1".into(),
                columns: vec![
                    "Region".into(),
                    "Amount".into(),
                    "Price".into(),
                    "Qty".into(),
                    "Status".into(),
                ],
                header_row: 0,
                data_start_row: 1,
                data_end_row: 10,
                start_col: 0,
            },
        );
        m.insert(
            "table2".to_string(),
            TableInfo {
                sheet_name: "Sheet2".into(),
                columns: vec!["Key".into(), "Value".into()],
                header_row: 0,
                data_start_row: 1,
                data_end_row: 5,
                start_col: 0,
            },
        );
        m
    }

    fn resolve(formula: &str) -> Ast {
        let ast = crate::parse(formula).unwrap();
        resolve_table_references(ast, &test_tables(), Some("Sheet1"), 2, 3)
    }

    fn dbg_root(ast: &Ast) -> String {
        format!("{:?}", ast.root)
    }

    #[test]
    fn table_column_resolves_to_bounded_range() {
        let ast = resolve("SUM(Table1[Amount])");
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Range"), "expected Range: {dbg}");
        assert!(!dbg.contains("Table"), "should not contain Table: {dbg}");
        assert!(dbg.contains("start_row: Some(2)"), "bounded start row: {dbg}");
        assert!(dbg.contains("end_row: Some(11)"), "bounded end row: {dbg}");
    }

    #[test]
    fn this_row_column_resolves_to_cell_ref() {
        let ast = resolve("[@Price]*1.1");
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Cell"), "expected Cell: {dbg}");
        assert!(!dbg.contains("Table"), "should not contain Table: {dbg}");
    }

    #[test]
    fn named_table_this_row_resolves() {
        let ast = resolve("Table1[@Region]");
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Cell"), "expected Cell: {dbg}");
    }

    #[test]
    fn case_insensitive_table_and_column() {
        let ast = resolve("SUM(table1[amount])");
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Range"), "expected Range: {dbg}");
        assert!(!dbg.contains("Table"), "case-insensitive failed: {dbg}");
    }

    #[test]
    fn unknown_table_stays_as_table_ref() {
        let ast = resolve("SUM(UnknownTable[Col])");
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Table"), "unknown table should stay: {dbg}");
    }

    #[test]
    fn unknown_column_stays_as_table_ref() {
        let ast = resolve("SUM(Table1[NonexistentCol])");
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Table"), "unknown column should stay: {dbg}");
    }

    #[test]
    fn data_specifier_resolves_to_data_range() {
        let ast = resolve("SUM(Table1[#Data])");
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Range"), "expected Range: {dbg}");
        assert!(!dbg.contains("Table"), "#Data should resolve: {dbg}");
    }

    #[test]
    fn headers_specifier_resolves_to_header_row() {
        let ast = resolve("Table1[#Headers]");
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Range"), "expected Range: {dbg}");
        assert!(dbg.contains("start_row: Some(1)"), "header row 1: {dbg}");
        assert!(dbg.contains("end_row: Some(1)"), "single row: {dbg}");
    }

    #[test]
    fn all_specifier_resolves_from_header_to_last_data() {
        let ast = resolve("Table1[#All]");
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Range"), "expected Range: {dbg}");
        assert!(dbg.contains("start_row: Some(1)"), "includes header: {dbg}");
        assert!(dbg.contains("end_row: Some(11)"), "includes last data row: {dbg}");
    }

    #[test]
    fn totals_specifier_stays_unresolved() {
        let ast = resolve("Table1[#Totals]");
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Table"), "totals should stay unresolved: {dbg}");
    }

    #[test]
    fn column_range_resolves_to_bounded_range() {
        let ast = resolve("Table1[[Region]:[Price]]");
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Range"), "expected Range: {dbg}");
        assert!(dbg.contains("start_col: Some(1)"), "Region col: {dbg}");
        assert!(dbg.contains("end_col: Some(3)"), "Price col: {dbg}");
    }

    #[test]
    fn table_ref_mixed_with_cell_ref() {
        let ast = resolve("[@Price]+B2");
        let dbg = dbg_root(&ast);
        assert!(!dbg.contains("Table"), "table ref should resolve: {dbg}");
    }

    #[test]
    fn parse_this_row_column_valid() {
        assert_eq!(parse_this_row_column("[@],[Price]"), Some("Price"));
        assert_eq!(parse_this_row_column("[@],[Amount]"), Some("Amount"));
    }

    #[test]
    fn parse_this_row_column_invalid() {
        assert_eq!(parse_this_row_column("Price"), None);
        assert_eq!(parse_this_row_column("#Data"), None);
        assert_eq!(parse_this_row_column("[@],[]"), None);
    }
}
