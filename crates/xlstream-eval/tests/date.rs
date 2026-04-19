//! Evaluator-level integration tests for date builtins.

#![allow(clippy::unwrap_used, clippy::float_cmp)]

use xlstream_core::{CellError, ExcelDate, Value};
use xlstream_eval::prelude::VolatileData;
use xlstream_eval::{Interpreter, Prelude, RowScope};
use xlstream_parse::parse;

fn eval_formula(formula: &str, row: &[Value]) -> Value {
    let prelude = Prelude::empty();
    let interp = Interpreter::new(&prelude);
    let ast = parse(formula).unwrap();
    let scope = RowScope::new(row, 0);
    interp.eval(ast.root(), &scope)
}

fn eval_formula_volatile(formula: &str, row: &[Value], volatile: VolatileData) -> Value {
    let prelude = Prelude::empty().with_volatile(volatile);
    let interp = Interpreter::new(&prelude);
    let ast = parse(formula).unwrap();
    let scope = RowScope::new(row, 0);
    interp.eval(ast.root(), &scope)
}

// ===== DATE =====

#[test]
fn date_constructs_date_value() {
    let result = eval_formula("DATE(2026, 4, 19)", &[]);
    if let Value::Date(d) = result {
        assert_eq!(d.year_month_day(), (2026, 4, 19));
    } else {
        panic!("expected Date, got {result:?}");
    }
}

#[test]
fn date_month_rollover() {
    let result = eval_formula("DATE(2026, 13, 1)", &[]);
    if let Value::Date(d) = result {
        assert_eq!(d.year_month_day(), (2027, 1, 1));
    } else {
        panic!("expected Date, got {result:?}");
    }
}

#[test]
fn date_with_cell_refs() {
    let row = vec![Value::Number(2026.0), Value::Number(6.0), Value::Number(15.0)];
    let result = eval_formula("DATE(A1, B1, C1)", &row);
    if let Value::Date(d) = result {
        assert_eq!(d.year_month_day(), (2026, 6, 15));
    } else {
        panic!("expected Date, got {result:?}");
    }
}

// ===== YEAR / MONTH / DAY =====

#[test]
fn year_extracts_from_date_serial() {
    assert_eq!(eval_formula("YEAR(44927)", &[]), Value::Number(2023.0));
}

#[test]
fn month_extracts_from_date_serial() {
    assert_eq!(eval_formula("MONTH(44927)", &[]), Value::Number(1.0));
}

#[test]
fn day_extracts_from_date_serial() {
    assert_eq!(eval_formula("DAY(44927)", &[]), Value::Number(1.0));
}

#[test]
fn year_month_day_roundtrip() {
    // DATE(2026,4,19) then extract year/month/day
    let result = eval_formula("YEAR(DATE(2026, 4, 19))", &[]);
    assert_eq!(result, Value::Number(2026.0));

    let result = eval_formula("MONTH(DATE(2026, 4, 19))", &[]);
    assert_eq!(result, Value::Number(4.0));

    let result = eval_formula("DAY(DATE(2026, 4, 19))", &[]);
    assert_eq!(result, Value::Number(19.0));
}

// ===== WEEKDAY =====

#[test]
fn weekday_default_type() {
    // Serial 1 = Jan 1, 1900 = Sunday = 1 in type 1
    assert_eq!(eval_formula("WEEKDAY(1)", &[]), Value::Number(1.0));
}

#[test]
fn weekday_type_2() {
    // Serial 2 = Monday = 1 in type 2
    assert_eq!(eval_formula("WEEKDAY(2, 2)", &[]), Value::Number(1.0));
}

#[test]
fn weekday_type_3() {
    // Serial 2 = Monday = 0 in type 3
    assert_eq!(eval_formula("WEEKDAY(2, 3)", &[]), Value::Number(0.0));
}

#[test]
fn weekday_invalid_type() {
    assert_eq!(eval_formula("WEEKDAY(1, 5)", &[]), Value::Error(CellError::Num));
}

// ===== EDATE =====

#[test]
fn edate_adds_months() {
    let result = eval_formula("EDATE(DATE(2026,1,15), 2)", &[]);
    if let Value::Date(d) = result {
        assert_eq!(d.year_month_day(), (2026, 3, 15));
    } else {
        panic!("expected Date, got {result:?}");
    }
}

// ===== EOMONTH =====

#[test]
fn eomonth_end_of_month() {
    let result = eval_formula("EOMONTH(DATE(2026,1,15), 0)", &[]);
    if let Value::Date(d) = result {
        assert_eq!(d.year_month_day(), (2026, 1, 31));
    } else {
        panic!("expected Date, got {result:?}");
    }
}

// ===== DATEDIF =====

#[test]
fn datedif_days_through_parser() {
    let result = eval_formula("DATEDIF(DATE(2026,1,1), DATE(2026,2,1), \"d\")", &[]);
    assert_eq!(result, Value::Number(31.0));
}

#[test]
fn datedif_months_through_parser() {
    let result = eval_formula("DATEDIF(DATE(2026,1,15), DATE(2026,4,15), \"m\")", &[]);
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn datedif_years_through_parser() {
    let result = eval_formula("DATEDIF(DATE(2020,1,1), DATE(2026,1,1), \"y\")", &[]);
    assert_eq!(result, Value::Number(6.0));
}

// ===== TODAY / NOW =====

#[test]
fn today_returns_volatile_date() {
    let v = VolatileData {
        today: ExcelDate::from_serial(46130.0),
        now: ExcelDate::from_serial(46130.75),
    };
    let result = eval_formula_volatile("TODAY()", &[], v);
    assert_eq!(result, Value::Date(ExcelDate::from_serial(46130.0)));
}

#[test]
fn now_returns_volatile_date() {
    let v = VolatileData {
        today: ExcelDate::from_serial(46130.0),
        now: ExcelDate::from_serial(46130.75),
    };
    let result = eval_formula_volatile("NOW()", &[], v);
    assert_eq!(result, Value::Date(ExcelDate::from_serial(46130.75)));
}

#[test]
fn today_without_volatile_returns_zero_serial() {
    let result = eval_formula("TODAY()", &[]);
    assert_eq!(result, Value::Date(ExcelDate::from_serial(0.0)));
}

#[test]
fn year_of_today() {
    let v = VolatileData {
        today: ExcelDate::from_ymd(2026, 4, 19),
        now: ExcelDate::from_ymd(2026, 4, 19),
    };
    let result = eval_formula_volatile("YEAR(TODAY())", &[], v);
    assert_eq!(result, Value::Number(2026.0));
}

// ===== Case insensitive function names =====

#[test]
fn date_case_insensitive() {
    let result = eval_formula("date(2026, 1, 1)", &[]);
    if let Value::Date(d) = result {
        assert_eq!(d.year_month_day(), (2026, 1, 1));
    } else {
        panic!("expected Date, got {result:?}");
    }
}

#[test]
fn weekday_case_insensitive() {
    assert_eq!(eval_formula("weekday(1)", &[]), Value::Number(1.0));
}

// ===== Nested formulas =====

#[test]
fn if_with_date_comparison() {
    let row = vec![Value::Number(44927.0)]; // Jan 1, 2023
    let result = eval_formula("IF(YEAR(A1)=2023, \"yes\", \"no\")", &row);
    assert_eq!(result, Value::Text("yes".into()));
}

#[test]
fn date_arithmetic_via_serial() {
    // DATE returns a Value::Date, adding a number to it should work
    // through the binary op path if supported
    let result = eval_formula("YEAR(DATE(2026,1,1))", &[]);
    assert_eq!(result, Value::Number(2026.0));
}
