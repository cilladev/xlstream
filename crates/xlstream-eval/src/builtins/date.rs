//! Date builtin functions (Phase 9, Chunk 1).
//!
//! Stateful functions (`TODAY`, `NOW`) need the interpreter for prelude
//! access. Pure functions (`DATE`, `YEAR`, etc.) take `&[Value]`.

use xlstream_core::{coerce, CellError, ExcelDate, Value};
use xlstream_parse::NodeRef;

use crate::interp::Interpreter;
use crate::scope::RowScope;

// ---------------------------------------------------------------------------
// Shared helper
// ---------------------------------------------------------------------------

/// Extract an Excel date serial from a value.
///
/// `Value::Date` → serial, `Value::Error` → propagated, otherwise
/// coerce to number.
fn date_serial(v: &Value) -> Result<f64, Value> {
    match v {
        Value::Date(d) => Ok(d.serial),
        Value::Error(e) => Err(Value::Error(*e)),
        other => coerce::to_number(other).map_err(Value::Error),
    }
}

// ---------------------------------------------------------------------------
// Stateful builtins (need prelude)
// ---------------------------------------------------------------------------

/// `TODAY()` — current date serial from prelude volatile data.
pub(crate) fn builtin_today(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    _scope: &RowScope<'_>,
) -> Value {
    if !args.is_empty() {
        return Value::Error(CellError::Value);
    }
    Value::Date(interp.prelude().volatile_today())
}

/// `NOW()` — current date+time serial from prelude volatile data.
pub(crate) fn builtin_now(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    _scope: &RowScope<'_>,
) -> Value {
    if !args.is_empty() {
        return Value::Error(CellError::Value);
    }
    Value::Date(interp.prelude().volatile_now())
}

// ---------------------------------------------------------------------------
// Pure builtins
// ---------------------------------------------------------------------------

/// `DATE(year, month, day)` — construct a date with Excel rollover.
pub(crate) fn builtin_date(args: &[Value]) -> Value {
    if args.len() != 3 {
        return Value::Error(CellError::Value);
    }
    let y = match coerce::to_number(&args[0]) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };
    let m = match coerce::to_number(&args[1]) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };
    let d = match coerce::to_number(&args[2]) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };

    #[allow(clippy::cast_possible_truncation)]
    let date = ExcelDate::from_ymd(y as i32, m as i32, d as i32);
    Value::Date(date)
}

/// `YEAR(date)` — extract the year component.
pub(crate) fn builtin_year(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let serial = match date_serial(&args[0]) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let (y, _, _) = ExcelDate::from_serial(serial).year_month_day();
    Value::Number(f64::from(y))
}

/// `MONTH(date)` — extract the month component (1..12).
pub(crate) fn builtin_month(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let serial = match date_serial(&args[0]) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let (_, m, _) = ExcelDate::from_serial(serial).year_month_day();
    Value::Number(f64::from(m))
}

/// `DAY(date)` — extract the day component.
pub(crate) fn builtin_day(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value::Error(CellError::Value);
    }
    let serial = match date_serial(&args[0]) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let (_, _, d) = ExcelDate::from_serial(serial).year_month_day();
    Value::Number(f64::from(d))
}

/// `WEEKDAY(date, return_type?)` — day of week.
///
/// Type 1 (default): Sun=1..Sat=7.
/// Type 2: Mon=1..Sun=7.
/// Type 3: Mon=0..Sun=6.
/// Others: `#NUM!`.
pub(crate) fn builtin_weekday(args: &[Value]) -> Value {
    if args.is_empty() || args.len() > 2 {
        return Value::Error(CellError::Value);
    }
    let serial = match date_serial(&args[0]) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let return_type = if args.len() == 2 {
        match coerce::to_number(&args[1]) {
            Ok(n) => {
                #[allow(clippy::cast_possible_truncation)]
                let t = n as i32;
                t
            }
            Err(e) => return Value::Error(e),
        }
    } else {
        1
    };

    // Raw weekday: 0=Sun..6=Sat
    let raw = ExcelDate::from_serial(serial).weekday();

    let result = match return_type {
        1 => raw + 1, // Sun=1..Sat=7
        2 => {
            // Mon=1..Sun=7
            if raw == 0 {
                7
            } else {
                raw
            }
        }
        3 => {
            // Mon=0..Sun=6
            if raw == 0 {
                6
            } else {
                raw - 1
            }
        }
        _ => return Value::Error(CellError::Num),
    };

    Value::Number(f64::from(result))
}

