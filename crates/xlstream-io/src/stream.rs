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
/// yields `(row_index, Vec<Value>)` pairs. Missing cells within a row
/// are padded with [`Value::Empty`]; rows with no data are skipped
/// (callers use the row index to detect gaps).
///
/// Each call opens a fresh read from the start of the sheet.
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
    initial_capacity: usize,
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

    /// Construct a stream backed by a cell source. `capacity_hint` is the
    /// initial buffer size (typically from sheet dimensions); the buffer
    /// grows on demand if cells exceed this.
    pub(crate) fn new(source: Box<dyn CellSource + 'a>, capacity_hint: usize) -> Self {
        Self {
            inner: Some(CellStreamInner {
                source,
                initial_capacity: capacity_hint,
                buffer: Vec::with_capacity(capacity_hint),
                pending_cell: None,
                current_row: None,
                finished: false,
            }),
        }
    }

    /// Yield the next row as `(row_index, values)` where `values` is a
    /// dense `Vec<Value>` with missing cells padded as [`Value::Empty`].
    /// Returns `Ok(None)` at end of sheet. Rows with no data are skipped;
    /// use the row index to detect gaps.
    ///
    /// The buffer grows on demand if cells exist beyond the declared
    /// sheet dimensions.
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
    pub fn next_row(&mut self) -> Result<Option<(u32, Vec<Value>)>, XlStreamError> {
        let Some(inner) = self.inner.as_mut() else {
            return Ok(None);
        };
        inner.next_row()
    }

    /// Advance the stream past all rows before `target_row`, discarding
    /// their data. The next call to [`next_row`] returns `target_row` (or
    /// the first row >= `target_row` if `target_row` has no data).
    ///
    /// Used by parallel workers to seek each reader to its row-range start.
    /// Cost is O(skipped cells) — acceptable for v0.1; v0.2 adds row-offset
    /// indexing for O(1) seek.
    ///
    /// # Errors
    ///
    /// Returns [`XlStreamError::Xlsx`] on calamine read failures.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_io::CellStream;
    /// let mut s = CellStream::empty();
    /// s.seek_to_row(10).unwrap(); // no-op on empty stream
    /// assert_eq!(s.next_row().unwrap(), None);
    /// ```
    pub fn seek_to_row(&mut self, target_row: u32) -> Result<(), XlStreamError> {
        let Some(inner) = self.inner.as_mut() else {
            return Ok(());
        };
        inner.seek_to_row(target_row)
    }
}

impl CellStreamInner<'_> {
    fn next_row(&mut self) -> Result<Option<(u32, Vec<Value>)>, XlStreamError> {
        if self.finished {
            return Ok(None);
        }

        self.buffer.clear();
        self.buffer.resize(self.initial_capacity, Value::Empty);
        self.current_row = None;

        if let Some((row, col, val)) = self.pending_cell.take() {
            self.current_row = Some(row);
            self.place_cell(col, val);
        }

        loop {
            if let Some((row, col, value)) = self.source.next_cell()? {
                if self.current_row.is_none() {
                    self.current_row = Some(row);
                }

                let Some(cr) = self.current_row else {
                    return Err(XlStreamError::Internal(
                        "current_row unexpectedly None after set".into(),
                    ));
                };

                if row != cr {
                    self.pending_cell = Some((row, col, value));
                    let row_data = std::mem::take(&mut self.buffer);
                    return Ok(Some((cr, row_data)));
                }

                self.place_cell(col, value);
            } else {
                self.finished = true;
                return match self.current_row {
                    Some(cr) => Ok(Some((cr, std::mem::take(&mut self.buffer)))),
                    None => Ok(None),
                };
            }
        }
    }

    fn seek_to_row(&mut self, target_row: u32) -> Result<(), XlStreamError> {
        loop {
            if let Some((row, col, val)) = self.source.next_cell()? {
                if row >= target_row {
                    self.pending_cell = Some((row, col, val));
                    return Ok(());
                }
            } else {
                self.finished = true;
                return Ok(());
            }
        }
    }

    fn place_cell(&mut self, col: u32, value: Value) {
        let idx = col as usize;
        if idx >= self.buffer.len() {
            self.buffer.resize(idx + 1, Value::Empty);
        }
        self.buffer[idx] = value;
    }
}

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

    #[test]
    fn seek_to_row_skips_rows_before_target() {
        struct FakeSource {
            cells: Vec<(u32, u32, Value)>,
            pos: usize,
        }
        impl CellSource for FakeSource {
            fn next_cell(&mut self) -> Result<Option<(u32, u32, Value)>, XlStreamError> {
                if self.pos < self.cells.len() {
                    let c = self.cells[self.pos].clone();
                    self.pos += 1;
                    Ok(Some(c))
                } else {
                    Ok(None)
                }
            }
        }

        let source = FakeSource {
            cells: vec![
                (0, 0, Value::Number(1.0)),
                (1, 0, Value::Number(2.0)),
                (2, 0, Value::Number(3.0)),
                (3, 0, Value::Number(4.0)),
            ],
            pos: 0,
        };
        let mut stream = CellStream::new(Box::new(source), 1);
        stream.seek_to_row(2).unwrap();
        let row = stream.next_row().unwrap().unwrap();
        assert_eq!(row.0, 2);
        assert_eq!(row.1[0], Value::Number(3.0));
    }

    #[test]
    fn seek_to_row_zero_is_noop() {
        struct FakeSource {
            cells: Vec<(u32, u32, Value)>,
            pos: usize,
        }
        impl CellSource for FakeSource {
            fn next_cell(&mut self) -> Result<Option<(u32, u32, Value)>, XlStreamError> {
                if self.pos < self.cells.len() {
                    let c = self.cells[self.pos].clone();
                    self.pos += 1;
                    Ok(Some(c))
                } else {
                    Ok(None)
                }
            }
        }

        let source = FakeSource {
            cells: vec![(0, 0, Value::Number(1.0)), (1, 0, Value::Number(2.0))],
            pos: 0,
        };
        let mut stream = CellStream::new(Box::new(source), 1);
        stream.seek_to_row(0).unwrap();
        let row = stream.next_row().unwrap().unwrap();
        assert_eq!(row.0, 0);
    }

    #[test]
    fn seek_past_end_returns_none() {
        struct FakeSource {
            cells: Vec<(u32, u32, Value)>,
            pos: usize,
        }
        impl CellSource for FakeSource {
            fn next_cell(&mut self) -> Result<Option<(u32, u32, Value)>, XlStreamError> {
                if self.pos < self.cells.len() {
                    let c = self.cells[self.pos].clone();
                    self.pos += 1;
                    Ok(Some(c))
                } else {
                    Ok(None)
                }
            }
        }

        let source = FakeSource { cells: vec![(0, 0, Value::Number(1.0))], pos: 0 };
        let mut stream = CellStream::new(Box::new(source), 1);
        stream.seek_to_row(999).unwrap();
        assert!(stream.next_row().unwrap().is_none());
    }
}
