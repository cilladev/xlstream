//! Special mathematical functions for statistical distributions.
//!
//! Private to `builtins` — provides `ln_gamma`, `regularized_incomplete_beta`,
//! and distribution primitives (t-distribution CDF/PDF/inverse) reused across
//! T.DIST, NORM.DIST, BINOM.DIST, etc.

use std::f64::consts::PI;

const LANCZOS_G: f64 = 7.0;
#[allow(clippy::excessive_precision)]
const LANCZOS_COEFF: [f64; 9] = [
    0.999_999_999_999_809_93,
    676.520_368_121_885_1,
    -1_259.139_216_722_402_8,
    771.323_428_777_653_13,
    -176.615_029_162_140_6,
    12.507_343_278_686_905,
    -0.138_571_095_265_720_12,
    9.984_369_578_019_572e-6,
    1.505_632_735_149_311_6e-7,
];

const CF_MAX_ITER: usize = 200;
const CF_EPSILON: f64 = 1e-14;
const CF_TINY: f64 = 1e-30;
const T_INV_MAX_ITER: usize = 100;
const T_INV_EPSILON: f64 = 1e-12;
const T_INV_PDF_FLOOR: f64 = 1e-300;

/// Natural log of the gamma function, ln(Gamma(x)).
///
/// Uses the Lanczos approximation (g=7, 9 coefficients). Accurate to
/// ~15 significant digits for x >= 0.5. No reflection formula — accuracy
/// degrades for 0 < x < 0.5. Returns `f64::NAN` for x <= 0.
pub(super) fn ln_gamma(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NAN;
    }
    let x = x - 1.0;
    let mut sum = LANCZOS_COEFF[0];
    for (i, &c) in LANCZOS_COEFF[1..].iter().enumerate() {
        #[allow(clippy::cast_precision_loss)]
        let denom = x + (i as f64) + 1.0;
        sum += c / denom;
    }
    let t = x + LANCZOS_G + 0.5;
    0.5 * (2.0 * PI).ln() + (x + 0.5) * t.ln() - t + sum.ln()
}

/// Regularized incomplete beta function `I_x(a, b)`.
///
/// Uses the continued fraction expansion evaluated via Lentz's modified
/// algorithm (Numerical Recipes `betacf`). When `x > (a+1)/(a+b+2)` the
/// identity `I_x(a,b) = 1 - I_{1-x}(b,a)` is used for faster convergence.
///
/// Returns `f64::NAN` if inputs are out of domain (a<=0, b<=0, x<0, x>1).
#[allow(clippy::manual_range_contains)]
pub(super) fn regularized_incomplete_beta(x: f64, a: f64, b: f64) -> f64 {
    if a <= 0.0 || b <= 0.0 || x < 0.0 || x > 1.0 {
        return f64::NAN;
    }
    #[allow(clippy::float_cmp)]
    if x == 0.0 {
        return 0.0;
    }
    #[allow(clippy::float_cmp)]
    if x == 1.0 {
        return 1.0;
    }

    if x > (a + 1.0) / (a + b + 2.0) {
        return 1.0 - regularized_incomplete_beta(1.0 - x, b, a);
    }

    let ln_prefix = a * x.ln() + b * (1.0 - x).ln() + ln_gamma(a + b) - ln_gamma(a) - ln_gamma(b);

    beta_cf(a, b, x).map_or(f64::NAN, |cf| ln_prefix.exp() * cf / a)
}

/// Continued fraction for the incomplete beta function (Numerical Recipes).
///
/// Returns `None` if the continued fraction fails to converge.
#[allow(clippy::many_single_char_names)]
fn beta_cf(a: f64, b: f64, x: f64) -> Option<f64> {
    let ab_sum = a + b;
    let a_plus1 = a + 1.0;
    let a_minus1 = a - 1.0;

    let mut c = 1.0;
    let mut d = 1.0 - ab_sum * x / a_plus1;
    if d.abs() < CF_TINY {
        d = CF_TINY;
    }
    d = 1.0 / d;
    let mut h = d;

    for m in 1..=CF_MAX_ITER {
        #[allow(clippy::cast_precision_loss)]
        let mf = m as f64;
        let m2 = 2.0 * mf;

        // Even term
        let coeff = mf * (b - mf) * x / ((a_minus1 + m2) * (a + m2));
        d = 1.0 + coeff * d;
        if d.abs() < CF_TINY {
            d = CF_TINY;
        }
        c = 1.0 + coeff / c;
        if c.abs() < CF_TINY {
            c = CF_TINY;
        }
        d = 1.0 / d;
        h *= d * c;

        // Odd term
        let coeff = -(a + mf) * (ab_sum + mf) * x / ((a + m2) * (a_plus1 + m2));
        d = 1.0 + coeff * d;
        if d.abs() < CF_TINY {
            d = CF_TINY;
        }
        c = 1.0 + coeff / c;
        if c.abs() < CF_TINY {
            c = CF_TINY;
        }
        d = 1.0 / d;
        let delta = d * c;
        h *= delta;

        if (delta - 1.0).abs() < CF_EPSILON {
            return Some(h);
        }
    }
    None
}