/// `EDATE(start_date, months)` — date N months from start, same day,
/// clamped to month end.
pub(crate) fn builtin_edate(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let serial = match date_serial(&args[0]) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let months = match coerce::to_number(&args[1]) {
        Ok(n) => {
            #[allow(clippy::cast_possible_truncation)]
            let m = n as i32;
            m
        }
        Err(e) => return Value::Error(e),
    };

    let (y, m, d) = ExcelDate::from_serial(serial).year_month_day();
    #[allow(clippy::cast_possible_wrap)]
    let total_months = y * 12 + (m as i32 - 1) + months;
    let new_y = total_months.div_euclid(12);
    let new_m = total_months.rem_euclid(12) + 1;

    // Clamp day to last day of target month
    let target_md = month_days_for_excel(new_y, new_m);
    #[allow(clippy::cast_possible_wrap)]
    let clamped_d = (d as i32).min(target_md);

    Value::Date(ExcelDate::from_ymd(new_y, new_m, clamped_d))
}

/// `EOMONTH(start_date, months)` — last day of month N months from start.
pub(crate) fn builtin_eomonth(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let serial = match date_serial(&args[0]) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let months = match coerce::to_number(&args[1]) {
        Ok(n) => {
            #[allow(clippy::cast_possible_truncation)]
            let m = n as i32;
            m
        }
        Err(e) => return Value::Error(e),
    };

    let (y, m, _) = ExcelDate::from_serial(serial).year_month_day();
    #[allow(clippy::cast_possible_wrap)]
    let total_months = y * 12 + (m as i32 - 1) + months;
    let new_y = total_months.div_euclid(12);
    let new_m = total_months.rem_euclid(12) + 1;

    let last_day = month_days_for_excel(new_y, new_m);
    Value::Date(ExcelDate::from_ymd(new_y, new_m, last_day))
}

/// `DATEDIF(start, end, unit)` — difference between two dates.
///
/// Units: "y" (years), "m" (months), "d" (days), "ym" (months ignoring
/// years), "yd" (days ignoring years), "md" (days ignoring months and
/// years). Start > end returns `#NUM!`.
pub(crate) fn builtin_datedif(args: &[Value]) -> Value {
    if args.len() != 3 {
        return Value::Error(CellError::Value);
    }
    let start_serial = match date_serial(&args[0]) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let end_serial = match date_serial(&args[1]) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let unit = match super::string::text_arg(args, 2) {
        Ok(s) => s.to_ascii_lowercase(),
        Err(e) => return e,
    };

    #[allow(clippy::cast_possible_truncation)]
    let start_int = start_serial.floor() as i64;
    #[allow(clippy::cast_possible_truncation)]
    let end_int = end_serial.floor() as i64;

    if start_int > end_int {
        return Value::Error(CellError::Num);
    }

    let (sy, sm, sd) = ExcelDate::from_serial(start_serial).year_month_day();
    let (ey, em, ed) = ExcelDate::from_serial(end_serial).year_month_day();

    match unit.as_str() {
        "d" =>
        {
            #[allow(clippy::cast_precision_loss)]
            Value::Number((end_int - start_int) as f64)
        }
        "m" => {
            #[allow(clippy::cast_possible_wrap)]
            let mut months = (ey - sy) * 12 + (em as i32 - sm as i32);
            if ed < sd {
                months -= 1;
            }
            Value::Number(f64::from(months.max(0)))
        }
        "y" => {
            let mut years = ey - sy;
            if em < sm || (em == sm && ed < sd) {
                years -= 1;
            }
            Value::Number(f64::from(years.max(0)))
        }
        "ym" => {
            // Months ignoring years
            #[allow(clippy::cast_possible_wrap)]
            let mut months = em as i32 - sm as i32;
            if ed < sd {
                months -= 1;
            }
            if months < 0 {
                months += 12;
            }
            Value::Number(f64::from(months))
        }
        "yd" => {
            // Days ignoring years — compute from adjusted start to end
            // within same year context
            #[allow(clippy::cast_possible_wrap)]
            let start_month = sm as i32;
            #[allow(clippy::cast_possible_wrap)]
            let start_day = sd as i32;
            let start_in_end_year = ExcelDate::from_ymd(ey, start_month, start_day);
            #[allow(clippy::cast_possible_truncation)]
            let mut diff = end_int - start_in_end_year.serial.floor() as i64;
            if diff < 0 {
                // Wrapped around a year boundary
                let start_in_prev_year = ExcelDate::from_ymd(ey - 1, start_month, start_day);
                #[allow(clippy::cast_possible_truncation)]
                {
                    diff = end_int - start_in_prev_year.serial.floor() as i64;
                }
            }
            #[allow(clippy::cast_precision_loss)]
            Value::Number(diff.max(0) as f64)
        }
        "md" => {
            // Days ignoring months and years
            #[allow(clippy::cast_possible_wrap)]
            let mut days = ed as i32 - sd as i32;
            if days < 0 {
                // Get days in previous month
                let prev_m = if em == 1 { 12 } else { em - 1 };
                let prev_y = if em == 1 { ey - 1 } else { ey };
                #[allow(clippy::cast_possible_wrap)]
                {
                    days += month_days_for_excel(prev_y, prev_m as i32);
                }
            }
            Value::Number(f64::from(days.max(0)))
        }
        _ => Value::Error(CellError::Num),
    }
}

