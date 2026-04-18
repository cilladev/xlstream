//! The [`Reader`] stub — calamine-backed xlsx reader. Real logic lands in
//! Phase 3.

use std::path::Path;

use xlstream_core::XlStreamError;

/// Streaming xlsx reader. Phase 1 ships a unit-struct stub; Phase 3 wires
/// it to a [`calamine::Xlsx`] cell reader that yields `Vec<Value>` rows.
///
/// [`calamine::Xlsx`]: https://docs.rs/calamine
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use xlstream_io::Reader;
/// let err = Reader::open(Path::new("nope.xlsx")).unwrap_err();
/// assert!(err.to_string().contains("unimplemented"));
/// ```
#[derive(Debug)]
pub struct Reader {
    _private: (),
}

impl Reader {
    /// Open an xlsx file for streaming reads.
    ///
    /// # Errors
    ///
    /// Phase 1: always [`XlStreamError::Internal`]. Phase 3 onward: real
    /// I/O errors surfaced as [`XlStreamError::Io`] /
    /// `XlStreamError::Xlsx` (variant added in Phase 3).
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use xlstream_io::Reader;
    /// assert!(Reader::open(Path::new("x.xlsx")).is_err());
    /// ```
    #[must_use = "opening a reader is useless without inspecting the result"]
    pub fn open(_path: &Path) -> Result<Self, XlStreamError> {
        Err(XlStreamError::Internal("unimplemented: Reader::open — lands in Phase 3".into()))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use std::path::Path;

    use super::*;

    #[test]
    fn open_returns_unimplemented_internal_error() {
        let err = Reader::open(Path::new("doesnt-exist.xlsx")).unwrap_err();
        assert!(
            matches!(err, xlstream_core::XlStreamError::Internal(ref msg) if msg.contains("unimplemented")),
            "expected Internal(unimplemented...), got {err:?}",
        );
    }
}
