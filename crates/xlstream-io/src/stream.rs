//! The [`CellStream`] stub — row-oriented iterator over an xlsx sheet. Real
//! logic lands in Phase 3.

use xlstream_core::{Value, XlStreamError};

/// Row-oriented stream of cells. Phase 1 ships a stub `empty()`
/// constructor and a `next_row` that always errors; Phase 3 wires it to
/// a calamine cell reader.
///
/// # Examples
///
/// ```
/// use xlstream_io::CellStream;
/// let mut s = CellStream::empty();
/// assert!(s.next_row().is_err());
/// ```
#[derive(Debug)]
pub struct CellStream {
    _private: (),
}

impl CellStream {
    /// Construct an empty stream. Useful only as a placeholder until Phase 3
    /// gives readers a real constructor.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_io::CellStream;
    /// let _s = CellStream::empty();
    /// ```
    #[must_use]
    pub fn empty() -> Self {
        Self { _private: () }
    }

    /// Yield the next row, or `Ok(None)` at end of sheet.
    ///
    /// # Errors
    ///
    /// Phase 1: always [`XlStreamError::Internal`]. Phase 3 onward: I/O or
    /// parse failures encountered while advancing the underlying reader.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_io::CellStream;
    /// let mut s = CellStream::empty();
    /// assert!(s.next_row().is_err());
    /// ```
    #[must_use = "advancing the stream is useless without inspecting the result"]
    pub fn next_row(&mut self) -> Result<Option<Vec<Value>>, XlStreamError> {
        Err(XlStreamError::Internal(
            "unimplemented: CellStream::next_row — lands in Phase 3".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn next_row_returns_unimplemented_internal_error() {
        let mut s = CellStream::empty();
        let err = s.next_row().unwrap_err();
        assert!(
            matches!(err, xlstream_core::XlStreamError::Internal(ref msg) if msg.contains("unimplemented")),
            "expected Internal(unimplemented...), got {err:?}",
        );
    }
}
