//! Financial builtin functions (Phase 9, Chunk 3).
//!
//! Standard time-value-of-money formulas: PMT, PV, FV, NPV, IRR, RATE.
//! Pure builtins take `&[Value]` and return `Value`.
//! Stateful builtins (IRR, NPV) take AST nodes for range expansion.

use xlstream_core::{coerce, CellError, Value};
use xlstream_parse::NodeRef;

use crate::interp::Interpreter;
use crate::scope::RowScope;

/// Maximum Newton-Raphson iterations for IRR and RATE. Matches Excel's
/// documented 100-iteration limit.
const MAX_NEWTON_ITERATIONS: usize = 100;

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Extract a required numeric argument at `idx`, propagating errors.
fn num_arg(args: &[Value], idx: usize) -> Result<f64, Value> {
    let v = args.get(idx).unwrap_or(&Value::Empty);
    coerce::to_number(v).map_err(Value::Error)
}

/// Extract an optional numeric argument at `idx` with a default value.
fn opt_num(args: &[Value], idx: usize, default: f64) -> Result<f64, Value> {
    if idx >= args.len() {
        return Ok(default);
    }
    num_arg(args, idx)
}

/// Guard against NaN/Infinity results from TVM calculations.
fn finite_or_num(result: f64) -> Value {
    if result.is_nan() || result.is_infinite() {
        Value::Error(CellError::Num)
    } else {
        Value::Number(result)
    }
}

// ---------------------------------------------------------------------------
// PMT
// ---------------------------------------------------------------------------

/// `PMT(rate, nper, pv, fv?, type?)` — periodic payment for a loan.
///
/// When `rate` is 0: `-(pv + fv) / nper`.
/// When `rate` != 0: `-(rate * (pv * pow + fv)) / (type_factor * (pow - 1))`
/// where `pow = (1 + rate)^nper` and `type_factor = 1 + rate * type`.
///
/// Returns `#NUM!` when `nper` is 0.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::financial::builtin_pmt;
/// let result = builtin_pmt(&[Value::Number(0.0), Value::Number(12.0), Value::Number(1200.0)]);
/// match result {
///     Value::Number(n) => assert!((n - (-100.0)).abs() < 0.01),
///     _ => panic!("expected Number"),
/// }
/// ```
#[must_use]
pub fn builtin_pmt(args: &[Value]) -> Value {
    if args.len() < 3 || args.len() > 5 {
        return Value::Error(CellError::Value);
    }
    let rate = match num_arg(args, 0) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let nper = match num_arg(args, 1) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let pv = match num_arg(args, 2) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let fv = match opt_num(args, 3, 0.0) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let pmt_type = match opt_num(args, 4, 0.0) {
        Ok(v) => v,
        Err(e) => return e,
    };

    if nper == 0.0 {
        return Value::Error(CellError::Num);
    }

    if rate == 0.0 {
        return finite_or_num(-(pv + fv) / nper);
    }

    let pow = (1.0 + rate).powf(nper);
    let type_factor = 1.0 + rate * pmt_type;
    finite_or_num(-(rate * (pv * pow + fv)) / (type_factor * (pow - 1.0)))
}

// ---------------------------------------------------------------------------
// PV
// ---------------------------------------------------------------------------

