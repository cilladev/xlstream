//! The [`ExcelDate`] newtype — an Excel date serial number.

/// Newtype wrapping an Excel date serial (days since the 1900 epoch, with
/// the Lotus 1-2-3 leap-year bug preserved at serial 60).
///
/// In Phase 1 this is a bare field-wrapper; calendar conversion lands in
/// a later phase (see `docs/phases/phase-09-strings-dates-math.md`).
///
/// # Examples
///
/// ```
/// use xlstream_core::ExcelDate;
/// let d = ExcelDate { serial: 44_927.0 };
/// assert_eq!(d.serial, 44_927.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExcelDate {
    /// Days since the Excel 1900 epoch (fractional part = time of day).
    pub serial: f64,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use super::*;

    #[test]
    fn construction_and_field_access() {
        let d = ExcelDate { serial: 44_927.0 };
        assert_eq!(d.serial, 44_927.0);
    }

    #[test]
    fn equal_serials_compare_equal() {
        assert_eq!(ExcelDate { serial: 1.0 }, ExcelDate { serial: 1.0 });
    }
}
