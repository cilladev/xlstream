/// Convert 1-based column and row numbers to A1 notation (e.g. col=3, row=2 → "C2").
///
/// # Examples
///
/// ```
/// use xlstream_core::col_row_to_a1;
/// assert_eq!(col_row_to_a1(1, 1), "A1");
/// assert_eq!(col_row_to_a1(3, 5), "C5");
/// assert_eq!(col_row_to_a1(27, 1), "AA1");
/// ```
#[must_use]
pub fn col_row_to_a1(col: u32, row: u32) -> String {
    let mut letters = String::new();
    let mut c = col;
    while c > 0 {
        c -= 1;
        let ch = char::from(b'A' + u8::try_from(c % 26).unwrap_or(0));
        letters.insert(0, ch);
        c /= 26;
    }
    format!("{letters}{row}")
}

/// Parse an A1-notation cell reference into 0-based `(row, col)`.
///
/// Returns `None` if the reference is malformed (no letters, no digits,
/// row is zero, or overflow).
///
/// # Examples
///
/// ```
/// use xlstream_core::a1_to_col_row;
/// assert_eq!(a1_to_col_row("A1"), Some((0, 0)));
/// assert_eq!(a1_to_col_row("C5"), Some((4, 2)));
/// assert_eq!(a1_to_col_row("AA1"), Some((0, 26)));
/// ```
#[must_use]
pub fn a1_to_col_row(cell_ref: &str) -> Option<(u32, u16)> {
    let bytes = cell_ref.as_bytes();
    let mut col: u32 = 0;
    let mut i = 0;

    while i < bytes.len() && bytes[i].is_ascii_alphabetic() {
        col = col
            .checked_mul(26)?
            .checked_add(u32::from(bytes[i].to_ascii_uppercase() - b'A') + 1)?;
        i += 1;
    }
    if i == 0 || col == 0 {
        return None;
    }

    let row_str = &cell_ref[i..];
    if row_str.is_empty() {
        return None;
    }
    let row: u32 = row_str.parse().ok()?;
    if row == 0 {
        return None;
    }

    Some((row - 1, u16::try_from(col - 1).ok()?))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
    use super::*;

    #[test]
    fn a1_to_col_row_simple() {
        assert_eq!(a1_to_col_row("A1"), Some((0, 0)));
        assert_eq!(a1_to_col_row("B2"), Some((1, 1)));
        assert_eq!(a1_to_col_row("Z1"), Some((0, 25)));
    }

    #[test]
    fn a1_to_col_row_multi_letter() {
        assert_eq!(a1_to_col_row("AA1"), Some((0, 26)));
        assert_eq!(a1_to_col_row("AZ1"), Some((0, 51)));
        assert_eq!(a1_to_col_row("BA1"), Some((0, 52)));
    }

    #[test]
    fn a1_to_col_row_large_row() {
        assert_eq!(a1_to_col_row("A1048576"), Some((1_048_575, 0)));
    }

    #[test]
    fn a1_to_col_row_invalid() {
        assert_eq!(a1_to_col_row(""), None);
        assert_eq!(a1_to_col_row("1A"), None);
        assert_eq!(a1_to_col_row("A0"), None);
        assert_eq!(a1_to_col_row("A"), None);
    }

    #[test]
    fn a1_to_col_row_lowercase() {
        assert_eq!(a1_to_col_row("a1"), Some((0, 0)));
        assert_eq!(a1_to_col_row("ab10"), Some((9, 27)));
    }

    #[test]
    #[allow(clippy::cast_possible_truncation)]
    fn round_trip_col_row() {
        for col in [1u32, 2, 26, 27, 52, 702] {
            for row in [1u32, 2, 100, 1_048_576] {
                let a1 = col_row_to_a1(col, row);
                let (r, c) = a1_to_col_row(&a1).unwrap();
                assert_eq!((r, c), (row - 1, (col - 1) as u16), "round-trip failed for {a1}");
            }
        }
    }
}