/// `PV(rate, nper, pmt, fv?, type?)` — present value.
///
/// When `rate` is 0: `-(pmt * nper + fv)`.
/// When `rate` != 0: `-(pmt * type_factor * (pow - 1) / rate + fv) / pow`.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::financial::builtin_pv;
/// let result = builtin_pv(&[Value::Number(0.0), Value::Number(12.0), Value::Number(-100.0)]);
/// match result {
///     Value::Number(n) => assert!((n - 1200.0).abs() < 0.01),
///     _ => panic!("expected Number"),
/// }
/// ```
#[must_use]
pub fn builtin_pv(args: &[Value]) -> Value {
    if args.len() < 3 || args.len() > 5 {
        return Value::Error(CellError::Value);
    }
    let rate = match num_arg(args, 0) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let nper = match num_arg(args, 1) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let pmt = match num_arg(args, 2) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let fv = match opt_num(args, 3, 0.0) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let pmt_type = match opt_num(args, 4, 0.0) {
        Ok(v) => v,
        Err(e) => return e,
    };

    if rate == 0.0 {
        return finite_or_num(-(pmt * nper + fv));
    }

    let pow = (1.0 + rate).powf(nper);
    let type_factor = 1.0 + rate * pmt_type;
    finite_or_num(-(pmt * type_factor * (pow - 1.0) / rate + fv) / pow)
}

// ---------------------------------------------------------------------------
// FV
// ---------------------------------------------------------------------------

/// `FV(rate, nper, pmt, pv?, type?)` — future value.
///
/// When `rate` is 0: `-(pv + pmt * nper)`.
/// When `rate` != 0: `-(pv * pow + pmt * type_factor * (pow - 1) / rate)`.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::financial::builtin_fv;
/// let result = builtin_fv(&[Value::Number(0.0), Value::Number(12.0), Value::Number(-100.0)]);
/// match result {
///     Value::Number(n) => assert!((n - 1200.0).abs() < 0.01),
///     _ => panic!("expected Number"),
/// }
/// ```
#[must_use]
pub fn builtin_fv(args: &[Value]) -> Value {
    if args.len() < 3 || args.len() > 5 {
        return Value::Error(CellError::Value);
    }
    let rate = match num_arg(args, 0) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let nper = match num_arg(args, 1) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let pmt = match num_arg(args, 2) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let pv = match opt_num(args, 3, 0.0) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let pmt_type = match opt_num(args, 4, 0.0) {
        Ok(v) => v,
        Err(e) => return e,
    };

    if rate == 0.0 {
        return finite_or_num(-(pv + pmt * nper));
    }

    let pow = (1.0 + rate).powf(nper);
    let type_factor = 1.0 + rate * pmt_type;
    finite_or_num(-(pv * pow + pmt * type_factor * (pow - 1.0) / rate))
}

// ---------------------------------------------------------------------------
// NPV
// ---------------------------------------------------------------------------

/// Compute NPV from a rate and cashflow slice.
///
/// Discounts each cashflow at `rate`, starting at period 1.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::financial::npv_from_values;
/// let result = npv_from_values(0.10, &[-1000.0, 300.0, 400.0, 500.0]);
/// match result {
///     Value::Number(n) => assert!((n - (-19.12)).abs() < 1.0),
///     _ => panic!("expected Number"),
/// }
/// ```
#[must_use]
pub fn npv_from_values(rate: f64, cashflows: &[f64]) -> Value {
    let mut npv = 0.0;
    for (i, &cf) in cashflows.iter().enumerate() {
        #[allow(clippy::cast_precision_loss)]
        let period = (i + 1) as f64;
        npv += cf / (1.0 + rate).powf(period);
    }
    finite_or_num(npv)
}

/// `NPV(rate, value1, value2, ...)` — net present value (stateful).
///
/// Evaluates the rate arg normally, then expands remaining args via
/// `expand_range` to support bounded range references.
pub(crate) fn builtin_npv(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.len() < 2 {
        return Value::Error(CellError::Value);
    }
    let rate_val = interp.eval(args[0], scope);
    let rate = match coerce::to_number(&rate_val) {
        Ok(v) => v,
        Err(e) => return Value::Error(e),
    };

    let mut cashflows = Vec::new();
    for &arg in &args[1..] {
        for v in super::expand_range(arg, interp, scope) {
            match coerce::to_number(&v) {
                Ok(n) => cashflows.push(n),
                Err(e) => return Value::Error(e),
            }
        }
    }

    npv_from_values(rate, &cashflows)
}

