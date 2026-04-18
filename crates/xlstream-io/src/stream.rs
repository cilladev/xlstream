//! The [`CellStream`] — row-oriented iterator over an xlsx sheet backed
//! by calamine's `XlsxCellReader`.

use xlstream_core::{Value, XlStreamError};

/// Internal trait for abstracting over calamine's cell reader (whose
/// concrete type is not re-exported). Implemented in `reader.rs` via a
/// wrapper that owns the `XlsxCellReader`.
pub(crate) trait CellSource {
    /// Return the next cell as `(row, col, value)`, or `None` at EOF.
    fn next_cell(&mut self) -> Result<Option<(u32, u32, Value)>, XlStreamError>;
}

/// Row-oriented stream of cells. Wraps a calamine `XlsxCellReader` and
/// yields one dense `Vec<Value>` per row. Missing cells are padded with
/// [`Value::Empty`] so every row has exactly `max_col` elements.
///
/// # Examples
///
/// ```
/// use xlstream_io::CellStream;
///
/// let mut s = CellStream::empty();
/// assert_eq!(s.next_row().unwrap(), None);
/// ```
pub struct CellStream<'a> {
    inner: Option<CellStreamInner<'a>>,
}

/// Actual streaming state — separated so `CellStream::empty()` can exist
/// without a source.
struct CellStreamInner<'a> {
    source: Box<dyn CellSource + 'a>,
    max_col: usize,
    buffer: Vec<Value>,
    pending_cell: Option<(u32, u32, Value)>,
    current_row: Option<u32>,
    finished: bool,
}

impl<'a> CellStream<'a> {
    /// Construct an empty stream that immediately returns `Ok(None)`.
    /// Useful as a placeholder or in tests.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_io::CellStream;
    /// let mut s = CellStream::empty();
    /// assert_eq!(s.next_row().unwrap(), None);
    /// ```
    #[must_use]
    pub fn empty() -> Self {
        Self { inner: None }
    }

    /// Construct a stream backed by a cell source.
    pub(crate) fn new(source: Box<dyn CellSource + 'a>, max_col: usize) -> Self {
        Self {
            inner: Some(CellStreamInner {
                source,
                max_col,
                buffer: Vec::with_capacity(max_col),
                pending_cell: None,
                current_row: None,
                finished: false,
            }),
        }
    }

    /// Yield the next row as a dense `Vec<Value>` (length == `max_col`),
    /// or `Ok(None)` at end of sheet. Missing cells are [`Value::Empty`].
    ///
    /// Uses `std::mem::take` to hand off the buffer without cloning.
    ///
    /// # Errors
    ///
    /// Returns [`XlStreamError::Xlsx`] on calamine I/O or parse failures.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_io::CellStream;
    /// let mut s = CellStream::empty();
    /// assert_eq!(s.next_row().unwrap(), None);
    /// ```
    pub fn next_row(&mut self) -> Result<Option<Vec<Value>>, XlStreamError> {
        let Some(inner) = self.inner.as_mut() else {
            return Ok(None);
        };
        inner.next_row()
    }
}

impl CellStreamInner<'_> {
    fn next_row(&mut self) -> Result<Option<Vec<Value>>, XlStreamError> {
        if self.finished {
            return Ok(None);
        }

        // Prepare buffer for a new row.
        self.buffer.clear();
        self.buffer.resize(self.max_col, Value::Empty);
        self.current_row = None;

        // If we have a pending cell saved from the previous call, place it.
        if let Some((row, col, val)) = self.pending_cell.take() {
            self.current_row = Some(row);
            if (col as usize) < self.max_col {
                self.buffer[col as usize] = val;
            }
        }

        loop {
            if let Some((row, col, value)) = self.source.next_cell()? {
                if self.current_row.is_none() {
                    self.current_row = Some(row);
                }

                if row != self.current_row.unwrap_or(row) {
                    // This cell belongs to the next row. Save it and
                    // return the current buffer.
                    self.pending_cell = Some((row, col, value));
                    return Ok(Some(std::mem::take(&mut self.buffer)));
                }

                if (col as usize) < self.max_col {
                    self.buffer[col as usize] = value;
                }
            } else {
                self.finished = true;
                return if self.current_row.is_some() {
                    Ok(Some(std::mem::take(&mut self.buffer)))
                } else {
                    Ok(None)
                };
            }
        }
    }
}

// Implement Debug manually since the inner source is a trait object.
impl std::fmt::Debug for CellStream<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CellStream").field("has_inner", &self.inner.is_some()).finish()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn empty_stream_returns_none() {
        let mut s = CellStream::empty();
        assert_eq!(s.next_row().unwrap(), None);
    }

    #[test]
    fn empty_stream_debug_output() {
        let s = CellStream::empty();
        let dbg = format!("{s:?}");
        assert!(dbg.contains("CellStream"), "expected CellStream in debug: {dbg}");
    }
}