/// `NETWORKDAYS(start, end, holidays?)` — count working days between
/// start and end (inclusive), skipping weekends.
///
/// v0.1: holidays argument is accepted but ignored with a warning.
pub(crate) fn builtin_networkdays(args: &[Value]) -> Value {
    if args.len() < 2 || args.len() > 3 {
        return Value::Error(CellError::Value);
    }
    let start_serial = match date_serial(&args[0]) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let end_serial = match date_serial(&args[1]) {
        Ok(s) => s,
        Err(e) => return e,
    };
    if args.len() == 3 {
        tracing::warn!("NETWORKDAYS: holidays argument ignored in v0.1");
    }

    #[allow(clippy::cast_possible_truncation)]
    let start = start_serial.floor() as i64;
    #[allow(clippy::cast_possible_truncation)]
    let end = end_serial.floor() as i64;

    let (lo, hi, sign) = if start <= end { (start, end, 1i64) } else { (end, start, -1i64) };

    let mut count = 0i64;
    for day in lo..=hi {
        #[allow(clippy::cast_precision_loss)]
        let wd = ExcelDate::from_serial(day as f64).weekday();
        // 0=Sun, 6=Sat — skip weekends
        if wd != 0 && wd != 6 {
            count += 1;
        }
    }

    #[allow(clippy::cast_precision_loss)]
    Value::Number((count * sign) as f64)
}

/// `WORKDAY(start, days, holidays?)` — date after N working days.
///
/// v0.1: holidays argument is accepted but ignored with a warning.
pub(crate) fn builtin_workday(args: &[Value]) -> Value {
    if args.len() < 2 || args.len() > 3 {
        return Value::Error(CellError::Value);
    }
    let start_serial = match date_serial(&args[0]) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let work_days = match coerce::to_number(&args[1]) {
        Ok(n) => {
            #[allow(clippy::cast_possible_truncation)]
            let d = n as i64;
            d
        }
        Err(e) => return Value::Error(e),
    };
    if args.len() == 3 {
        tracing::warn!("WORKDAY: holidays argument ignored in v0.1");
    }

    #[allow(clippy::cast_possible_truncation)]
    let mut current = start_serial.floor() as i64;
    let step: i64 = if work_days >= 0 { 1 } else { -1 };
    let mut remaining = work_days.abs();

    while remaining > 0 {
        current += step;
        #[allow(clippy::cast_precision_loss)]
        let wd = ExcelDate::from_serial(current as f64).weekday();
        if wd != 0 && wd != 6 {
            remaining -= 1;
        }
    }

    #[allow(clippy::cast_precision_loss)]
    Value::Date(ExcelDate::from_serial(current as f64))
}