// ---------------------------------------------------------------------------
// IRR
// ---------------------------------------------------------------------------

/// Compute IRR from a cashflow slice using Newton-Raphson iteration.
///
/// Up to 100 iterations, 1e-10 threshold. Default guess is 0.1.
///
/// Returns `#NUM!` if values don't contain both positive and negative
/// cashflows, or if the iteration doesn't converge.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::financial::irr_from_cashflows;
/// let result = irr_from_cashflows(&[-1000.0, 300.0, 400.0, 500.0, 200.0]);
/// match result {
///     Value::Number(n) => assert!((n - 0.1532).abs() < 0.01),
///     _ => panic!("expected Number"),
/// }
/// ```
#[must_use]
pub fn irr_from_cashflows(cashflows: &[f64]) -> Value {
    if cashflows.len() < 2 {
        return Value::Error(CellError::Num);
    }

    // Must have both positive and negative cashflows
    let has_positive = cashflows.iter().any(|&c| c > 0.0);
    let has_negative = cashflows.iter().any(|&c| c < 0.0);
    if !has_positive || !has_negative {
        return Value::Error(CellError::Num);
    }

    let mut rate: f64 = 0.1; // default guess

    for _ in 0..MAX_NEWTON_ITERATIONS {
        let mut npv: f64 = 0.0;
        let mut d_npv: f64 = 0.0;
        for (t, &cf) in cashflows.iter().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let t_f = t as f64;
            let denom = (1.0_f64 + rate).powf(t_f);
            if denom == 0.0 {
                return Value::Error(CellError::Num);
            }
            npv += cf / denom;
            if t > 0 {
                d_npv -= t_f * cf / (1.0_f64 + rate).powf(t_f + 1.0);
            }
        }

        if npv.abs() < 1e-10 {
            return Value::Number(rate);
        }

        if d_npv == 0.0 {
            return Value::Error(CellError::Num);
        }

        rate -= npv / d_npv;
    }

    Value::Error(CellError::Num)
}

