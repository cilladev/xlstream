//! Table metadata extracted from xlsx workbooks.

/// Metadata for a single Excel table, extracted from calamine.
///
/// Contains the table name, parent sheet, column names, and data
/// dimensions (0-based row/col coordinates, headers excluded).
///
/// # Examples
///
/// ```
/// use xlstream_io::TableMeta;
/// let meta = TableMeta {
///     name: "Table1".into(),
///     sheet_name: "Sheet1".into(),
///     columns: vec!["Region".into(), "Amount".into()],
///     header_row: 0,
///     data_start_row: 1,
///     data_end_row: 10,
///     start_col: 0,
/// };
/// assert_eq!(meta.column_index("Amount"), Some(1));
/// ```
#[derive(Debug, Clone)]
pub struct TableMeta {
    /// Table display name (e.g. `"Table1"`).
    pub name: String,
    /// Parent sheet name.
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

impl TableMeta {
    /// Find a column's 0-based offset within this table (case-insensitive).
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_io::TableMeta;
    /// let meta = TableMeta {
    ///     name: "T".into(),
    ///     sheet_name: "S".into(),
    ///     columns: vec!["Price".into(), "Qty".into()],
    ///     header_row: 0,
    ///     data_start_row: 1,
    ///     data_end_row: 5,
    ///     start_col: 0,
    /// };
    /// assert_eq!(meta.column_index("price"), Some(0));
    /// assert_eq!(meta.column_index("Qty"), Some(1));
    /// assert_eq!(meta.column_index("Missing"), None);
    /// ```
    #[must_use]
    pub fn column_index(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|c| c.eq_ignore_ascii_case(name))
    }

    /// Absolute 1-based column number for a named column.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_io::TableMeta;
    /// let meta = TableMeta {
    ///     name: "T".into(),
    ///     sheet_name: "S".into(),
    ///     columns: vec!["A".into(), "B".into()],
    ///     header_row: 0,
    ///     data_start_row: 1,
    ///     data_end_row: 5,
    ///     start_col: 2,
    /// };
    /// assert_eq!(meta.absolute_col("A"), Some(3));
    /// assert_eq!(meta.absolute_col("B"), Some(4));
    /// ```
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn absolute_col(&self, column_name: &str) -> Option<u32> {
        self.column_index(column_name).map(|i| self.start_col + i as u32 + 1)
    }
}
