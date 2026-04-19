//! The [`ExcelDate`] newtype — an Excel date serial number with
//! calendar conversion methods.

/// Newtype wrapping an Excel date serial (days since the 1900 epoch, with
/// the Lotus 1-2-3 leap-year bug preserved at serial 60).
///
/// # The 1900 leap-year bug
///
/// Excel (inherited from Lotus 1-2-3) treats 1900 as a leap year. Serial
/// 60 maps to the non-existent date Feb 29, 1900. All serials > 60 are
/// offset by +1 relative to the real Gregorian calendar.
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

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Gregorian leap-year test (1900 is NOT a leap year in reality).
fn is_real_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// Days per month for the real Gregorian calendar.
fn month_days(year: i32) -> [i32; 12] {
    let feb = if is_real_leap_year(year) { 29 } else { 28 };
    [31, feb, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
}

/// Days in a real Gregorian year.
fn year_days(year: i32) -> i32 {
    if is_real_leap_year(year) {
        366
    } else {
        365
    }
}

impl ExcelDate {
    /// Construct from a serial number.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_core::ExcelDate;
    /// let d = ExcelDate::from_serial(1.0);
    /// assert_eq!(d.serial, 1.0);
    /// ```
    #[must_use]
    pub fn from_serial(serial: f64) -> Self {
        Self { serial }
    }

    /// Convert serial to `(year, month, day)`.
    ///
    /// Serial 1 = Jan 1, 1900. Serial 60 = Feb 29, 1900 (the Lotus
    /// 1-2-3 bug). Serial 0 and negative serials return `(1900, 1, 0)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_core::ExcelDate;
    /// assert_eq!(ExcelDate::from_serial(1.0).year_month_day(), (1900, 1, 1));
    /// assert_eq!(ExcelDate::from_serial(60.0).year_month_day(), (1900, 2, 29));
    /// assert_eq!(ExcelDate::from_serial(61.0).year_month_day(), (1900, 3, 1));
    /// ```
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn year_month_day(&self) -> (i32, u32, u32) {
        let serial_int = self.serial.floor() as i64;

        // Invalid / zero serial
        if serial_int <= 0 {
            return (1900, 1, 0);
        }

        // The fake Feb 29, 1900
        if serial_int == 60 {
            return (1900, 2, 29);
        }

        // For serial > 60, subtract 1 to get real day count (undoing
        // the phantom Feb 29).
        let real_days = if serial_int > 60 { serial_int - 1 } else { serial_int };

        // real_days == 1 means Jan 1, 1900. Convert to 0-based day
        // offset from Jan 1, 1900.
        let mut remaining = (real_days - 1) as i32;

        let mut year = 1900i32;
        loop {
            let yd = year_days(year);
            if remaining < yd {
                break;
            }
            remaining -= yd;
            year += 1;
        }

        let md = month_days(year);
        let mut month = 0u32;
        for (i, &days) in md.iter().enumerate() {
            if remaining < days {
                month = i as u32 + 1;
                break;
            }
            remaining -= days;
        }

        let day = remaining as u32 + 1;
        (year, month, day)
    }

    /// Construct from year, month, day with Excel-style rollover.
    ///
    /// Months outside 1..12 roll over into adjacent years. Days outside
    /// the valid range roll over into adjacent months via serial
    /// arithmetic.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_core::ExcelDate;
    /// // Normal date
    /// assert_eq!(ExcelDate::from_ymd(2026, 4, 19).year_month_day(), (2026, 4, 19));
    /// // Month rollover
    /// assert_eq!(ExcelDate::from_ymd(2026, 13, 1).year_month_day(), (2027, 1, 1));
    /// // Day rollover
    /// assert_eq!(ExcelDate::from_ymd(2026, 1, 32).year_month_day(), (2026, 2, 1));
    /// // Negative month rollover
    /// assert_eq!(ExcelDate::from_ymd(2026, 0, 1).year_month_day(), (2025, 12, 1));
    /// ```
    #[must_use]
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Self {
        // Special case: the fake Feb 29, 1900
        if year == 1900 && month == 2 && day == 29 {
            return Self { serial: 60.0 };
        }

        // Normalize month to 1..12 range
        let mut y = year;
        let mut m = month;

        // Handle month overflow/underflow
        if m < 1 {
            let years_back = (1 - m) / 12 + 1;
            y -= years_back;
            m += years_back * 12;
        }
        if m > 12 {
            y += (m - 1) / 12;
            m = (m - 1) % 12 + 1;
        }

        // Compute serial for (y, m, 1) then add (day - 1).
        // serial = 1 + days_from_1900_jan_1_to_(y,m,1)
        let mut total_days: i64 = 0;
        for yr in 1900..y {
            total_days += i64::from(year_days(yr));
        }
        if y < 1900 {
            for yr in y..1900 {
                total_days -= i64::from(year_days(yr));
            }
        }
        let md = month_days(y);
        #[allow(clippy::cast_sign_loss)]
        for &d in md.iter().take((m as usize - 1).min(12)) {
            total_days += i64::from(d);
        }

        // serial for (y, m, 1) is total_days + 1
        let base_serial = total_days + 1;
        let serial = base_serial + i64::from(day - 1);

        // Account for the fake Feb 29, 1900: if serial >= 60, add 1
        let adjusted = if serial >= 60 { serial + 1 } else { serial };

        #[allow(clippy::cast_precision_loss)]
        Self { serial: adjusted as f64 }
    }

    /// Day of week. 0 = Sunday, 1 = Monday, ..., 6 = Saturday.
    ///
    /// Excel serial 1 (Jan 1, 1900) was a Sunday.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_core::ExcelDate;
    /// // Serial 1 = Jan 1, 1900 = Sunday
    /// assert_eq!(ExcelDate::from_serial(1.0).weekday(), 0);
    /// // Serial 7 = Jan 7, 1900 = Saturday
    /// assert_eq!(ExcelDate::from_serial(7.0).weekday(), 6);
    /// ```
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn weekday(&self) -> u32 {
        let serial_int = self.serial.floor() as i64;
        if serial_int <= 0 {
            return 0; // fallback for invalid serials
        }
        ((serial_int - 1) % 7) as u32
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use super::*;

    // -- from_serial --

    #[test]
    fn from_serial_wraps_value() {
        let d = ExcelDate::from_serial(44_927.0);
        assert_eq!(d.serial, 44_927.0);
    }

    // -- year_month_day --

    #[test]
    fn serial_1_is_jan_1_1900() {
        assert_eq!(ExcelDate::from_serial(1.0).year_month_day(), (1900, 1, 1));
    }

    #[test]
    fn serial_59_is_feb_28_1900() {
        assert_eq!(ExcelDate::from_serial(59.0).year_month_day(), (1900, 2, 28));
    }

    #[test]
    fn serial_60_is_fake_feb_29_1900() {
        assert_eq!(ExcelDate::from_serial(60.0).year_month_day(), (1900, 2, 29));
    }

    #[test]
    fn serial_61_is_mar_1_1900() {
        assert_eq!(ExcelDate::from_serial(61.0).year_month_day(), (1900, 3, 1));
    }

    #[test]
    fn serial_36526_is_jan_1_2000() {
        assert_eq!(ExcelDate::from_serial(36526.0).year_month_day(), (2000, 1, 1));
    }

    #[test]
    fn serial_44927_is_jan_1_2023() {
        assert_eq!(ExcelDate::from_serial(44927.0).year_month_day(), (2023, 1, 1));
    }

    #[test]
    fn serial_0_returns_invalid() {
        assert_eq!(ExcelDate::from_serial(0.0).year_month_day(), (1900, 1, 0));
    }

    #[test]
    fn serial_negative_returns_invalid() {
        assert_eq!(ExcelDate::from_serial(-5.0).year_month_day(), (1900, 1, 0));
    }

    #[test]
    fn serial_366_is_dec_31_1900() {
        assert_eq!(ExcelDate::from_serial(366.0).year_month_day(), (1900, 12, 31));
    }

    #[test]
    fn serial_367_is_jan_1_1901() {
        assert_eq!(ExcelDate::from_serial(367.0).year_month_day(), (1901, 1, 1));
    }

    // -- from_ymd --

    #[test]
    fn from_ymd_roundtrip_2026_04_19() {
        let d = ExcelDate::from_ymd(2026, 4, 19);
        assert_eq!(d.year_month_day(), (2026, 4, 19));
    }

    #[test]
    fn from_ymd_month_rollover() {
        let d = ExcelDate::from_ymd(2026, 13, 1);
        assert_eq!(d.year_month_day(), (2027, 1, 1));
    }

    #[test]
    fn from_ymd_day_rollover() {
        let d = ExcelDate::from_ymd(2026, 1, 32);
        assert_eq!(d.year_month_day(), (2026, 2, 1));
    }

    #[test]
    fn from_ymd_negative_month_rollover() {
        let d = ExcelDate::from_ymd(2026, 0, 1);
        assert_eq!(d.year_month_day(), (2025, 12, 1));
    }

    #[test]
    fn from_ymd_fake_feb_29_1900() {
        let d = ExcelDate::from_ymd(1900, 2, 29);
        assert_eq!(d.serial, 60.0);
    }

    #[test]
    fn from_ymd_leap_year_2024_feb_29() {
        let d = ExcelDate::from_ymd(2024, 2, 29);
        assert_eq!(d.year_month_day(), (2024, 2, 29));
    }

    #[test]
    fn from_ymd_jan_1_1900() {
        let d = ExcelDate::from_ymd(1900, 1, 1);
        assert_eq!(d.serial, 1.0);
    }

    #[test]
    fn from_ymd_mar_1_1900() {
        let d = ExcelDate::from_ymd(1900, 3, 1);
        assert_eq!(d.serial, 61.0);
    }

    #[test]
    fn from_ymd_large_month_rollover() {
        // Month 25 = 2 years + 1 month = Feb
        let d = ExcelDate::from_ymd(2026, 25, 1);
        assert_eq!(d.year_month_day(), (2028, 1, 1));
    }

    #[test]
    fn from_ymd_negative_day_rollover() {
        // Day 0 of February = last day of January
        let d = ExcelDate::from_ymd(2026, 2, 0);
        assert_eq!(d.year_month_day(), (2026, 1, 31));
    }

    // -- weekday --

    #[test]
    fn weekday_serial_1_is_sunday() {
        assert_eq!(ExcelDate::from_serial(1.0).weekday(), 0);
    }

    #[test]
    fn weekday_serial_2_is_monday() {
        assert_eq!(ExcelDate::from_serial(2.0).weekday(), 1);
    }

    #[test]
    fn weekday_serial_7_is_saturday() {
        assert_eq!(ExcelDate::from_serial(7.0).weekday(), 6);
    }

    #[test]
    fn weekday_serial_8_is_sunday_again() {
        assert_eq!(ExcelDate::from_serial(8.0).weekday(), 0);
    }

    #[test]
    fn weekday_zero_serial_fallback() {
        assert_eq!(ExcelDate::from_serial(0.0).weekday(), 0);
    }

    // -- is_real_leap_year --

    #[test]
    fn leap_year_1900_is_not_real_leap() {
        assert!(!is_real_leap_year(1900));
    }

    #[test]
    fn leap_year_2000_is_leap() {
        assert!(is_real_leap_year(2000));
    }

    #[test]
    fn leap_year_2024_is_leap() {
        assert!(is_real_leap_year(2024));
    }

    #[test]
    fn leap_year_2100_is_not_leap() {
        assert!(!is_real_leap_year(2100));
    }

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
