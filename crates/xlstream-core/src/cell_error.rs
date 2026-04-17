//! The [`CellError`] enum — Excel's in-cell error values.

/// Excel cell-level errors. These are **values**, not exceptions: a cell can
/// hold `#DIV/0!` just as it can hold a number, and formulas that consume
/// it propagate the error rather than aborting the workbook.
///
/// See [`docs/architecture/errors.md`] for the full propagation rules.
///
/// [`docs/architecture/errors.md`]: https://github.com/cilladev/xlstream/blob/main/docs/architecture/errors.md
///
/// # Examples
///
/// ```
/// use xlstream_core::CellError;
/// let e = CellError::Div0;
/// assert_eq!(e, CellError::Div0);
/// assert_ne!(e, CellError::Na);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellError {
    /// `#DIV/0!` — division by zero.
    Div0,
    /// `#VALUE!` — argument or operand has the wrong type.
    Value,
    /// `#REF!` — a referenced cell is invalid or has been deleted.
    Ref,
    /// `#NAME?` — an unrecognised name (function, defined name, or identifier).
    Name,
    /// `#N/A` — a value is not available (for example a lookup miss).
    Na,
    /// `#NUM!` — a numeric value is out of range or otherwise invalid.
    Num,
    /// `#NULL!` — an intersection operator produced an empty set.
    Null,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn each_variant_is_distinct() {
        assert_ne!(CellError::Div0, CellError::Value);
        assert_ne!(CellError::Value, CellError::Ref);
        assert_ne!(CellError::Ref, CellError::Name);
        assert_ne!(CellError::Name, CellError::Na);
        assert_ne!(CellError::Na, CellError::Num);
        assert_ne!(CellError::Num, CellError::Null);
    }

    #[test]
    fn variants_are_clone_and_copy() {
        let a = CellError::Div0;
        let b = a;
        let _ = a; // Copy, so this still compiles.
        assert_eq!(a, b);
    }
}