// ---------------------------------------------------------------------------
// Private helper
// ---------------------------------------------------------------------------

/// Days in a month for Excel-compatible calendar (1900 is NOT a leap
/// year, except serial 60 is handled at a higher level).
fn month_days_for_excel(year: i32, month: i32) -> i32 {
    let is_leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let feb = if is_leap { 29 } else { 28 };
    let table = [31, feb, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if (1..=12).contains(&month) {
        #[allow(clippy::cast_sign_loss)]
        table[(month - 1) as usize]
    } else {
        30 // fallback, should not be reached with valid input
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use xlstream_core::{CellError, ExcelDate, Value};

    use super::*;

    // ===== DATE =====

    #[test]
    fn date_basic() {
        let result =
            builtin_date(&[Value::Number(2026.0), Value::Number(4.0), Value::Number(19.0)]);
        if let Value::Date(d) = result {
            assert_eq!(d.year_month_day(), (2026, 4, 19));
        } else {
            panic!("expected Date, got {result:?}");
        }
    }

    #[test]
    fn date_month_rollover() {
        let result =
            builtin_date(&[Value::Number(2026.0), Value::Number(13.0), Value::Number(1.0)]);
        if let Value::Date(d) = result {
            assert_eq!(d.year_month_day(), (2027, 1, 1));
        } else {
            panic!("expected Date, got {result:?}");
        }
    }

    #[test]
    fn date_wrong_arg_count() {
        assert_eq!(
            builtin_date(&[Value::Number(2026.0), Value::Number(1.0)]),
            Value::Error(CellError::Value)
        );
    }

    #[test]
    fn date_error_propagation() {
        assert_eq!(
            builtin_date(&[Value::Error(CellError::Na), Value::Number(1.0), Value::Number(1.0)]),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn date_from_text_coercion() {
        let result = builtin_date(&[
            Value::Text("2026".into()),
            Value::Text("1".into()),
            Value::Text("15".into()),
        ]);
        if let Value::Date(d) = result {
            assert_eq!(d.year_month_day(), (2026, 1, 15));
        } else {
            panic!("expected Date, got {result:?}");
        }
    }

    // ===== YEAR =====

    #[test]
    fn year_from_serial() {
        assert_eq!(builtin_year(&[Value::Number(44927.0)]), Value::Number(2023.0));
    }

    #[test]
    fn year_from_date_value() {
        assert_eq!(
            builtin_year(&[Value::Date(ExcelDate::from_serial(44927.0))]),
            Value::Number(2023.0)
        );
    }

    #[test]
    fn year_wrong_arg_count() {
        assert_eq!(builtin_year(&[]), Value::Error(CellError::Value));
    }

    #[test]
    fn year_error_propagation() {
        assert_eq!(builtin_year(&[Value::Error(CellError::Div0)]), Value::Error(CellError::Div0));
    }

    #[test]
    fn year_serial_1() {
        assert_eq!(builtin_year(&[Value::Number(1.0)]), Value::Number(1900.0));
    }

    // ===== MONTH =====

    #[test]
    fn month_from_serial() {
        assert_eq!(
            builtin_month(&[Value::Number(44927.0)]),
            Value::Number(1.0) // Jan 1, 2023
        );
    }

    #[test]
    fn month_december() {
        // Dec 31, 1900 = serial 366
        assert_eq!(builtin_month(&[Value::Number(366.0)]), Value::Number(12.0));
    }

    #[test]
    fn month_wrong_arg_count() {
        assert_eq!(builtin_month(&[]), Value::Error(CellError::Value));
    }

    #[test]
    fn month_error_propagation() {
        assert_eq!(builtin_month(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na));
    }

    #[test]
    fn month_from_date_value() {
        assert_eq!(
            builtin_month(&[Value::Date(ExcelDate::from_serial(61.0))]),
            Value::Number(3.0) // Mar 1, 1900
        );
    }

    // ===== DAY =====

    #[test]
    fn day_from_serial() {
        assert_eq!(
            builtin_day(&[Value::Number(44927.0)]),
            Value::Number(1.0) // Jan 1, 2023
        );
    }

    #[test]
    fn day_last_of_month() {
        // Dec 31, 1900 = serial 366
        assert_eq!(builtin_day(&[Value::Number(366.0)]), Value::Number(31.0));
    }

    #[test]
    fn day_wrong_arg_count() {
        assert_eq!(builtin_day(&[]), Value::Error(CellError::Value));
    }

    #[test]
    fn day_error_propagation() {
        assert_eq!(builtin_day(&[Value::Error(CellError::Ref)]), Value::Error(CellError::Ref));
    }

    #[test]
    fn day_from_date_value() {
        assert_eq!(
            builtin_day(&[Value::Date(ExcelDate::from_serial(59.0))]),
            Value::Number(28.0) // Feb 28, 1900
        );
    }

    // ===== WEEKDAY =====

    #[test]
    fn weekday_type1_default_sunday_is_1() {
        // Serial 1 = Jan 1, 1900 = Sunday
        assert_eq!(builtin_weekday(&[Value::Number(1.0)]), Value::Number(1.0));
    }

    #[test]
    fn weekday_type1_saturday_is_7() {
        // Serial 7 = Jan 7, 1900 = Saturday
        assert_eq!(builtin_weekday(&[Value::Number(7.0)]), Value::Number(7.0));
    }

    #[test]
    fn weekday_type2_monday_is_1() {
        // Serial 2 = Jan 2, 1900 = Monday
        assert_eq!(builtin_weekday(&[Value::Number(2.0), Value::Number(2.0)]), Value::Number(1.0));
    }

    #[test]
    fn weekday_type2_sunday_is_7() {
        // Serial 1 = Sunday
        assert_eq!(builtin_weekday(&[Value::Number(1.0), Value::Number(2.0)]), Value::Number(7.0));
    }

    #[test]
    fn weekday_type3_monday_is_0() {
        // Serial 2 = Monday
        assert_eq!(builtin_weekday(&[Value::Number(2.0), Value::Number(3.0)]), Value::Number(0.0));
    }

    #[test]
    fn weekday_type3_sunday_is_6() {
        // Serial 1 = Sunday
        assert_eq!(builtin_weekday(&[Value::Number(1.0), Value::Number(3.0)]), Value::Number(6.0));
    }

    #[test]
    fn weekday_invalid_type_returns_num_error() {
        assert_eq!(
            builtin_weekday(&[Value::Number(1.0), Value::Number(5.0)]),
            Value::Error(CellError::Num)
        );
    }

    #[test]
    fn weekday_wrong_arg_count() {
        assert_eq!(builtin_weekday(&[]), Value::Error(CellError::Value));
    }

    #[test]
    fn weekday_error_propagation() {
        assert_eq!(builtin_weekday(&[Value::Error(CellError::Na)]), Value::Error(CellError::Na));
    }

    // ===== EDATE =====

    #[test]
    fn edate_positive_months() {
        // Jan 31, 2026 + 1 month = Feb 28, 2026 (clamped)
        let jan31 = ExcelDate::from_ymd(2026, 1, 31);
        let result = builtin_edate(&[Value::Date(jan31), Value::Number(1.0)]);
        if let Value::Date(d) = result {
            assert_eq!(d.year_month_day(), (2026, 2, 28));
        } else {
            panic!("expected Date, got {result:?}");
        }
    }

    #[test]
    fn edate_negative_months() {
        // Mar 15, 2026 - 2 months = Jan 15, 2026
        let mar15 = ExcelDate::from_ymd(2026, 3, 15);
        let result = builtin_edate(&[Value::Date(mar15), Value::Number(-2.0)]);
        if let Value::Date(d) = result {
            assert_eq!(d.year_month_day(), (2026, 1, 15));
        } else {
            panic!("expected Date, got {result:?}");
        }
    }

    #[test]
    fn edate_zero_months() {
        let jan15 = ExcelDate::from_ymd(2026, 1, 15);
        let result = builtin_edate(&[Value::Date(jan15), Value::Number(0.0)]);
        if let Value::Date(d) = result {
            assert_eq!(d.year_month_day(), (2026, 1, 15));
        } else {
            panic!("expected Date, got {result:?}");
        }
    }

    #[test]
    fn edate_wrong_arg_count() {
        assert_eq!(builtin_edate(&[Value::Number(44927.0)]), Value::Error(CellError::Value));
    }

    #[test]
    fn edate_error_propagation() {
        assert_eq!(
            builtin_edate(&[Value::Error(CellError::Div0), Value::Number(1.0)]),
            Value::Error(CellError::Div0)
        );
    }

    #[test]
    fn edate_leap_year_clamp() {
        // Jan 31, 2024 + 1 month = Feb 29, 2024 (leap year)
        let jan31 = ExcelDate::from_ymd(2024, 1, 31);
        let result = builtin_edate(&[Value::Date(jan31), Value::Number(1.0)]);
        if let Value::Date(d) = result {
            assert_eq!(d.year_month_day(), (2024, 2, 29));
        } else {
            panic!("expected Date, got {result:?}");
        }
    }

    // ===== EOMONTH =====

    #[test]
    fn eomonth_same_month() {
        // Jan 15, 2026, +0 months = Jan 31, 2026
        let jan15 = ExcelDate::from_ymd(2026, 1, 15);
        let result = builtin_eomonth(&[Value::Date(jan15), Value::Number(0.0)]);
        if let Value::Date(d) = result {
            assert_eq!(d.year_month_day(), (2026, 1, 31));
        } else {
            panic!("expected Date, got {result:?}");
        }
    }

    #[test]
    fn eomonth_next_month() {
        // Jan 15, 2026, +1 month = Feb 28, 2026
        let jan15 = ExcelDate::from_ymd(2026, 1, 15);
        let result = builtin_eomonth(&[Value::Date(jan15), Value::Number(1.0)]);
        if let Value::Date(d) = result {
            assert_eq!(d.year_month_day(), (2026, 2, 28));
        } else {
            panic!("expected Date, got {result:?}");
        }
    }

    #[test]
    fn eomonth_negative_months() {
        // Mar 15, 2026, -1 month = Feb 28, 2026
        let mar15 = ExcelDate::from_ymd(2026, 3, 15);
        let result = builtin_eomonth(&[Value::Date(mar15), Value::Number(-1.0)]);
        if let Value::Date(d) = result {
            assert_eq!(d.year_month_day(), (2026, 2, 28));
        } else {
            panic!("expected Date, got {result:?}");
        }
    }

    #[test]
    fn eomonth_wrong_arg_count() {
        assert_eq!(builtin_eomonth(&[Value::Number(44927.0)]), Value::Error(CellError::Value));
    }

    #[test]
    fn eomonth_error_propagation() {
        assert_eq!(
            builtin_eomonth(&[Value::Error(CellError::Na), Value::Number(1.0)]),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn eomonth_leap_year_feb() {
        // Jan 15, 2024, +1 = Feb 29, 2024
        let jan15 = ExcelDate::from_ymd(2024, 1, 15);
        let result = builtin_eomonth(&[Value::Date(jan15), Value::Number(1.0)]);
        if let Value::Date(d) = result {
            assert_eq!(d.year_month_day(), (2024, 2, 29));
        } else {
            panic!("expected Date, got {result:?}");
        }
    }

    // ===== DATEDIF =====

    #[test]
    fn datedif_days() {
        let start = ExcelDate::from_ymd(2026, 1, 1);
        let end = ExcelDate::from_ymd(2026, 1, 31);
        assert_eq!(
            builtin_datedif(&[Value::Date(start), Value::Date(end), Value::Text("d".into())]),
            Value::Number(30.0)
        );
    }

    #[test]
    fn datedif_months() {
        let start = ExcelDate::from_ymd(2026, 1, 15);
        let end = ExcelDate::from_ymd(2026, 4, 15);
        assert_eq!(
            builtin_datedif(&[Value::Date(start), Value::Date(end), Value::Text("m".into())]),
            Value::Number(3.0)
        );
    }

    #[test]
    fn datedif_years() {
        let start = ExcelDate::from_ymd(2020, 6, 15);
        let end = ExcelDate::from_ymd(2026, 6, 15);
        assert_eq!(
            builtin_datedif(&[Value::Date(start), Value::Date(end), Value::Text("y".into())]),
            Value::Number(6.0)
        );
    }

    #[test]
    fn datedif_ym() {
        let start = ExcelDate::from_ymd(2020, 3, 15);
        let end = ExcelDate::from_ymd(2026, 6, 15);
        assert_eq!(
            builtin_datedif(&[Value::Date(start), Value::Date(end), Value::Text("ym".into())]),
            Value::Number(3.0)
        );
    }

    #[test]
    fn datedif_start_after_end_returns_num() {
        let start = ExcelDate::from_ymd(2026, 6, 1);
        let end = ExcelDate::from_ymd(2026, 1, 1);
        assert_eq!(
            builtin_datedif(&[Value::Date(start), Value::Date(end), Value::Text("d".into())]),
            Value::Error(CellError::Num)
        );
    }

    #[test]
    fn datedif_invalid_unit() {
        let start = ExcelDate::from_ymd(2026, 1, 1);
        let end = ExcelDate::from_ymd(2026, 2, 1);
        assert_eq!(
            builtin_datedif(&[Value::Date(start), Value::Date(end), Value::Text("x".into())]),
            Value::Error(CellError::Num)
        );
    }

    #[test]
    fn datedif_wrong_arg_count() {
        assert_eq!(
            builtin_datedif(&[Value::Number(1.0), Value::Number(2.0)]),
            Value::Error(CellError::Value)
        );
    }

    #[test]
    fn datedif_error_propagation() {
        assert_eq!(
            builtin_datedif(&[
                Value::Error(CellError::Ref),
                Value::Number(2.0),
                Value::Text("d".into())
            ]),
            Value::Error(CellError::Ref)
        );
    }

    #[test]
    fn datedif_md_unit() {
        let start = ExcelDate::from_ymd(2026, 1, 15);
        let end = ExcelDate::from_ymd(2026, 3, 10);
        assert_eq!(
            builtin_datedif(&[Value::Date(start), Value::Date(end), Value::Text("md".into())]),
            Value::Number(23.0) // Mar 10 - Jan 15 ignoring months: wraps Feb
        );
    }

    #[test]
    fn datedif_yd_unit() {
        let start = ExcelDate::from_ymd(2020, 3, 1);
        let end = ExcelDate::from_ymd(2026, 6, 15);
        assert_eq!(
            builtin_datedif(&[Value::Date(start), Value::Date(end), Value::Text("yd".into())]),
            Value::Number(106.0) // Mar 1 to Jun 15 = 106 days
        );
    }

    // ===== NETWORKDAYS =====

    #[test]
    fn networkdays_one_week() {
        // Mon Jan 5, 2026 to Fri Jan 9, 2026 = 5 working days
        let start = ExcelDate::from_ymd(2026, 1, 5);
        let end = ExcelDate::from_ymd(2026, 1, 9);
        assert_eq!(
            builtin_networkdays(&[Value::Date(start), Value::Date(end)]),
            Value::Number(5.0)
        );
    }

    #[test]
    fn networkdays_includes_weekends() {
        // Mon Jan 5 to Mon Jan 12 = 6 working days (skip Sat+Sun)
        let start = ExcelDate::from_ymd(2026, 1, 5);
        let end = ExcelDate::from_ymd(2026, 1, 12);
        assert_eq!(
            builtin_networkdays(&[Value::Date(start), Value::Date(end)]),
            Value::Number(6.0)
        );
    }

    #[test]
    fn networkdays_same_day_weekday() {
        let mon = ExcelDate::from_ymd(2026, 1, 5);
        assert_eq!(builtin_networkdays(&[Value::Date(mon), Value::Date(mon)]), Value::Number(1.0));
    }

    #[test]
    fn networkdays_same_day_weekend() {
        // Jan 3, 2026 = Saturday
        let sat = ExcelDate::from_ymd(2026, 1, 3);
        assert_eq!(builtin_networkdays(&[Value::Date(sat), Value::Date(sat)]), Value::Number(0.0));
    }

    #[test]
    fn networkdays_wrong_arg_count() {
        assert_eq!(builtin_networkdays(&[Value::Number(44927.0)]), Value::Error(CellError::Value));
    }

    #[test]
    fn networkdays_error_propagation() {
        assert_eq!(
            builtin_networkdays(&[Value::Error(CellError::Na), Value::Number(44927.0)]),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn networkdays_reverse_order_negative() {
        // Reversed: end < start yields negative count
        let start = ExcelDate::from_ymd(2026, 1, 9);
        let end = ExcelDate::from_ymd(2026, 1, 5);
        assert_eq!(
            builtin_networkdays(&[Value::Date(start), Value::Date(end)]),
            Value::Number(-5.0)
        );
    }

    // ===== WORKDAY =====

    #[test]
    fn workday_positive_days() {
        // Mon Jan 5, 2026 + 5 working days = Mon Jan 12
        let start = ExcelDate::from_ymd(2026, 1, 5);
        let result = builtin_workday(&[Value::Date(start), Value::Number(5.0)]);
        if let Value::Date(d) = result {
            assert_eq!(d.year_month_day(), (2026, 1, 12));
        } else {
            panic!("expected Date, got {result:?}");
        }
    }

    #[test]
    fn workday_negative_days() {
        // Mon Jan 12, 2026 - 5 working days = Mon Jan 5
        let start = ExcelDate::from_ymd(2026, 1, 12);
        let result = builtin_workday(&[Value::Date(start), Value::Number(-5.0)]);
        if let Value::Date(d) = result {
            assert_eq!(d.year_month_day(), (2026, 1, 5));
        } else {
            panic!("expected Date, got {result:?}");
        }
    }

    #[test]
    fn workday_zero_days() {
        let start = ExcelDate::from_ymd(2026, 1, 5);
        let result = builtin_workday(&[Value::Date(start), Value::Number(0.0)]);
        if let Value::Date(d) = result {
            assert_eq!(d.year_month_day(), (2026, 1, 5));
        } else {
            panic!("expected Date, got {result:?}");
        }
    }

    #[test]
    fn workday_skips_weekends() {
        // Fri Jan 9, 2026 + 1 working day = Mon Jan 12
        let start = ExcelDate::from_ymd(2026, 1, 9);
        let result = builtin_workday(&[Value::Date(start), Value::Number(1.0)]);
        if let Value::Date(d) = result {
            assert_eq!(d.year_month_day(), (2026, 1, 12));
        } else {
            panic!("expected Date, got {result:?}");
        }
    }

    #[test]
    fn workday_wrong_arg_count() {
        assert_eq!(builtin_workday(&[Value::Number(44927.0)]), Value::Error(CellError::Value));
    }

    #[test]
    fn workday_error_propagation() {
        assert_eq!(
            builtin_workday(&[Value::Error(CellError::Div0), Value::Number(1.0)]),
            Value::Error(CellError::Div0)
        );
    }

    // ===== date_serial helper =====

    #[test]
    fn date_serial_from_number() {
        assert_eq!(date_serial(&Value::Number(44927.0)), Ok(44927.0));
    }

    #[test]
    fn date_serial_from_date() {
        assert_eq!(date_serial(&Value::Date(ExcelDate::from_serial(100.0))), Ok(100.0));
    }

    #[test]
    fn date_serial_from_error() {
        assert_eq!(date_serial(&Value::Error(CellError::Na)), Err(Value::Error(CellError::Na)));
    }

    #[test]
    fn date_serial_from_text() {
        assert_eq!(date_serial(&Value::Text("100".into())), Ok(100.0));
    }

    #[test]
    fn date_serial_from_bad_text() {
        assert_eq!(date_serial(&Value::Text("abc".into())), Err(Value::Error(CellError::Value)));
    }
}
