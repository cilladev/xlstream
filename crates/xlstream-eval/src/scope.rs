//! [`RowScope`] — current row's cell values, accessible by 1-based column
//! index.

use xlstream_core::{CellError, Value};

/// Current row's cell values, accessible by 1-based column index.
///
/// The evaluator constructs one `RowScope` per streamed row and passes it
/// into [`Interpreter::eval`](crate::Interpreter::eval) for each formula
/// cell. All cell references resolved during the row pass go through
/// [`get`](RowScope::get).
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::RowScope;
/// let row = vec![Value::Number(10.0), Value::Text("hi".into())];
/// let scope = RowScope::new(&row, 2);
/// assert_eq!(scope.get(1), Value::Number(10.0));
/// assert_eq!(scope.get(3), Value::Error(xlstream_core::CellError::Ref));
/// ```
pub struct RowScope<'row> {
    values: &'row [Value],
    row_idx: u32,
}

impl<'row> RowScope<'row> {
    /// Build a scope over the given row values at 0-based row index
    /// `row_idx`.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_core::Value;
    /// use xlstream_eval::RowScope;
    /// let row = vec![Value::Number(1.0)];
    /// let scope = RowScope::new(&row, 0);
    /// assert_eq!(scope.row_idx(), 0);
    /// ```
    #[must_use]
    pub fn new(values: &'row [Value], row_idx: u32) -> Self {
        Self { values, row_idx }
    }

    /// Get cell value by 1-based column index. Out of bounds returns
    /// `#REF!`.
    ///
    /// Note: clones the `Value`. `Number`/`Bool`/`Empty`/`Error` are cheap
    /// (Copy-like). `Text(Box<str>)` clones the heap string — accepted
    /// tech debt for Phase 10.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_core::{CellError, Value};
    /// use xlstream_eval::RowScope;
    /// let row = vec![Value::Bool(true)];
    /// let scope = RowScope::new(&row, 0);
    /// assert_eq!(scope.get(1), Value::Bool(true));
    /// assert_eq!(scope.get(2), Value::Error(CellError::Ref));
    /// ```
    #[must_use]
    pub fn get(&self, col: u32) -> Value {
        if col == 0 {
            return Value::Error(CellError::Ref);
        }
        let idx = (col as usize) - 1;
        self.values.get(idx).cloned().unwrap_or(Value::Error(CellError::Ref))
    }

    /// 0-based row index.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_eval::RowScope;
    /// let scope = RowScope::new(&[], 7);
    /// assert_eq!(scope.row_idx(), 7);
    /// ```
    #[must_use]
    pub fn row_idx(&self) -> u32 {
        self.row_idx
    }

    /// Borrow the underlying values slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_core::Value;
    /// use xlstream_eval::RowScope;
    /// let row = vec![Value::Empty];
    /// let scope = RowScope::new(&row, 0);
    /// assert_eq!(scope.values().len(), 1);
    /// ```
    #[must_use]
    pub fn values(&self) -> &[Value] {
        self.values
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use xlstream_core::{CellError, Value};

    use super::*;

    #[test]
    fn get_returns_value_at_index() {
        let row = vec![Value::Number(1.0), Value::Text("two".into()), Value::Bool(false)];
        let scope = RowScope::new(&row, 0);
        assert_eq!(scope.get(1), Value::Number(1.0));
        assert_eq!(scope.get(2), Value::Text("two".into()));
        assert_eq!(scope.get(3), Value::Bool(false));
    }

    #[test]
    fn get_out_of_bounds_returns_ref_error() {
        let row = vec![Value::Number(1.0)];
        let scope = RowScope::new(&row, 0);
        assert_eq!(scope.get(2), Value::Error(CellError::Ref));
        assert_eq!(scope.get(100), Value::Error(CellError::Ref));
    }

    #[test]
    fn get_col_zero_returns_ref_error() {
        let row = vec![Value::Number(1.0)];
        let scope = RowScope::new(&row, 0);
        // col 0 is invalid — columns are 1-based.
        assert_eq!(scope.get(0), Value::Error(CellError::Ref));
    }
}