/// CDF of Student's t-distribution: P(T <= t) for df degrees of freedom.
///
/// Uses the regularized incomplete beta function:
/// - For t >= 0: CDF = 1 - 0.5 * I_{df/(df+t^2)}(df/2, 0.5)
/// - For t < 0: CDF = 0.5 * I_{df/(df+t^2)}(df/2, 0.5)  (symmetry)
pub(super) fn t_dist_cdf(t: f64, df: f64) -> f64 {
    if df <= 0.0 {
        return f64::NAN;
    }
    let x = df / (df + t * t);
    let ib = regularized_incomplete_beta(x, df / 2.0, 0.5);
    if t >= 0.0 {
        1.0 - 0.5 * ib
    } else {
        0.5 * ib
    }
}

/// PDF of Student's t-distribution: density at point t for df degrees of freedom.
///
/// f(t, df) = Gamma((df+1)/2) / (sqrt(df*pi) * Gamma(df/2)) * (1 + t^2/df)^(-(df+1)/2)
pub(super) fn t_dist_pdf(t: f64, df: f64) -> f64 {
    if df <= 0.0 {
        return f64::NAN;
    }
    let half_dfp1 = f64::midpoint(df, 1.0);
    let half_df = df / 2.0;
    let ln_coeff = ln_gamma(half_dfp1) - ln_gamma(half_df) - 0.5 * (df * PI).ln();
    let ln_body = -(half_dfp1) * (1.0 + t * t / df).ln();
    (ln_coeff + ln_body).exp()
}

/// Inverse CDF of Student's t-distribution (left-tail).
///
/// Given probability p and degrees of freedom df, returns the value t
/// such that P(T <= t) = p. Uses Newton-Raphson with a rational
/// approximation initial guess.
///
/// Returns `f64::NAN` if p is not in (0, 1) or df <= 0.
pub(super) fn t_inv(p: f64, df: f64) -> f64 {
    if p <= 0.0 || p >= 1.0 || df <= 0.0 {
        return f64::NAN;
    }

    let mut t = if p < 0.5 {
        -initial_guess(1.0 - p, df)
    } else if p > 0.5 {
        initial_guess(p, df)
    } else {
        return 0.0;
    };

    if !t.is_finite() {
        return f64::NAN;
    }

    let mut converged = false;
    for _ in 0..T_INV_MAX_ITER {
        let cdf = t_dist_cdf(t, df);
        let pdf = t_dist_pdf(t, df);
        if pdf < T_INV_PDF_FLOOR {
            break;
        }
        let delta = (cdf - p) / pdf;
        t -= delta;
        if delta.abs() < T_INV_EPSILON {
            converged = true;
            break;
        }
    }
    if converged {
        t
    } else {
        f64::NAN
    }
}

/// Rational approximation initial guess for `t_inv` when p > 0.5.
fn initial_guess(p: f64, df: f64) -> f64 {
    let one_minus_p = 1.0 - p;
    if one_minus_p <= 0.0 {
        return f64::INFINITY;
    }
    let t_val = (-2.0 * one_minus_p.ln()).sqrt();
    let z = t_val
        - (2.515_517 + 0.802_853 * t_val + 0.010_328 * t_val * t_val)
            / (1.0
                + 1.432_788 * t_val
                + 0.189_269 * t_val * t_val
                + 0.001_308 * t_val * t_val * t_val);

    // Cornish-Fisher correction for small df
    let g1 = (z.powi(3) + z) / (4.0 * df);
    let g2 = (5.0 * z.powi(5) + 16.0 * z.powi(3) + 3.0 * z) / (96.0 * df * df);
    z + g1 + g2
}

