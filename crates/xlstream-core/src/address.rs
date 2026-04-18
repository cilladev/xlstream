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