/// `IRR(values, guess?)` — internal rate of return (stateful).
///
/// Expands all args via `expand_range` to collect cashflows, then
/// delegates to [`irr_from_cashflows`].
pub(crate) fn builtin_irr(
    args: &[NodeRef<'_>],
    interp: &Interpreter<'_>,
    scope: &RowScope<'_>,
) -> Value {
    if args.is_empty() {
        return Value::Error(CellError::Value);
    }

    let mut cashflows = Vec::new();
    for &arg in args {
        for v in super::expand_range(arg, interp, scope) {
            match coerce::to_number(&v) {
                Ok(n) => cashflows.push(n),
                Err(e) => return Value::Error(e),
            }
        }
    }

    irr_from_cashflows(&cashflows)
}

// ---------------------------------------------------------------------------
// RATE
// ---------------------------------------------------------------------------

/// `RATE(nper, pmt, pv, fv?, type?, guess?)` — interest rate per period.
///
/// Uses Newton-Raphson iteration (up to 100 iterations, 1e-10 threshold).
/// Default guess is 0.1.
///
/// Returns `#NUM!` when `pmt == 0 && pv == 0`, or on non-convergence.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::financial::builtin_rate;
/// let result = builtin_rate(&[
///     Value::Number(360.0),
///     Value::Number(-1073.64),
///     Value::Number(200000.0),
/// ]);
/// match result {
///     Value::Number(n) => assert!((n - 0.05 / 12.0).abs() < 0.0001),
///     _ => panic!("expected Number"),
/// }
/// ```
#[must_use]
pub fn builtin_rate(args: &[Value]) -> Value {
    if args.len() < 3 || args.len() > 6 {
        return Value::Error(CellError::Value);
    }
    let nper = match num_arg(args, 0) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let pmt = match num_arg(args, 1) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let pv = match num_arg(args, 2) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let fv = match opt_num(args, 3, 0.0) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let pmt_type = match opt_num(args, 4, 0.0) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let guess = match opt_num(args, 5, 0.1) {
        Ok(v) => v,
        Err(e) => return e,
    };

    if pmt == 0.0 && pv == 0.0 {
        return Value::Error(CellError::Num);
    }

    let mut rate = guess;

    for _ in 0..MAX_NEWTON_ITERATIONS {
        if rate == 0.0 {
            // Special case: avoid division by zero in derivative
            // f(0) = pmt * nper + pv + fv
            let f = pmt * nper + pv + fv;
            if f.abs() < 1e-10 {
                return Value::Number(0.0);
            }
            // Nudge away from zero
            rate = 1e-10;
            continue;
        }

        let pow = (1.0 + rate).powf(nper);
        let type_factor = 1.0 + rate * pmt_type;

        // f(rate) = pv * pow + pmt * type_factor * (pow - 1) / rate + fv
        let f = pv * pow + pmt * type_factor * (pow - 1.0) / rate + fv;

        // f'(rate) = pv * nper * (1+rate)^(nper-1)
        //          + pmt * [ pmt_type * (pow - 1) / rate
        //                  + type_factor * nper * (1+rate)^(nper-1) / rate
        //                  - type_factor * (pow - 1) / rate^2 ]
        let pow_m1 = (1.0 + rate).powf(nper - 1.0);
        let df = pv * nper * pow_m1
            + pmt
                * (pmt_type * (pow - 1.0) / rate + type_factor * nper * pow_m1 / rate
                    - type_factor * (pow - 1.0) / (rate * rate));

        if f.abs() < 1e-7 {
            return Value::Number(rate);
        }

        if df == 0.0 {
            return Value::Error(CellError::Num);
        }

        let step = f / df;
        rate -= step;

        // If the step is negligible relative to rate, we've converged
        if step.abs() < 1e-10 * rate.abs().max(1e-10) {
            return Value::Number(rate);
        }
    }

    Value::Error(CellError::Num)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use xlstream_core::{CellError, Value};

    use super::*;

    // ===== PMT =====

    #[test]
    fn pmt_standard_loan() {
        // PMT(0.05/12, 360, 200000) ~ -1073.64
        let result = builtin_pmt(&[
            Value::Number(0.05 / 12.0),
            Value::Number(360.0),
            Value::Number(200_000.0),
        ]);
        match result {
            Value::Number(n) => assert!((n - (-1073.64)).abs() < 0.01),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn pmt_zero_rate() {
        // PMT(0, 12, 1200) = -100
        let result = builtin_pmt(&[Value::Number(0.0), Value::Number(12.0), Value::Number(1200.0)]);
        match result {
            Value::Number(n) => assert!((n - (-100.0)).abs() < 0.01),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn pmt_with_fv() {
        // PMT(0.05/12, 360, 200000, 10000) — loan with residual
        let result = builtin_pmt(&[
            Value::Number(0.05 / 12.0),
            Value::Number(360.0),
            Value::Number(200_000.0),
            Value::Number(10_000.0),
        ]);
        match result {
            Value::Number(n) => assert!(n < -1073.0), // larger payment due to fv
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn pmt_nper_zero_returns_num_error() {
        assert_eq!(
            builtin_pmt(&[Value::Number(0.05), Value::Number(0.0), Value::Number(1000.0)]),
            Value::Error(CellError::Num)
        );
    }

    #[test]
    fn pmt_error_propagation() {
        assert_eq!(
            builtin_pmt(&[Value::Error(CellError::Na), Value::Number(12.0), Value::Number(1000.0)]),
            Value::Error(CellError::Na)
        );
    }

    #[test]
    fn pmt_wrong_arg_count() {
        assert_eq!(
            builtin_pmt(&[Value::Number(0.05), Value::Number(12.0)]),
            Value::Error(CellError::Value)
        );
    }

    #[test]
    fn pmt_with_type_begin() {
        // type=1 (beginning of period) should give different result
        let end = builtin_pmt(&[
            Value::Number(0.05 / 12.0),
            Value::Number(360.0),
            Value::Number(200_000.0),
            Value::Number(0.0),
            Value::Number(0.0),
        ]);
        let begin = builtin_pmt(&[
            Value::Number(0.05 / 12.0),
            Value::Number(360.0),
            Value::Number(200_000.0),
            Value::Number(0.0),
            Value::Number(1.0),
        ]);
        match (end, begin) {
            (Value::Number(e), Value::Number(b)) => {
                // Beginning-of-period payment should be smaller in magnitude
                assert!(b.abs() < e.abs());
            }
            (e, b) => panic!("expected Numbers, got {e:?} and {b:?}"),
        }
    }

    // ===== PV =====

    #[test]
    fn pv_roundtrip_with_pmt() {
        // PV(0.05/12, 360, -1073.64) ~ 200000
        let result = builtin_pv(&[
            Value::Number(0.05 / 12.0),
            Value::Number(360.0),
            Value::Number(-1073.64),
        ]);
        match result {
            Value::Number(n) => assert!((n - 200_000.0).abs() < 100.0),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn pv_zero_rate() {
        // PV(0, 12, -100) = 1200
        let result = builtin_pv(&[Value::Number(0.0), Value::Number(12.0), Value::Number(-100.0)]);
        match result {
            Value::Number(n) => assert!((n - 1200.0).abs() < 0.01),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn pv_with_fv() {
        let result = builtin_pv(&[
            Value::Number(0.05),
            Value::Number(10.0),
            Value::Number(0.0),
            Value::Number(-1000.0),
        ]);
        match result {
            Value::Number(n) => assert!(n > 0.0), // positive PV for future value
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn pv_error_propagation() {
        assert_eq!(
            builtin_pv(&[Value::Number(0.05), Value::Error(CellError::Div0), Value::Number(100.0)]),
            Value::Error(CellError::Div0)
        );
    }

    #[test]
    fn pv_wrong_arg_count() {
        assert_eq!(
            builtin_pv(&[Value::Number(0.05), Value::Number(12.0)]),
            Value::Error(CellError::Value)
        );
    }

    // ===== FV =====

    #[test]
    fn fv_zero_rate() {
        // FV(0, 12, -100) = 1200
        let result = builtin_fv(&[Value::Number(0.0), Value::Number(12.0), Value::Number(-100.0)]);
        match result {
            Value::Number(n) => assert!((n - 1200.0).abs() < 0.01),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn fv_with_rate() {
        // FV(0.05/12, 360, -100) — savings accumulation
        let result =
            builtin_fv(&[Value::Number(0.05 / 12.0), Value::Number(360.0), Value::Number(-100.0)]);
        match result {
            Value::Number(n) => assert!(n > 1200.0 * 12.0), // compound interest > simple
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn fv_with_pv() {
        // FV(0.05, 10, 0, -1000) — simple compounding of lump sum
        let result = builtin_fv(&[
            Value::Number(0.05),
            Value::Number(10.0),
            Value::Number(0.0),
            Value::Number(-1000.0),
        ]);
        match result {
            Value::Number(n) => assert!((n - 1628.89).abs() < 1.0),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn fv_error_propagation() {
        assert_eq!(
            builtin_fv(&[Value::Number(0.05), Value::Number(10.0), Value::Error(CellError::Ref)]),
            Value::Error(CellError::Ref)
        );
    }

    #[test]
    fn fv_wrong_arg_count() {
        assert_eq!(
            builtin_fv(&[Value::Number(0.05), Value::Number(10.0)]),
            Value::Error(CellError::Value)
        );
    }

    // ===== NPV =====

    #[test]
    fn npv_basic() {
        // NPV(0.10, -1000, 300, 400, 500) ~ -19.12
        let result = npv_from_values(0.10, &[-1000.0, 300.0, 400.0, 500.0]);
        match result {
            Value::Number(n) => assert!((n - (-19.12)).abs() < 1.0),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn npv_positive_project() {
        // NPV(0.05, -1000, 600, 600) > 0
        let result = npv_from_values(0.05, &[-1000.0, 600.0, 600.0]);
        match result {
            Value::Number(n) => assert!(n > 0.0),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn npv_zero_rate() {
        // NPV(0, 100, 200, 300) = 600
        let result = npv_from_values(0.0, &[100.0, 200.0, 300.0]);
        match result {
            Value::Number(n) => assert!((n - 600.0).abs() < 0.01),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    // ===== IRR =====

    #[test]
    fn irr_convergence() {
        // IRR(-1000, 300, 400, 500, 200) ~ 0.1532
        let result = irr_from_cashflows(&[-1000.0, 300.0, 400.0, 500.0, 200.0]);
        match result {
            Value::Number(n) => assert!((n - 0.1532).abs() < 0.01),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn irr_simple_doubling() {
        // IRR(-100, 200) = 1.0 (100% return)
        let result = irr_from_cashflows(&[-100.0, 200.0]);
        match result {
            Value::Number(n) => assert!((n - 1.0).abs() < 0.01),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn irr_no_sign_change_returns_num_error() {
        assert_eq!(irr_from_cashflows(&[100.0, 200.0, 300.0]), Value::Error(CellError::Num));
    }

    #[test]
    fn irr_all_negative_returns_num_error() {
        assert_eq!(irr_from_cashflows(&[-100.0, -200.0]), Value::Error(CellError::Num));
    }

    #[test]
    fn irr_too_few_cashflows_returns_num_error() {
        assert_eq!(irr_from_cashflows(&[-100.0]), Value::Error(CellError::Num));
    }

    // ===== RATE =====

    #[test]
    fn rate_roundtrip() {
        // RATE(360, -1073.64, 200000) ~ 0.05/12
        let result = builtin_rate(&[
            Value::Number(360.0),
            Value::Number(-1073.64),
            Value::Number(200_000.0),
        ]);
        match result {
            Value::Number(n) => assert!((n - 0.05 / 12.0).abs() < 0.0001),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn rate_simple_case() {
        // RATE(1, 0, -100, 110) = 0.10
        let result = builtin_rate(&[
            Value::Number(1.0),
            Value::Number(0.0),
            Value::Number(-100.0),
            Value::Number(110.0),
        ]);
        match result {
            Value::Number(n) => assert!((n - 0.10).abs() < 0.01),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn rate_zero_pmt_and_pv_returns_num_error() {
        assert_eq!(
            builtin_rate(&[Value::Number(12.0), Value::Number(0.0), Value::Number(0.0)]),
            Value::Error(CellError::Num)
        );
    }

    #[test]
    fn rate_error_propagation() {
        assert_eq!(
            builtin_rate(&[
                Value::Error(CellError::Div0),
                Value::Number(-100.0),
                Value::Number(1000.0)
            ]),
            Value::Error(CellError::Div0)
        );
    }

    #[test]
    fn rate_wrong_arg_count() {
        assert_eq!(
            builtin_rate(&[Value::Number(12.0), Value::Number(-100.0)]),
            Value::Error(CellError::Value)
        );
    }

    #[test]
    fn rate_with_custom_guess() {
        // Same as roundtrip but with explicit guess
        let result = builtin_rate(&[
            Value::Number(360.0),
            Value::Number(-1073.64),
            Value::Number(200_000.0),
            Value::Number(0.0),
            Value::Number(0.0),
            Value::Number(0.05),
        ]);
        match result {
            Value::Number(n) => assert!((n - 0.05 / 12.0).abs() < 0.0001),
            other => panic!("expected Number, got {other:?}"),
        }
    }
}