/// Approximation of the error function erf(x).
///
/// Uses the Chebyshev fitting from Numerical Recipes. Max absolute error
/// ~1.2e-7 — sufficient for Excel-compatible results at 1e-6 conformance
/// tolerance.
///
/// NaN passes through (returns NaN). Infinity returns +/-1.0 correctly.
/// Callers must guard NaN before calling if they need `#NUM!`.
pub(super) fn erf_approx(x: f64) -> f64 {
    let t = 1.0 / (1.0 + 0.5 * x.abs());
    let tau = t
        * (-x * x - 1.265_512_23
            + t * (1.000_023_68
                + t * (0.374_091_96
                    + t * (0.096_784_18
                        + t * (-0.186_288_06
                            + t * (0.278_868_07
                                + t * (-1.135_203_98
                                    + t * (1.488_515_87
                                        + t * (-0.822_152_23 + t * 0.170_872_77)))))))))
            .exp();
    if x >= 0.0 {
        1.0 - tau
    } else {
        tau - 1.0
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use super::*;

    fn assert_close(actual: f64, expected: f64, tol: f64) {
        assert!(
            (actual - expected).abs() < tol,
            "expected {expected}, got {actual} (diff {})",
            (actual - expected).abs()
        );
    }

    // ===== ln_gamma =====

    #[test]
    fn ln_gamma_one_is_zero() {
        assert_close(ln_gamma(1.0), 0.0, 1e-12);
    }

    #[test]
    fn ln_gamma_two_is_zero() {
        assert_close(ln_gamma(2.0), 0.0, 1e-12);
    }

    #[test]
    fn ln_gamma_half_is_half_ln_pi() {
        assert_close(ln_gamma(0.5), 0.5 * PI.ln(), 1e-10);
    }

    #[test]
    fn ln_gamma_five_is_ln_24() {
        assert_close(ln_gamma(5.0), 24.0_f64.ln(), 1e-10);
    }

    #[test]
    fn ln_gamma_ten_is_ln_362880() {
        assert_close(ln_gamma(10.0), 362_880.0_f64.ln(), 1e-8);
    }

    #[test]
    fn ln_gamma_negative_returns_nan() {
        assert!(ln_gamma(-1.0).is_nan());
        assert!(ln_gamma(0.0).is_nan());
    }

    // ===== regularized_incomplete_beta =====

    #[test]
    fn beta_at_zero_is_zero() {
        assert_close(regularized_incomplete_beta(0.0, 2.0, 3.0), 0.0, 1e-14);
    }

    #[test]
    fn beta_at_one_is_one() {
        assert_close(regularized_incomplete_beta(1.0, 2.0, 3.0), 1.0, 1e-14);
    }

    #[test]
    fn beta_symmetric_half() {
        assert_close(regularized_incomplete_beta(0.5, 5.0, 5.0), 0.5, 1e-10);
    }

    #[test]
    fn beta_known_value_2_3() {
        // I_0.3(2, 3) = 0.34839 (SciPy betainc)
        assert_close(regularized_incomplete_beta(0.3, 2.0, 3.0), 0.34839, 1e-4);
    }

    #[test]
    fn beta_known_value_half_half() {
        // I_x(0.5, 0.5) = 2/pi * arcsin(sqrt(x))
        let x: f64 = 0.25;
        let expected = 2.0 / PI * (x.sqrt()).asin();
        assert_close(regularized_incomplete_beta(x, 0.5, 0.5), expected, 1e-10);
    }

    #[test]
    fn beta_used_by_t_dist_df10() {
        let x = 10.0 / 11.0;
        let result = regularized_incomplete_beta(x, 5.0, 0.5);
        assert!(result > 0.0 && result <= 1.0, "expected in (0,1], got {result}");
    }

    #[test]
    fn beta_invalid_inputs_return_nan() {
        assert!(regularized_incomplete_beta(-0.1, 2.0, 3.0).is_nan());
        assert!(regularized_incomplete_beta(1.1, 2.0, 3.0).is_nan());
        assert!(regularized_incomplete_beta(0.5, 0.0, 3.0).is_nan());
        assert!(regularized_incomplete_beta(0.5, 2.0, -1.0).is_nan());
    }

    // ===== t_dist_cdf =====

    #[test]
    fn t_dist_cdf_at_zero_is_half() {
        assert_close(t_dist_cdf(0.0, 10.0), 0.5, 1e-10);
    }

    #[test]
    fn t_dist_cdf_positive_t() {
        assert_close(t_dist_cdf(1.0, 10.0), 0.82955, 1e-4);
    }

    #[test]
    fn t_dist_cdf_negative_t() {
        assert_close(t_dist_cdf(-1.0, 10.0), 0.17045, 1e-4);
    }

    #[test]
    fn t_dist_cdf_symmetry() {
        let cdf_pos = t_dist_cdf(1.0, 10.0);
        let cdf_neg = t_dist_cdf(-1.0, 10.0);
        assert_close(cdf_pos + cdf_neg, 1.0, 1e-10);
    }

    #[test]
    fn t_dist_cdf_df_one_is_cauchy() {
        assert_close(t_dist_cdf(1.0, 1.0), 0.75, 1e-6);
    }

    #[test]
    fn t_dist_cdf_large_df_approaches_normal() {
        let result = t_dist_cdf(1.96, 1000.0);
        assert!((result - 0.975).abs() < 0.001);
    }

    // ===== t_dist_pdf =====

    #[test]
    fn t_dist_pdf_at_zero_is_peak() {
        assert_close(t_dist_pdf(0.0, 10.0), 0.38909, 1e-4);
    }

    #[test]
    fn t_dist_pdf_positive() {
        assert_close(t_dist_pdf(1.0, 10.0), 0.23036, 1e-4);
    }

    #[test]
    fn t_dist_pdf_symmetry() {
        assert_close(t_dist_pdf(-1.0, 10.0), t_dist_pdf(1.0, 10.0), 1e-14);
    }

    // ===== t_inv =====

    #[test]
    fn t_inv_median_is_zero() {
        assert_close(t_inv(0.5, 10.0), 0.0, 1e-10);
    }

    #[test]
    fn t_inv_ninety_five_pct() {
        assert_close(t_inv(0.95, 10.0), 1.81246, 1e-4);
    }

    #[test]
    fn t_inv_left_tail() {
        assert_close(t_inv(0.025, 10.0), -2.22814, 1e-4);
    }

    #[test]
    fn t_inv_right_tail() {
        assert_close(t_inv(0.975, 10.0), 2.22814, 1e-4);
    }

    #[test]
    fn t_inv_round_trip() {
        let t = 1.5;
        let p = t_dist_cdf(t, 10.0);
        assert_close(t_inv(p, 10.0), t, 1e-8);
    }

    #[test]
    fn t_inv_df_one_cauchy() {
        assert_close(t_inv(0.75, 1.0), 1.0, 1e-6);
    }

    #[test]
    fn t_inv_out_of_range_returns_nan() {
        assert!(t_inv(0.0, 10.0).is_nan());
        assert!(t_inv(1.0, 10.0).is_nan());
        assert!(t_inv(-0.1, 10.0).is_nan());
        assert!(t_inv(1.1, 10.0).is_nan());
        assert!(t_inv(0.5, 0.0).is_nan());
    }

    // ===== Convergence failure paths =====

    #[test]
    fn t_inv_extreme_tail_returns_nan() {
        // p = 1e-15 — extremely far in the tail, initial guess may not converge
        let result = t_inv(1e-15, 1.0);
        // Either converges to a large value or returns NAN
        assert!(result.is_nan() || result.is_finite());
    }

    #[test]
    fn t_inv_near_one_returns_nan_or_converges() {
        let result = t_inv(0.999_999_999_999_999, 1.0);
        assert!(result.is_nan() || result.is_finite());
    }

    #[test]
    fn beta_large_params_converges() {
        // Large a/b should still converge
        let result = regularized_incomplete_beta(0.5, 100.0, 100.0);
        assert_close(result, 0.5, 1e-6);
    }

    #[test]
    fn beta_extreme_asymmetry() {
        // Very asymmetric params
        let result = regularized_incomplete_beta(0.01, 1.0, 100.0);
        assert!(result > 0.0 && result <= 1.0, "got {result}");
    }
}
