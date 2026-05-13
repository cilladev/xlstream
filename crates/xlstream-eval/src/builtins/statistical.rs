//! Statistical builtin functions.
//!
//! Aggregate stats (AVEDEV, VAR, STDEV, SKEW, KURT, MODE, PERCENTILE,
//! QUARTILE, RANK) use [`collect_numerics`] and [`mean_and_variance`].
//! Distribution functions (T.DIST, EXPON.DIST, POISSON.DIST) use
//! [`num_arg`] + `specfn` primitives.

use std::collections::HashMap;

use xlstream_core::{coerce, CellError, Value};

use super::specfn;

/// Lanczos approximation of ln(Gamma(x)) for x > 0.
///
/// g=7, 9 coefficients. Accurate to ~15 digits for x >= 0.5.
/// For x in (0, 0.5), uses reflection: Gamma(x)*Gamma(1-x) = pi / sin(pi*x).
fn ln_gamma(x: f64) -> f64 {
    #[allow(clippy::excessive_precision)]
    const COEFFICIENTS: [f64; 9] = [
        0.999_999_999_999_809_93,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_9,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    const G: f64 = 7.0;
    const HALF_LN_2PI: f64 = 0.918_938_533_204_672_8;

    if x < 0.5 {
        let reflection = std::f64::consts::PI / (std::f64::consts::PI * x).sin();
        return reflection.ln() - ln_gamma(1.0 - x);
    }

    let z = x - 1.0;
    let mut sum = COEFFICIENTS[0];
    for (i, &c) in COEFFICIENTS[1..].iter().enumerate() {
        #[allow(clippy::cast_precision_loss)]
        let denom = z + (i as f64) + 1.0;
        sum += c / denom;
    }
    let t = z + G + 0.5;
    HALF_LN_2PI + (z + 0.5) * t.ln() - t + sum.ln()
}

fn finite_or_num(v: f64) -> Result<f64, CellError> {
    if v.is_finite() {
        Ok(v)
    } else {
        Err(CellError::Num)
    }
}

/// Extract numeric values from a `&[Value]` slice using range semantics.
///
/// Includes `Number`, `Integer` (cast to f64), and `Date` (serial).
/// Skips `Text`, `Bool`, and `Empty`. Propagates errors immediately.
/// NaN/Infinity values pass through — callers must guard their output.
///
/// Reused by variance, standard deviation, skewness, kurtosis, etc.
///
/// # Errors
///
/// Returns `Err(CellError)` if any value is an error variant.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::collect_numerics;
/// let vals = [Value::Number(1.0), Value::Text("x".into()), Value::Number(3.0)];
/// assert_eq!(collect_numerics(&vals).unwrap(), vec![1.0, 3.0]);
/// ```
pub fn collect_numerics(values: &[Value]) -> Result<Vec<f64>, CellError> {
    let mut nums = Vec::with_capacity(values.len());
    for v in values {
        match v {
            Value::Error(e) => return Err(*e),
            Value::Number(n) => nums.push(*n),
            #[allow(clippy::cast_precision_loss)]
            Value::Integer(i) => nums.push(*i as f64),
            Value::Date(d) => nums.push(d.serial),
            Value::Text(_) | Value::Bool(_) | Value::Empty => {}
        }
    }
    Ok(nums)
}

/// Compute the mean and variance of a `&[f64]` slice.
///
/// `ddof` is the delta degrees of freedom: 0 for population, 1 for sample.
/// Returns `None` if `nums.len() <= ddof` (too few values).
fn mean_and_variance(nums: &[f64], ddof: usize) -> Option<(f64, f64)> {
    let n = nums.len();
    if n <= ddof {
        return None;
    }
    #[allow(clippy::cast_precision_loss)]
    let nf = n as f64;
    let mean = nums.iter().sum::<f64>() / nf;
    #[allow(clippy::cast_precision_loss)]
    let variance = nums.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - ddof) as f64;
    Some((mean, variance))
}

/// `VAR.S` — sample variance (divides by n-1).
///
/// Requires at least 2 numeric values. Returns `#DIV/0!` otherwise.
/// Text, booleans, and empty cells are skipped. Errors propagate.
///
/// # Errors
///
/// Returns `Err(CellError::Div0)` if fewer than 2 numeric values.
/// Returns `Err(CellError::Num)` if the result overflows to NaN/Infinity.
/// Returns `Err(CellError)` if any value is an error.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::var_s;
/// let vals = [Value::Number(2.0), Value::Number(4.0), Value::Number(6.0), Value::Number(8.0)];
/// let result = var_s(&vals).unwrap();
/// assert!((result - 6.666_666_666_666_667).abs() < 1e-9);
/// ```
pub fn var_s(values: &[Value]) -> Result<f64, CellError> {
    let nums = collect_numerics(values)?;
    let (_, variance) = mean_and_variance(&nums, 1).ok_or(CellError::Div0)?;
    finite_or_num(variance)
}

/// `VAR.P` — population variance (divides by n).
///
/// Requires at least 1 numeric value. Returns `#DIV/0!` otherwise.
/// Text, booleans, and empty cells are skipped. Errors propagate.
///
/// # Errors
///
/// Returns `Err(CellError::Div0)` if no numeric values.
/// Returns `Err(CellError::Num)` if the result overflows to NaN/Infinity.
/// Returns `Err(CellError)` if any value is an error.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::var_p;
/// let vals = [Value::Number(2.0), Value::Number(4.0), Value::Number(6.0), Value::Number(8.0)];
/// let result = var_p(&vals).unwrap();
/// assert!((result - 5.0).abs() < 1e-9);
/// ```
pub fn var_p(values: &[Value]) -> Result<f64, CellError> {
    let nums = collect_numerics(values)?;
    let (_, variance) = mean_and_variance(&nums, 0).ok_or(CellError::Div0)?;
    finite_or_num(variance)
}

/// `STDEV.S` — sample standard deviation (sqrt of [`var_s`]).
///
/// Requires at least 2 numeric values. Returns `#DIV/0!` otherwise.
/// Text, booleans, and empty cells are skipped. Errors propagate.
///
/// # Errors
///
/// Returns `Err(CellError::Div0)` if fewer than 2 numeric values.
/// Returns `Err(CellError::Num)` if the underlying variance overflows.
/// Returns `Err(CellError)` if any value is an error.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::stdev_s;
/// let vals = [Value::Number(2.0), Value::Number(4.0), Value::Number(6.0), Value::Number(8.0)];
/// let result = stdev_s(&vals).unwrap();
/// assert!((result - 2.581_988_897_471_611).abs() < 1e-9);
/// ```
pub fn stdev_s(values: &[Value]) -> Result<f64, CellError> {
    var_s(values).map(f64::sqrt)
}

/// `STDEV.P` — population standard deviation (sqrt of [`var_p`]).
///
/// Requires at least 1 numeric value. Returns `#DIV/0!` otherwise.
/// Text, booleans, and empty cells are skipped. Errors propagate.
///
/// # Errors
///
/// Returns `Err(CellError::Div0)` if no numeric values.
/// Returns `Err(CellError::Num)` if the underlying variance overflows.
/// Returns `Err(CellError)` if any value is an error.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::stdev_p;
/// let vals = [Value::Number(2.0), Value::Number(4.0), Value::Number(6.0), Value::Number(8.0)];
/// let result = stdev_p(&vals).unwrap();
/// assert!((result - 2.236_067_977_499_79).abs() < 1e-9);
/// ```
pub fn stdev_p(values: &[Value]) -> Result<f64, CellError> {
    var_p(values).map(f64::sqrt)
}

/// `SKEW` — sample skewness (adjusted).
///
/// Formula: `[n / ((n-1)(n-2))] * sum[(xi - mean) / stdev_s]^3`
///
/// # Errors
///
/// Returns `Err(CellError::Div0)` if fewer than 3 numeric values,
/// if standard deviation is zero, or if any input is an error.
/// Returns `Err(CellError::Num)` if the result overflows to
/// infinity or NaN.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::skew;
/// let vals = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0),
///             Value::Number(4.0), Value::Number(5.0)];
/// let result = skew(&vals).unwrap();
/// assert!(result.abs() < 1e-10);
/// ```
pub fn skew(values: &[Value]) -> Result<f64, CellError> {
    let nums = collect_numerics(values)?;
    if nums.len() < 3 {
        return Err(CellError::Div0);
    }
    let (mean, variance) = mean_and_variance(&nums, 1).ok_or(CellError::Div0)?;
    let stdev = variance.sqrt();
    if stdev == 0.0 {
        return Err(CellError::Div0);
    }

    #[allow(clippy::cast_precision_loss)]
    let nf = nums.len() as f64;
    let m3: f64 = nums.iter().map(|x| ((x - mean) / stdev).powi(3)).sum();
    let adjustment = nf / ((nf - 1.0) * (nf - 2.0));

    finite_or_num(adjustment * m3)
}

/// `SKEW.P` — population skewness.
///
/// Formula: `(1/n) * sum[(xi - mean) / stdev_p]^3`
///
/// # Errors
///
/// Returns `Err(CellError::Div0)` if no numeric values, if standard
/// deviation is zero, or if any input is an error.
/// Returns `Err(CellError::Num)` if the result overflows to
/// infinity or NaN.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::skew_p;
/// let vals = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0),
///             Value::Number(4.0), Value::Number(5.0)];
/// let result = skew_p(&vals).unwrap();
/// assert!(result.abs() < 1e-10);
/// ```
pub fn skew_p(values: &[Value]) -> Result<f64, CellError> {
    let nums = collect_numerics(values)?;
    let (mean, variance) = mean_and_variance(&nums, 0).ok_or(CellError::Div0)?;
    let stdev = variance.sqrt();
    if stdev == 0.0 {
        return Err(CellError::Div0);
    }

    #[allow(clippy::cast_precision_loss)]
    let nf = nums.len() as f64;
    let m3: f64 = nums.iter().map(|x| ((x - mean) / stdev).powi(3)).sum();

    finite_or_num(m3 / nf)
}

/// `KURT` — excess kurtosis (sample-adjusted).
///
/// Formula: `[(n(n+1)) / ((n-1)(n-2)(n-3))] * sum[(xi - mean) / stdev_s]^4
///           - [3(n-1)^2 / ((n-2)(n-3))]`
///
/// # Errors
///
/// Returns `Err(CellError::Div0)` if fewer than 4 numeric values,
/// if standard deviation is zero, or if any input is an error.
/// Returns `Err(CellError::Num)` if the result overflows to
/// infinity or NaN.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::kurt;
/// let vals = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0),
///             Value::Number(4.0), Value::Number(5.0)];
/// let result = kurt(&vals).unwrap();
/// assert!((result - (-1.2)).abs() < 1e-10);
/// ```
pub fn kurt(values: &[Value]) -> Result<f64, CellError> {
    let nums = collect_numerics(values)?;
    if nums.len() < 4 {
        return Err(CellError::Div0);
    }
    let (mean, variance) = mean_and_variance(&nums, 1).ok_or(CellError::Div0)?;
    let stdev = variance.sqrt();
    if stdev == 0.0 {
        return Err(CellError::Div0);
    }

    #[allow(clippy::cast_precision_loss)]
    let nf = nums.len() as f64;
    let m4: f64 = nums.iter().map(|x| ((x - mean) / stdev).powi(4)).sum();
    let term1 = (nf * (nf + 1.0)) / ((nf - 1.0) * (nf - 2.0) * (nf - 3.0));
    let term2 = (3.0 * (nf - 1.0).powi(2)) / ((nf - 2.0) * (nf - 3.0));

    finite_or_num(term1 * m4 - term2)
}

/// `AVEDEV` — average of absolute deviations from the mean.
///
/// `AVEDEV(values)` = `(1/n) * Σ|xi - mean|`
///
/// Skips text, booleans, and empty cells. Propagates errors. Returns
/// `#DIV/0!` if no numeric values remain, `#NUM!` if the result is
/// non-finite.
///
/// # Errors
///
/// Returns `Err(CellError::Div0)` if no numeric values exist.
/// Returns `Err(CellError::Num)` if the result is NaN or infinite.
/// Returns `Err(CellError)` if any value is an error.
///
/// # Examples
///
/// ```
/// use xlstream_eval::builtins::statistical::builtin_avedev;
/// use xlstream_core::Value;
/// let vals = [Value::Number(2.0), Value::Number(4.0), Value::Number(8.0), Value::Number(16.0)];
/// assert_eq!(builtin_avedev(&vals), Ok(4.5));
/// ```
pub fn builtin_avedev(values: &[Value]) -> Result<f64, CellError> {
    let nums = collect_numerics(values)?;
    if nums.is_empty() {
        return Err(CellError::Div0);
    }
    #[allow(clippy::cast_precision_loss)]
    let n = nums.len() as f64;
    let mean = nums.iter().sum::<f64>() / n;
    let dev_sum: f64 = nums.iter().map(|x| (x - mean).abs()).sum();
    finite_or_num(dev_sum / n)
}

/// Normalize -0.0 to +0.0, then return `f64::to_bits()`.
fn canonical_bits(v: f64) -> u64 {
    if v == 0.0 {
        0.0_f64.to_bits()
    } else {
        v.to_bits()
    }
}

/// `MODE.SNGL` — most frequently occurring value.
///
/// Returns the value that appears most often. Ties are broken by first
/// occurrence (the value that appears earliest wins). Returns `#N/A` if
/// no value repeats or if no numeric values exist.
///
/// Text, booleans, and empty cells are skipped. Errors propagate.
/// Float comparison uses exact bit equality (after -0.0 → +0.0
/// normalization), matching Excel semantics.
///
/// # Errors
///
/// Returns `Err(CellError::Na)` if no numeric values exist, or if all
/// values are unique (no repeats).
/// Returns `Err(CellError)` if any input value is an error.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::mode_sngl;
/// let vals = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0),
///             Value::Number(3.0), Value::Number(4.0)];
/// assert_eq!(mode_sngl(&vals).unwrap(), 3.0);
/// ```
pub fn mode_sngl(values: &[Value]) -> Result<f64, CellError> {
    let nums = collect_numerics(values)?;
    if nums.is_empty() {
        return Err(CellError::Na);
    }

    let mut freq: HashMap<u64, (usize, usize)> = HashMap::with_capacity(nums.len());
    for (pos, &v) in nums.iter().enumerate() {
        let bits = canonical_bits(v);
        freq.entry(bits).and_modify(|(count, _)| *count += 1).or_insert((1, pos));
    }

    let (_, (max_count, _)) =
        freq.iter().max_by_key(|&(_, &(count, _))| count).ok_or(CellError::Na)?;
    if *max_count < 2 {
        return Err(CellError::Na);
    }

    let (_, (_, first_pos)) = freq
        .iter()
        .filter(|&(_, &(count, _))| count == *max_count)
        .min_by_key(|&(_, &(_, pos))| pos)
        .ok_or(CellError::Na)?;

    finite_or_num(nums[*first_pos])
}

/// Collect numerics, reject non-finite values, sort ascending.
///
/// Shared by percentile and quartile functions. Returns `#NUM!` if
/// any value is NaN or infinite, or if no numeric values remain.
fn sorted_numerics(values: &[Value]) -> Result<Vec<f64>, CellError> {
    let mut nums = collect_numerics(values)?;
    if nums.is_empty() || nums.iter().any(|x| !x.is_finite()) {
        return Err(CellError::Num);
    }
    nums.sort_by(f64::total_cmp);
    Ok(nums)
}

/// `PERCENTILE.INC` — inclusive percentile via linear interpolation.
///
/// `k` must be in \[0, 1\]. Collects numerics, sorts, interpolates
/// using Excel's method: `rank = k * (n - 1)`.
///
/// # Errors
///
/// Returns `Err(CellError::Num)` if `k` is out of range, no numeric
/// values exist, or any input is NaN/infinite.
/// Returns `Err(CellError)` if any value is an error.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::percentile_inc;
/// let vals = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
/// assert!((percentile_inc(&vals, 0.5).unwrap() - 2.0).abs() < 1e-9);
/// ```
pub fn percentile_inc(values: &[Value], k: f64) -> Result<f64, CellError> {
    if !(0.0..=1.0).contains(&k) {
        return Err(CellError::Num);
    }
    let nums = sorted_numerics(values)?;
    #[allow(clippy::cast_precision_loss)]
    let rank = k * (nums.len() - 1) as f64;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let lo = rank.floor() as usize;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let hi = rank.ceil() as usize;
    let frac = rank - rank.floor();
    Ok(nums[lo] + frac * (nums[hi] - nums[lo]))
}

/// `PERCENTILE.EXC` — exclusive percentile via linear interpolation.
///
/// `k` must be in (0, 1). Uses 1-based ranking: `rank = k * (n + 1)`.
/// Rank must fall in \[1, n\] after computation.
///
/// # Errors
///
/// Returns `Err(CellError::Num)` if `k` is out of range, rank falls
/// outside \[1, n\], no numeric values exist, or any input is
/// NaN/infinite.
/// Returns `Err(CellError)` if any value is an error.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::percentile_exc;
/// let vals = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0),
///             Value::Number(4.0), Value::Number(5.0)];
/// assert!((percentile_exc(&vals, 0.5).unwrap() - 3.0).abs() < 1e-9);
/// ```
pub fn percentile_exc(values: &[Value], k: f64) -> Result<f64, CellError> {
    if k <= 0.0 || k >= 1.0 {
        return Err(CellError::Num);
    }
    let nums = sorted_numerics(values)?;
    let n = nums.len();
    #[allow(clippy::cast_precision_loss)]
    let nf = n as f64;
    let lower = 1.0 / (nf + 1.0);
    let upper = nf / (nf + 1.0);
    if lower >= upper || k < lower || k > upper {
        return Err(CellError::Num);
    }
    let rank = k * (nf + 1.0);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let lo = (rank.floor() as usize).saturating_sub(1);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let hi = (rank.ceil() as usize).saturating_sub(1).min(n - 1);
    let frac = rank - rank.floor();
    Ok(nums[lo] + frac * (nums[hi] - nums[lo]))
}

/// `QUARTILE.INC` — inclusive quartile. `quart` must be in \[0, 4\].
///
/// Delegates to [`percentile_inc`] with `k = quart / 4`.
///
/// # Errors
///
/// Returns `Err(CellError::Num)` if `quart` is not in \[0, 4\] or data is empty.
/// Returns `Err(CellError)` if any value is an error.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::quartile_inc;
/// let vals = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0),
///             Value::Number(4.0), Value::Number(5.0)];
/// assert!((quartile_inc(&vals, 2).unwrap() - 3.0).abs() < 1e-9);
/// ```
pub fn quartile_inc(values: &[Value], quart: i32) -> Result<f64, CellError> {
    if !(0..=4).contains(&quart) {
        return Err(CellError::Num);
    }
    let k = f64::from(quart) / 4.0;
    percentile_inc(values, k)
}

/// `QUARTILE.EXC` — exclusive quartile. `quart` must be in \[1, 3\].
///
/// Delegates to [`percentile_exc`] with `k = quart / 4`.
///
/// # Errors
///
/// Returns `Err(CellError::Num)` if `quart` is not in \[1, 3\] or data
/// is insufficient for exclusive percentile.
/// Returns `Err(CellError)` if any value is an error.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::quartile_exc;
/// let vals = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0),
///             Value::Number(4.0), Value::Number(5.0)];
/// assert!((quartile_exc(&vals, 2).unwrap() - 3.0).abs() < 1e-9);
/// ```
pub fn quartile_exc(values: &[Value], quart: i32) -> Result<f64, CellError> {
    if !(1..=3).contains(&quart) {
        return Err(CellError::Num);
    }
    let k = f64::from(quart) / 4.0;
    percentile_exc(values, k)
}

/// Return the k-th value from a sorted data set.
///
/// Reuses [`sorted_numerics`] (collect, reject non-finite, sort
/// ascending), validates k (1..=n), and indexes into the result.
/// If `descending`, reverses before indexing.
fn kth_value(values: &[Value], k_val: &Value, descending: bool) -> Result<f64, CellError> {
    let mut nums = sorted_numerics(values)?;

    let k_f64 = xlstream_core::coerce::to_number(k_val)?;
    if !k_f64.is_finite() || k_f64 < 1.0 {
        return Err(CellError::Num);
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let k = k_f64 as usize;
    if k > nums.len() {
        return Err(CellError::Num);
    }

    if descending {
        nums.reverse();
    }
    Ok(nums[k - 1])
}

/// `LARGE` — k-th largest value from a data set.
///
/// Skips text, booleans, and empty cells. Propagates errors.
/// k=1 returns the maximum.
///
/// # Errors
///
/// Returns `Err(CellError::Num)` if k < 1, k > n, or no numeric
/// values exist.
/// Returns `Err(CellError)` if any value is an error or k cannot
/// be coerced.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::large;
/// let vals = [Value::Number(3.0), Value::Number(1.0), Value::Number(5.0)];
/// assert_eq!(large(&vals, &Value::Number(1.0)), Ok(5.0));
/// ```
pub fn large(values: &[Value], k_val: &Value) -> Result<f64, CellError> {
    kth_value(values, k_val, true)
}

/// `SMALL` — k-th smallest value from a data set.
///
/// Skips text, booleans, and empty cells. Propagates errors.
/// k=1 returns the minimum.
///
/// # Errors
///
/// Returns `Err(CellError::Num)` if k < 1, k > n, or no numeric
/// values exist.
/// Returns `Err(CellError)` if any value is an error or k cannot
/// be coerced.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::small;
/// let vals = [Value::Number(3.0), Value::Number(1.0), Value::Number(5.0)];
/// assert_eq!(small(&vals, &Value::Number(1.0)), Ok(1.0));
/// ```
pub fn small(values: &[Value], k_val: &Value) -> Result<f64, CellError> {
    kth_value(values, k_val, false)
}

/// `RANK.EQ` — rank of a number in a list (ties get the top rank).
///
/// `ascending`: if `true`, smallest is rank 1; if `false`, largest is rank 1.
/// Uses exact f64 equality for lookup — no epsilon tolerance.
///
/// # Errors
///
/// Returns `Err(CellError::Na)` if `number` is not found in `nums`.
///
/// # Examples
///
/// ```
/// use xlstream_eval::builtins::statistical::rank_eq;
/// assert_eq!(rank_eq(3.0, &[5.0, 3.0, 1.0], false).unwrap(), 2.0);
/// ```
pub fn rank_eq(number: f64, nums: &[f64], ascending: bool) -> Result<f64, CellError> {
    if !nums.contains(&number) {
        return Err(CellError::Na);
    }
    #[allow(clippy::cast_precision_loss)]
    let rank = if ascending {
        nums.iter().filter(|&&x| x < number).count() as f64 + 1.0
    } else {
        nums.iter().filter(|&&x| x > number).count() as f64 + 1.0
    };
    Ok(rank)
}

/// `RANK.AVG` — rank of a number in a list (ties get the average rank).
///
/// `ascending`: if `true`, smallest is rank 1; if `false`, largest is rank 1.
/// Uses exact f64 equality for lookup — no epsilon tolerance.
///
/// # Errors
///
/// Returns `Err(CellError::Na)` if `number` is not found in `nums`.
///
/// # Examples
///
/// ```
/// use xlstream_eval::builtins::statistical::rank_avg;
/// assert_eq!(rank_avg(3.0, &[5.0, 3.0, 3.0, 1.0], false).unwrap(), 2.5);
/// ```
pub fn rank_avg(number: f64, nums: &[f64], ascending: bool) -> Result<f64, CellError> {
    let eq_rank = rank_eq(number, nums, ascending)?;
    #[allow(clippy::cast_precision_loss, clippy::float_cmp)]
    let dup_count = nums.iter().filter(|&&x| x == number).count() as f64;
    Ok(eq_rank + (dup_count - 1.0) / 2.0)
}

/// `EXPON.DIST` — exponential distribution (pure math).
///
/// Returns the PDF or CDF of the exponential distribution.
///
/// - **CDF** (`cumulative = true`): `1 − e^(−λx)`
/// - **PDF** (`cumulative = false`): `λ · e^(−λx)`
///
/// # Errors
///
/// Returns `Err(CellError::Num)` if `x < 0`, `lambda ≤ 0`, any input
/// is non-finite, or the result is non-finite.
///
/// # Examples
///
/// ```
/// use xlstream_eval::builtins::statistical::expon_dist;
/// let result = expon_dist(1.0, 1.5, true).unwrap();
/// assert!((result - 0.776_869_839_851_570_2).abs() < 1e-9);
/// ```
pub fn expon_dist(x: f64, lambda: f64, cumulative: bool) -> Result<f64, CellError> {
    if !x.is_finite() || !lambda.is_finite() {
        return Err(CellError::Num);
    }
    if x < 0.0 || lambda <= 0.0 {
        return Err(CellError::Num);
    }

    let result = if cumulative { 1.0 - (-lambda * x).exp() } else { lambda * (-lambda * x).exp() };

    finite_or_num(result)
}

/// `POISSON.DIST(x, mean, cumulative)` — Poisson distribution.
///
/// Returns PMF `P(X=k)` when `cumulative` is false, CDF `P(X≤k)` when true.
/// Uses log-space arithmetic via `ln_gamma` to avoid overflow.
///
/// `x` is truncated to a non-negative integer. `mean` must be ≥ 0.
/// When `mean` = 0: returns 1.0 if `x` = 0, 0.0 otherwise (degenerate).
///
/// # Errors
///
/// Returns `Err(CellError::Value)` if arg count ≠ 3.
/// Returns `Err(CellError::Num)` if `x` < 0, `mean` < 0, or either is
/// non-finite (NaN/Infinity).
/// Returns `Err(CellError)` if any input is an error.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::builtin_poisson_dist;
/// let args = [Value::Number(3.0), Value::Number(5.0), Value::Bool(false)];
/// let result = builtin_poisson_dist(&args).unwrap();
/// assert!((result - 0.140374).abs() < 1e-5);
/// ```
pub fn builtin_poisson_dist(args: &[Value]) -> Result<f64, CellError> {
    if args.len() != 3 {
        return Err(CellError::Value);
    }
    let x_raw = coerce::to_number(&args[0])?;
    let mean = coerce::to_number(&args[1])?;
    let cumulative = coerce::to_bool(&args[2])?;

    if !x_raw.is_finite() || !mean.is_finite() {
        return Err(CellError::Num);
    }

    #[allow(clippy::cast_possible_truncation)]
    let k = x_raw.trunc() as i64;
    if k < 0 || mean < 0.0 {
        return Err(CellError::Num);
    }

    #[allow(clippy::cast_precision_loss)]
    let kf = k as f64;

    if mean == 0.0 {
        return if k == 0 { Ok(1.0) } else { Ok(0.0) };
    }

    if cumulative {
        poisson_cdf(kf, mean)
    } else {
        poisson_pmf(kf, mean)
    }
}

/// PMF: `P(X=k) = exp(−λ + k·ln(λ) − ln_gamma(k+1))`
fn poisson_pmf(k: f64, lambda: f64) -> Result<f64, CellError> {
    let ln_pmf = -lambda + k * lambda.ln() - ln_gamma(k + 1.0);
    finite_or_num(ln_pmf.exp())
}

/// CDF: `P(X≤k) = Σ PMF(i, λ)` for i in 0..=k
///
/// Caps k at [`POISSON_CDF_MAX_K`] to prevent runaway loops on extreme
/// inputs. Exits early once the cumulative sum reaches 1.0 within f64
/// precision.
fn poisson_cdf(k: f64, lambda: f64) -> Result<f64, CellError> {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let ki = (k as u64).min(POISSON_CDF_MAX_K);
    let mut sum = 0.0_f64;
    for i in 0..=ki {
        #[allow(clippy::cast_precision_loss)]
        let fi = i as f64;
        let ln_pmf = -lambda + fi * lambda.ln() - ln_gamma(fi + 1.0);
        sum += ln_pmf.exp();
        if sum >= 1.0 {
            return Ok(1.0);
        }
    }
    finite_or_num(sum)
}

const POISSON_CDF_MAX_K: u64 = 1_000_000;

fn num_arg(args: &[Value], idx: usize) -> Result<f64, Value> {
    coerce::to_number(args.get(idx).unwrap_or(&Value::Empty)).map_err(Value::Error)
}

/// `T.DIST(x, deg_freedom, cumulative)` — Student's t-distribution.
///
/// When `cumulative` is TRUE, returns the left-tail CDF P(T <= x).
/// When FALSE, returns the PDF (probability density).
/// Non-integer `deg_freedom` is truncated to integer (Excel behavior).
///
/// # Errors
///
/// Returns `#VALUE!` for wrong arg count or non-numeric args.
/// Returns `#NUM!` for df < 1.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::builtin_t_dist;
/// let args = [Value::Number(1.0), Value::Number(10.0), Value::Bool(true)];
/// match builtin_t_dist(&args) {
///     Value::Number(n) => assert!((n - 0.82955).abs() < 1e-4),
///     other => panic!("expected Number, got {other:?}"),
/// }
/// ```
#[must_use]
pub fn builtin_t_dist(args: &[Value]) -> Value {
    if args.len() != 3 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let df_raw = match num_arg(args, 1) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let cumulative = match &args[2] {
        Value::Error(e) => return Value::Error(*e),
        Value::Bool(b) => *b,
        Value::Number(n) => *n != 0.0,
        Value::Integer(i) => *i != 0,
        Value::Empty => false,
        _ => return Value::Error(CellError::Value),
    };

    let df = df_raw.trunc();
    if df < 1.0 {
        return Value::Error(CellError::Num);
    }

    let result = if cumulative { specfn::t_dist_cdf(x, df) } else { specfn::t_dist_pdf(x, df) };

    if result.is_finite() {
        Value::Number(result)
    } else {
        Value::Error(CellError::Num)
    }
}

/// `T.DIST.RT(x, deg_freedom)` — right-tail CDF of Student's t.
///
/// Returns P(T >= x) = 1 - T.DIST(x, df, TRUE).
///
/// # Errors
///
/// Returns `#VALUE!` for wrong arg count. Returns `#NUM!` for df < 1.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::builtin_t_dist_rt;
/// let args = [Value::Number(1.0), Value::Number(10.0)];
/// match builtin_t_dist_rt(&args) {
///     Value::Number(n) => assert!((n - 0.17045).abs() < 1e-4),
///     other => panic!("expected Number, got {other:?}"),
/// }
/// ```
#[must_use]
pub fn builtin_t_dist_rt(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let df_raw = match num_arg(args, 1) {
        Ok(v) => v,
        Err(e) => return e,
    };

    let df = df_raw.trunc();
    if df < 1.0 {
        return Value::Error(CellError::Num);
    }

    let result = 1.0 - specfn::t_dist_cdf(x, df);
    if result.is_finite() {
        Value::Number(result)
    } else {
        Value::Error(CellError::Num)
    }
}

/// `T.DIST.2T(x, deg_freedom)` — two-tail CDF of Student's t.
///
/// Returns P(|T| >= x). Requires x >= 0.
///
/// # Errors
///
/// Returns `#VALUE!` for wrong arg count. Returns `#NUM!` for x < 0 or df < 1.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::builtin_t_dist_2t;
/// let args = [Value::Number(1.0), Value::Number(10.0)];
/// match builtin_t_dist_2t(&args) {
///     Value::Number(n) => assert!((n - 0.34090).abs() < 1e-4),
///     other => panic!("expected Number, got {other:?}"),
/// }
/// ```
#[must_use]
pub fn builtin_t_dist_2t(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let x = match num_arg(args, 0) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let df_raw = match num_arg(args, 1) {
        Ok(v) => v,
        Err(e) => return e,
    };

    if x < 0.0 {
        return Value::Error(CellError::Num);
    }

    let df = df_raw.trunc();
    if df < 1.0 {
        return Value::Error(CellError::Num);
    }

    let result = 2.0 * (1.0 - specfn::t_dist_cdf(x, df));
    if result.is_finite() {
        Value::Number(result)
    } else {
        Value::Error(CellError::Num)
    }
}

/// `T.INV(probability, deg_freedom)` — left-tail inverse of Student's t.
///
/// Returns the value t such that P(T <= t) = probability.
///
/// # Errors
///
/// Returns `#VALUE!` for wrong arg count. Returns `#NUM!` for p outside (0,1) or df < 1.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::builtin_t_inv;
/// let args = [Value::Number(0.5), Value::Number(10.0)];
/// match builtin_t_inv(&args) {
///     Value::Number(n) => assert!(n.abs() < 1e-10),
///     other => panic!("expected Number, got {other:?}"),
/// }
/// ```
#[must_use]
pub fn builtin_t_inv(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let p = match num_arg(args, 0) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let df_raw = match num_arg(args, 1) {
        Ok(v) => v,
        Err(e) => return e,
    };

    if p <= 0.0 || p >= 1.0 {
        return Value::Error(CellError::Num);
    }

    let df = df_raw.trunc();
    if df < 1.0 {
        return Value::Error(CellError::Num);
    }

    let result = specfn::t_inv(p, df);
    if result.is_finite() {
        Value::Number(result)
    } else {
        Value::Error(CellError::Num)
    }
}

/// `T.INV.2T(probability, deg_freedom)` — two-tail inverse of Student's t.
///
/// Returns the positive value t such that P(|T| >= t) = probability.
/// Equivalent to `T.INV(1 - probability/2, df)`.
///
/// # Errors
///
/// Returns `#VALUE!` for wrong arg count. Returns `#NUM!` for p outside (0,1] or df < 1.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::statistical::builtin_t_inv_2t;
/// let args = [Value::Number(0.05), Value::Number(10.0)];
/// match builtin_t_inv_2t(&args) {
///     Value::Number(n) => assert!((n - 2.22814).abs() < 1e-4),
///     other => panic!("expected Number, got {other:?}"),
/// }
/// ```
#[must_use]
pub fn builtin_t_inv_2t(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value::Error(CellError::Value);
    }
    let p = match num_arg(args, 0) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let df_raw = match num_arg(args, 1) {
        Ok(v) => v,
        Err(e) => return e,
    };

    if p <= 0.0 || p > 1.0 {
        return Value::Error(CellError::Num);
    }

    let df = df_raw.trunc();
    if df < 1.0 {
        return Value::Error(CellError::Num);
    }

    let left_p = 1.0 - p / 2.0;
    let result = specfn::t_inv(left_p, df);
    if result.is_finite() {
        Value::Number(result)
    } else {
        Value::Error(CellError::Num)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use xlstream_core::{CellError, Value};

    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < 1e-9, "expected {expected}, got {actual}");
    }

    // ===== collect_numerics =====

    #[test]
    fn collect_numerics_empty_input() {
        assert_eq!(collect_numerics(&[]).unwrap(), Vec::<f64>::new());
    }

    #[test]
    fn collect_numerics_all_text_returns_empty() {
        let vals = [Value::Text("a".into()), Value::Text("b".into())];
        assert_eq!(collect_numerics(&vals).unwrap(), Vec::<f64>::new());
    }

    #[test]
    fn collect_numerics_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Na), Value::Number(3.0)];
        assert_eq!(collect_numerics(&vals).unwrap_err(), CellError::Na);
    }

    #[test]
    fn collect_numerics_mixed_types() {
        let vals = [
            Value::Number(1.0),
            Value::Text("x".into()),
            Value::Integer(2),
            Value::Bool(true),
            Value::Empty,
            Value::Number(3.0),
        ];
        assert_eq!(collect_numerics(&vals).unwrap(), vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn collect_numerics_includes_date_serial() {
        let date = xlstream_core::ExcelDate { serial: 45000.0 };
        let vals = [Value::Number(1.0), Value::Date(date)];
        assert_eq!(collect_numerics(&vals).unwrap(), vec![1.0, 45000.0]);
    }

    // ===== VAR.S =====

    #[test]
    fn var_s_four_values() {
        let vals = [Value::Number(2.0), Value::Number(4.0), Value::Number(6.0), Value::Number(8.0)];
        assert_close(var_s(&vals).unwrap(), 6.666_666_666_666_667);
    }

    #[test]
    fn var_s_two_values() {
        let vals = [Value::Number(3.0), Value::Number(7.0)];
        assert_close(var_s(&vals).unwrap(), 8.0);
    }

    #[test]
    fn var_s_all_same_returns_zero() {
        let vals = [Value::Number(5.0), Value::Number(5.0), Value::Number(5.0)];
        assert_close(var_s(&vals).unwrap(), 0.0);
    }

    #[test]
    fn var_s_single_value_returns_div0() {
        let vals = [Value::Number(5.0)];
        assert_eq!(var_s(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn var_s_empty_returns_div0() {
        assert_eq!(var_s(&[]).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn var_s_all_text_returns_div0() {
        let vals = [Value::Text("a".into()), Value::Text("b".into())];
        assert_eq!(var_s(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn var_s_skips_text_and_bool() {
        let vals =
            [Value::Number(1.0), Value::Text("text".into()), Value::Number(3.0), Value::Bool(true)];
        assert_close(var_s(&vals).unwrap(), 2.0);
    }

    #[test]
    fn var_s_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Na), Value::Number(3.0)];
        assert_eq!(var_s(&vals).unwrap_err(), CellError::Na);
    }

    #[test]
    fn var_s_negative_numbers() {
        let vals = [Value::Number(-2.0), Value::Number(-4.0), Value::Number(-6.0)];
        assert_close(var_s(&vals).unwrap(), 4.0);
    }

    // ===== VAR.P =====

    #[test]
    fn var_p_four_values() {
        let vals = [Value::Number(2.0), Value::Number(4.0), Value::Number(6.0), Value::Number(8.0)];
        assert_close(var_p(&vals).unwrap(), 5.0);
    }

    #[test]
    fn var_p_single_value_returns_zero() {
        let vals = [Value::Number(5.0)];
        assert_close(var_p(&vals).unwrap(), 0.0);
    }

    #[test]
    fn var_p_empty_returns_div0() {
        assert_eq!(var_p(&[]).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn var_p_all_same_returns_zero() {
        let vals = [Value::Number(7.0), Value::Number(7.0), Value::Number(7.0)];
        assert_close(var_p(&vals).unwrap(), 0.0);
    }

    #[test]
    fn var_p_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Div0)];
        assert_eq!(var_p(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn var_p_skips_text_and_bool() {
        let vals =
            [Value::Number(1.0), Value::Text("x".into()), Value::Number(3.0), Value::Bool(false)];
        assert_close(var_p(&vals).unwrap(), 1.0);
    }

    // ===== STDEV.S =====

    #[test]
    fn stdev_s_four_values() {
        let vals = [Value::Number(2.0), Value::Number(4.0), Value::Number(6.0), Value::Number(8.0)];
        assert_close(stdev_s(&vals).unwrap(), 2.581_988_897_471_611);
    }

    #[test]
    fn stdev_s_two_values() {
        let vals = [Value::Number(3.0), Value::Number(7.0)];
        assert_close(stdev_s(&vals).unwrap(), 2.828_427_124_746_190_3);
    }

    #[test]
    fn stdev_s_all_same_returns_zero() {
        let vals = [Value::Number(5.0), Value::Number(5.0), Value::Number(5.0)];
        assert_close(stdev_s(&vals).unwrap(), 0.0);
    }

    #[test]
    fn stdev_s_single_value_returns_div0() {
        let vals = [Value::Number(5.0)];
        assert_eq!(stdev_s(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn stdev_s_empty_returns_div0() {
        assert_eq!(stdev_s(&[]).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn stdev_s_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Na)];
        assert_eq!(stdev_s(&vals).unwrap_err(), CellError::Na);
    }

    #[test]
    fn stdev_s_negative_numbers() {
        let vals = [Value::Number(-2.0), Value::Number(-4.0), Value::Number(-6.0)];
        assert_close(stdev_s(&vals).unwrap(), 2.0);
    }

    // ===== STDEV.P =====

    #[test]
    fn stdev_p_four_values() {
        let vals = [Value::Number(2.0), Value::Number(4.0), Value::Number(6.0), Value::Number(8.0)];
        assert_close(stdev_p(&vals).unwrap(), 2.236_067_977_499_79);
    }

    #[test]
    fn stdev_p_single_value_returns_zero() {
        let vals = [Value::Number(5.0)];
        assert_close(stdev_p(&vals).unwrap(), 0.0);
    }

    #[test]
    fn stdev_p_empty_returns_div0() {
        assert_eq!(stdev_p(&[]).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn stdev_p_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Ref)];
        assert_eq!(stdev_p(&vals).unwrap_err(), CellError::Ref);
    }

    // ===== Numerical stability =====

    #[test]
    fn var_p_large_numbers_stable() {
        let vals = [Value::Number(1e10), Value::Number(1e10 + 1.0)];
        assert_close(var_p(&vals).unwrap(), 0.25);
    }

    #[test]
    fn var_s_overflow_returns_num() {
        let vals = [Value::Number(f64::MAX), Value::Number(f64::MAX)];
        assert_eq!(var_s(&vals).unwrap_err(), CellError::Num);
    }

    #[test]
    fn var_p_infinity_returns_num() {
        let vals = [Value::Number(f64::INFINITY), Value::Number(1.0)];
        assert_eq!(var_p(&vals).unwrap_err(), CellError::Num);
    }

    // ===== SKEW =====

    #[test]
    fn skew_symmetric_is_zero() {
        let vals: Vec<Value> =
            [1.0, 2.0, 3.0, 4.0, 5.0].iter().map(|&n| Value::Number(n)).collect();
        assert_close(skew(&vals).unwrap(), 0.0);
    }

    #[test]
    fn skew_right_skewed_is_positive() {
        let vals: Vec<Value> =
            [1.0, 2.0, 3.0, 4.0, 100.0].iter().map(|&n| Value::Number(n)).collect();
        assert!(skew(&vals).unwrap() > 0.0);
    }

    #[test]
    fn skew_left_skewed_is_negative() {
        let vals: Vec<Value> =
            [1.0, 97.0, 98.0, 99.0, 100.0].iter().map(|&n| Value::Number(n)).collect();
        assert!(skew(&vals).unwrap() < 0.0);
    }

    #[test]
    fn skew_minimum_n_three_symmetric_is_zero() {
        let vals: Vec<Value> = [10.0, 20.0, 30.0].iter().map(|&n| Value::Number(n)).collect();
        assert_close(skew(&vals).unwrap(), 0.0);
    }

    #[test]
    fn skew_below_minimum_returns_div0() {
        let vals = [Value::Number(1.0), Value::Number(2.0)];
        assert_eq!(skew(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn skew_single_value_returns_div0() {
        let vals = [Value::Number(1.0)];
        assert_eq!(skew(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn skew_empty_returns_div0() {
        assert_eq!(skew(&[]).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn skew_all_same_returns_div0() {
        let vals: Vec<Value> = [5.0, 5.0, 5.0].iter().map(|&n| Value::Number(n)).collect();
        assert_eq!(skew(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn skew_skips_text() {
        let vals = [
            Value::Number(1.0),
            Value::Text("text".into()),
            Value::Number(3.0),
            Value::Number(5.0),
        ];
        assert_close(skew(&vals).unwrap(), 0.0);
    }

    #[test]
    fn skew_propagates_error() {
        let vals = [
            Value::Number(1.0),
            Value::Error(CellError::Na),
            Value::Number(3.0),
            Value::Number(5.0),
        ];
        assert_eq!(skew(&vals).unwrap_err(), CellError::Na);
    }

    // ===== SKEW.P =====

    #[test]
    fn skew_p_symmetric_is_zero() {
        let vals: Vec<Value> =
            [1.0, 2.0, 3.0, 4.0, 5.0].iter().map(|&n| Value::Number(n)).collect();
        assert_close(skew_p(&vals).unwrap(), 0.0);
    }

    #[test]
    fn skew_p_right_skewed_is_positive() {
        let vals: Vec<Value> =
            [1.0, 2.0, 3.0, 4.0, 100.0].iter().map(|&n| Value::Number(n)).collect();
        assert!(skew_p(&vals).unwrap() > 0.0);
    }

    #[test]
    fn skew_p_two_symmetric_values_is_zero() {
        let vals = [Value::Number(1.0), Value::Number(3.0)];
        assert_close(skew_p(&vals).unwrap(), 0.0);
    }

    #[test]
    fn skew_p_single_value_div0() {
        let vals = [Value::Number(5.0)];
        assert_eq!(skew_p(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn skew_p_empty_returns_div0() {
        assert_eq!(skew_p(&[]).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn skew_p_all_same_returns_div0() {
        let vals: Vec<Value> = [5.0, 5.0, 5.0].iter().map(|&n| Value::Number(n)).collect();
        assert_eq!(skew_p(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn skew_p_propagates_error() {
        let vals = [
            Value::Number(1.0),
            Value::Error(CellError::Na),
            Value::Number(3.0),
            Value::Number(5.0),
        ];
        assert_eq!(skew_p(&vals).unwrap_err(), CellError::Na);
    }

    #[test]
    fn skew_p_skips_text_and_bool() {
        let vals = [
            Value::Number(1.0),
            Value::Text("text".into()),
            Value::Number(3.0),
            Value::Bool(true),
            Value::Number(5.0),
        ];
        assert_close(skew_p(&vals).unwrap(), 0.0);
    }

    // ===== KURT =====

    #[test]
    fn kurt_uniform_is_negative_1_2() {
        let vals: Vec<Value> =
            [1.0, 2.0, 3.0, 4.0, 5.0].iter().map(|&n| Value::Number(n)).collect();
        assert_close(kurt(&vals).unwrap(), -1.2);
    }

    #[test]
    fn kurt_right_skewed_is_positive() {
        let vals: Vec<Value> =
            [1.0, 2.0, 3.0, 4.0, 100.0].iter().map(|&n| Value::Number(n)).collect();
        assert!(kurt(&vals).unwrap() > 0.0);
    }

    #[test]
    fn kurt_uniform_ten_values() {
        let vals: Vec<Value> = [2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0, 20.0]
            .iter()
            .map(|&n| Value::Number(n))
            .collect();
        assert_close(kurt(&vals).unwrap(), -1.2);
    }

    #[test]
    fn kurt_minimum_n_four_uniform_is_negative_1_2() {
        let vals: Vec<Value> = [1.0, 2.0, 3.0, 4.0].iter().map(|&n| Value::Number(n)).collect();
        assert_close(kurt(&vals).unwrap(), -1.2);
    }

    #[test]
    fn kurt_below_minimum_returns_div0() {
        let vals: Vec<Value> = [1.0, 2.0, 3.0].iter().map(|&n| Value::Number(n)).collect();
        assert_eq!(kurt(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn kurt_empty_returns_div0() {
        assert_eq!(kurt(&[]).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn kurt_all_same_returns_div0() {
        let vals: Vec<Value> = [5.0, 5.0, 5.0, 5.0].iter().map(|&n| Value::Number(n)).collect();
        assert_eq!(kurt(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn kurt_skips_bool() {
        let vals = [
            Value::Number(1.0),
            Value::Bool(true),
            Value::Number(3.0),
            Value::Number(5.0),
            Value::Number(7.0),
        ];
        assert_close(kurt(&vals).unwrap(), -1.2);
    }

    #[test]
    fn kurt_propagates_error() {
        let vals = [
            Value::Number(1.0),
            Value::Error(CellError::Na),
            Value::Number(3.0),
            Value::Number(5.0),
            Value::Number(7.0),
        ];
        assert_eq!(kurt(&vals).unwrap_err(), CellError::Na);
    }

    // ===== Overflow / NaN guards =====

    #[test]
    fn skew_infinity_returns_num() {
        let vals = [Value::Number(f64::INFINITY), Value::Number(1.0), Value::Number(2.0)];
        assert_eq!(skew(&vals).unwrap_err(), CellError::Num);
    }

    #[test]
    fn skew_p_overflow_returns_num() {
        let vals = [Value::Number(f64::MAX), Value::Number(f64::MAX), Value::Number(1.0)];
        assert_eq!(skew_p(&vals).unwrap_err(), CellError::Num);
    }

    #[test]
    fn kurt_infinity_returns_num() {
        let vals = [
            Value::Number(f64::INFINITY),
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ];
        assert_eq!(kurt(&vals).unwrap_err(), CellError::Num);
    }

    #[test]
    fn kurt_max_values_returns_num() {
        let vals = [
            Value::Number(f64::MAX),
            Value::Number(f64::MAX),
            Value::Number(1.0),
            Value::Number(2.0),
        ];
        assert_eq!(kurt(&vals).unwrap_err(), CellError::Num);
    }

    // ===== mean_and_variance =====

    #[test]
    fn mean_and_variance_sample() {
        let nums = [2.0, 4.0, 6.0, 8.0];
        let (mean, var) = mean_and_variance(&nums, 1).unwrap();
        assert_close(mean, 5.0);
        assert_close(var, 6.666_666_666_666_667);
    }

    #[test]
    fn mean_and_variance_population() {
        let nums = [2.0, 4.0, 6.0, 8.0];
        let (mean, var) = mean_and_variance(&nums, 0).unwrap();
        assert_close(mean, 5.0);
        assert_close(var, 5.0);
    }

    #[test]
    fn mean_and_variance_too_few_returns_none() {
        assert!(mean_and_variance(&[5.0], 1).is_none());
        assert!(mean_and_variance(&[], 0).is_none());
    }

    // ===== AVEDEV =====

    #[test]
    fn avedev_four_positive_values() {
        let vals =
            [Value::Number(2.0), Value::Number(4.0), Value::Number(8.0), Value::Number(16.0)];
        assert_eq!(builtin_avedev(&vals).unwrap(), 4.5);
    }

    #[test]
    fn avedev_sequential_values() {
        let vals = [
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
            Value::Number(4.0),
            Value::Number(5.0),
        ];
        assert_eq!(builtin_avedev(&vals).unwrap(), 1.2);
    }

    #[test]
    fn avedev_negative_values() {
        let vals = [
            Value::Number(-2.0),
            Value::Number(-1.0),
            Value::Number(0.0),
            Value::Number(1.0),
            Value::Number(2.0),
        ];
        assert_eq!(builtin_avedev(&vals).unwrap(), 1.2);
    }

    #[test]
    fn avedev_single_value_returns_zero() {
        let vals = [Value::Number(5.0)];
        assert_eq!(builtin_avedev(&vals).unwrap(), 0.0);
    }

    #[test]
    fn avedev_two_values() {
        let vals = [Value::Number(0.0), Value::Number(10.0)];
        assert_eq!(builtin_avedev(&vals).unwrap(), 5.0);
    }

    #[test]
    fn avedev_all_same_returns_zero() {
        let vals = [Value::Number(7.0), Value::Number(7.0), Value::Number(7.0)];
        assert_eq!(builtin_avedev(&vals).unwrap(), 0.0);
    }

    #[test]
    fn avedev_empty_returns_div0() {
        assert_eq!(builtin_avedev(&[]).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn avedev_all_text_returns_div0() {
        let vals = [Value::Text("a".into()), Value::Text("b".into())];
        assert_eq!(builtin_avedev(&vals).unwrap_err(), CellError::Div0);
    }

    #[test]
    fn avedev_skips_text() {
        let vals = [Value::Number(1.0), Value::Text("text".into()), Value::Number(3.0)];
        assert_eq!(builtin_avedev(&vals).unwrap(), 1.0);
    }

    #[test]
    fn avedev_skips_bool() {
        let vals = [Value::Number(1.0), Value::Bool(true), Value::Number(3.0)];
        assert_eq!(builtin_avedev(&vals).unwrap(), 1.0);
    }

    #[test]
    fn avedev_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Na), Value::Number(3.0)];
        assert_eq!(builtin_avedev(&vals).unwrap_err(), CellError::Na);
    }

    #[test]
    fn avedev_large_numbers() {
        let vals = [Value::Number(1e10), Value::Number(1e10 + 2.0)];
        assert_eq!(builtin_avedev(&vals).unwrap(), 1.0);
    }

    // ===== MODE.SNGL — happy path =====

    #[test]
    fn mode_sngl_single_mode() {
        let vals: Vec<Value> =
            [1.0, 2.0, 3.0, 3.0, 4.0].iter().map(|&n| Value::Number(n)).collect();
        assert_eq!(mode_sngl(&vals).unwrap(), 3.0);
    }

    #[test]
    fn mode_sngl_tie_returns_first_occurrence() {
        let vals: Vec<Value> =
            [1.0, 2.0, 2.0, 3.0, 3.0, 4.0].iter().map(|&n| Value::Number(n)).collect();
        assert_eq!(mode_sngl(&vals).unwrap(), 2.0);
    }

    #[test]
    fn mode_sngl_all_same() {
        let vals: Vec<Value> = [5.0, 5.0, 5.0].iter().map(|&n| Value::Number(n)).collect();
        assert_eq!(mode_sngl(&vals).unwrap(), 5.0);
    }

    #[test]
    fn mode_sngl_fractional() {
        let vals: Vec<Value> =
            [1.5, 1.5, 2.5, 2.5, 2.5].iter().map(|&n| Value::Number(n)).collect();
        assert_eq!(mode_sngl(&vals).unwrap(), 2.5);
    }

    // ===== MODE.SNGL — edge cases =====

    #[test]
    fn mode_sngl_all_unique_returns_na() {
        let vals: Vec<Value> = [1.0, 2.0, 3.0, 4.0].iter().map(|&n| Value::Number(n)).collect();
        assert_eq!(mode_sngl(&vals).unwrap_err(), CellError::Na);
    }

    #[test]
    fn mode_sngl_single_value_returns_na() {
        let vals = [Value::Number(5.0)];
        assert_eq!(mode_sngl(&vals).unwrap_err(), CellError::Na);
    }

    #[test]
    fn mode_sngl_two_identical_returns_value() {
        let vals = [Value::Number(5.0), Value::Number(5.0)];
        assert_eq!(mode_sngl(&vals).unwrap(), 5.0);
    }

    #[test]
    fn mode_sngl_empty_returns_na() {
        assert_eq!(mode_sngl(&[]).unwrap_err(), CellError::Na);
    }

    #[test]
    fn mode_sngl_all_text_returns_na() {
        let vals = [Value::Text("a".into()), Value::Text("b".into())];
        assert_eq!(mode_sngl(&vals).unwrap_err(), CellError::Na);
    }

    #[test]
    fn mode_sngl_negative_values() {
        let vals: Vec<Value> =
            [-3.0, -3.0, -1.0, -1.0, -1.0].iter().map(|&n| Value::Number(n)).collect();
        assert_eq!(mode_sngl(&vals).unwrap(), -1.0);
    }

    #[test]
    fn mode_sngl_zero_mode() {
        let vals: Vec<Value> = [0.0, 0.0, 1.0].iter().map(|&n| Value::Number(n)).collect();
        assert_eq!(mode_sngl(&vals).unwrap(), 0.0);
    }

    #[test]
    fn mode_sngl_large_count_wins() {
        let vals: Vec<Value> =
            [1.0, 1.0, 1.0, 2.0, 2.0, 3.0].iter().map(|&n| Value::Number(n)).collect();
        assert_eq!(mode_sngl(&vals).unwrap(), 1.0);
    }

    // ===== MODE.SNGL — type handling =====

    #[test]
    fn mode_sngl_skips_text() {
        let vals = [
            Value::Number(1.0),
            Value::Text("text".into()),
            Value::Number(1.0),
            Value::Number(3.0),
        ];
        assert_eq!(mode_sngl(&vals).unwrap(), 1.0);
    }

    #[test]
    fn mode_sngl_skips_bool() {
        let vals = [Value::Number(1.0), Value::Bool(true), Value::Number(1.0), Value::Number(3.0)];
        assert_eq!(mode_sngl(&vals).unwrap(), 1.0);
    }

    #[test]
    fn mode_sngl_propagates_error() {
        let vals = [
            Value::Number(1.0),
            Value::Error(CellError::Na),
            Value::Number(1.0),
            Value::Number(3.0),
        ];
        assert_eq!(mode_sngl(&vals).unwrap_err(), CellError::Na);
    }

    #[test]
    fn mode_sngl_propagates_non_na_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Div0), Value::Number(1.0)];
        assert_eq!(mode_sngl(&vals).unwrap_err(), CellError::Div0);
    }

    // ===== MODE.SNGL — float edge cases =====

    #[test]
    fn mode_sngl_positive_and_negative_zero_treated_equal() {
        let vals = [Value::Number(0.0), Value::Number(-0.0), Value::Number(1.0)];
        assert_eq!(mode_sngl(&vals).unwrap(), 0.0);
    }

    #[test]
    fn mode_sngl_infinity_mode_returns_num() {
        let vals = [Value::Number(f64::INFINITY), Value::Number(f64::INFINITY), Value::Number(1.0)];
        assert_eq!(mode_sngl(&vals).unwrap_err(), CellError::Num);
    }

    // ===== PERCENTILE.INC =====

    #[test]
    fn percentile_inc_quartiles_of_five() {
        let vals: Vec<Value> =
            [1.0, 2.0, 3.0, 4.0, 5.0].iter().map(|&n| Value::Number(n)).collect();
        assert_close(percentile_inc(&vals, 0.0).unwrap(), 1.0);
        assert_close(percentile_inc(&vals, 0.25).unwrap(), 2.0);
        assert_close(percentile_inc(&vals, 0.5).unwrap(), 3.0);
        assert_close(percentile_inc(&vals, 0.75).unwrap(), 4.0);
        assert_close(percentile_inc(&vals, 1.0).unwrap(), 5.0);
    }

    #[test]
    fn percentile_inc_interpolates_ten_values() {
        let vals: Vec<Value> = (1..=10).map(|n| Value::Number(f64::from(n))).collect();
        assert_close(percentile_inc(&vals, 0.25).unwrap(), 3.25);
        assert_close(percentile_inc(&vals, 0.75).unwrap(), 7.75);
    }

    #[test]
    fn percentile_inc_single_value() {
        let vals = [Value::Number(5.0)];
        assert_close(percentile_inc(&vals, 0.5).unwrap(), 5.0);
    }

    #[test]
    fn percentile_inc_two_values() {
        let vals = [Value::Number(1.0), Value::Number(3.0)];
        assert_close(percentile_inc(&vals, 0.5).unwrap(), 2.0);
    }

    #[test]
    fn percentile_inc_all_same() {
        let vals = [Value::Number(5.0), Value::Number(5.0), Value::Number(5.0)];
        assert_close(percentile_inc(&vals, 0.25).unwrap(), 5.0);
    }

    #[test]
    fn percentile_inc_k_out_of_range_returns_num() {
        let vals = [Value::Number(1.0), Value::Number(2.0)];
        assert_eq!(percentile_inc(&vals, 1.5).unwrap_err(), CellError::Num);
        assert_eq!(percentile_inc(&vals, -0.1).unwrap_err(), CellError::Num);
    }

    #[test]
    fn percentile_inc_empty_returns_num() {
        assert_eq!(percentile_inc(&[], 0.5).unwrap_err(), CellError::Num);
    }

    #[test]
    fn percentile_inc_all_text_returns_num() {
        let vals = [Value::Text("a".into()), Value::Text("b".into())];
        assert_eq!(percentile_inc(&vals, 0.5).unwrap_err(), CellError::Num);
    }

    #[test]
    fn percentile_inc_skips_text_and_bool() {
        let vals =
            [Value::Number(1.0), Value::Text("x".into()), Value::Number(3.0), Value::Bool(true)];
        assert_close(percentile_inc(&vals, 0.5).unwrap(), 2.0);
    }

    #[test]
    fn percentile_inc_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Na), Value::Number(3.0)];
        assert_eq!(percentile_inc(&vals, 0.5).unwrap_err(), CellError::Na);
    }

    #[test]
    fn percentile_inc_nan_returns_num() {
        let vals = [Value::Number(1.0), Value::Number(f64::NAN), Value::Number(3.0)];
        assert_eq!(percentile_inc(&vals, 0.5).unwrap_err(), CellError::Num);
    }

    #[test]
    fn percentile_inc_infinity_returns_num() {
        let vals = [Value::Number(1.0), Value::Number(f64::INFINITY)];
        assert_eq!(percentile_inc(&vals, 0.5).unwrap_err(), CellError::Num);
    }

    // ===== PERCENTILE.EXC =====

    #[test]
    fn percentile_exc_quartiles_of_five() {
        let vals: Vec<Value> =
            [1.0, 2.0, 3.0, 4.0, 5.0].iter().map(|&n| Value::Number(n)).collect();
        assert_close(percentile_exc(&vals, 0.25).unwrap(), 1.5);
        assert_close(percentile_exc(&vals, 0.5).unwrap(), 3.0);
        assert_close(percentile_exc(&vals, 0.75).unwrap(), 4.5);
    }

    #[test]
    fn percentile_exc_ten_values() {
        let vals: Vec<Value> = (1..=10).map(|n| Value::Number(f64::from(n))).collect();
        assert_close(percentile_exc(&vals, 0.25).unwrap(), 2.75);
        assert_close(percentile_exc(&vals, 0.5).unwrap(), 5.5);
        assert_close(percentile_exc(&vals, 0.75).unwrap(), 8.25);
    }

    #[test]
    fn percentile_exc_k_zero_returns_num() {
        let vals = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
        assert_eq!(percentile_exc(&vals, 0.0).unwrap_err(), CellError::Num);
    }

    #[test]
    fn percentile_exc_k_one_returns_num() {
        let vals = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
        assert_eq!(percentile_exc(&vals, 1.0).unwrap_err(), CellError::Num);
    }

    #[test]
    fn percentile_exc_single_value_returns_num() {
        let vals = [Value::Number(5.0)];
        assert_eq!(percentile_exc(&vals, 0.5).unwrap_err(), CellError::Num);
    }

    #[test]
    fn percentile_exc_empty_returns_num() {
        assert_eq!(percentile_exc(&[], 0.5).unwrap_err(), CellError::Num);
    }

    #[test]
    fn percentile_exc_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Na)];
        assert_eq!(percentile_exc(&vals, 0.5).unwrap_err(), CellError::Na);
    }

    // ===== QUARTILE.INC =====

    #[test]
    fn quartile_inc_all_five_quarts() {
        let vals: Vec<Value> =
            [1.0, 2.0, 3.0, 4.0, 5.0].iter().map(|&n| Value::Number(n)).collect();
        assert_close(quartile_inc(&vals, 0).unwrap(), 1.0);
        assert_close(quartile_inc(&vals, 1).unwrap(), 2.0);
        assert_close(quartile_inc(&vals, 2).unwrap(), 3.0);
        assert_close(quartile_inc(&vals, 3).unwrap(), 4.0);
        assert_close(quartile_inc(&vals, 4).unwrap(), 5.0);
    }

    #[test]
    fn quartile_inc_invalid_quart_returns_num() {
        let vals = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
        assert_eq!(quartile_inc(&vals, 5).unwrap_err(), CellError::Num);
        assert_eq!(quartile_inc(&vals, -1).unwrap_err(), CellError::Num);
    }

    #[test]
    fn quartile_inc_single_value() {
        let vals = [Value::Number(5.0)];
        assert_close(quartile_inc(&vals, 2).unwrap(), 5.0);
    }

    #[test]
    fn quartile_inc_empty_returns_num() {
        assert_eq!(quartile_inc(&[], 1).unwrap_err(), CellError::Num);
    }

    #[test]
    fn quartile_inc_all_same() {
        let vals = [Value::Number(7.0), Value::Number(7.0), Value::Number(7.0)];
        assert_close(quartile_inc(&vals, 3).unwrap(), 7.0);
    }

    #[test]
    fn quartile_inc_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Na), Value::Number(3.0)];
        assert_eq!(quartile_inc(&vals, 1).unwrap_err(), CellError::Na);
    }

    // ===== QUARTILE.EXC =====

    #[test]
    fn quartile_exc_three_quarts() {
        let vals: Vec<Value> =
            [1.0, 2.0, 3.0, 4.0, 5.0].iter().map(|&n| Value::Number(n)).collect();
        assert_close(quartile_exc(&vals, 1).unwrap(), 1.5);
        assert_close(quartile_exc(&vals, 2).unwrap(), 3.0);
        assert_close(quartile_exc(&vals, 3).unwrap(), 4.5);
    }

    #[test]
    fn quartile_exc_invalid_quart_returns_num() {
        let vals = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
        assert_eq!(quartile_exc(&vals, 0).unwrap_err(), CellError::Num);
        assert_eq!(quartile_exc(&vals, 4).unwrap_err(), CellError::Num);
    }

    #[test]
    fn quartile_exc_single_value_returns_num() {
        let vals = [Value::Number(5.0)];
        assert_eq!(quartile_exc(&vals, 1).unwrap_err(), CellError::Num);
    }

    #[test]
    fn quartile_exc_empty_returns_num() {
        assert_eq!(quartile_exc(&[], 1).unwrap_err(), CellError::Num);
    }

    #[test]
    fn quartile_exc_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Na)];
        assert_eq!(quartile_exc(&vals, 1).unwrap_err(), CellError::Na);
    }

    // ===== LARGE =====

    #[test]
    fn large_k1_returns_max() {
        let vals = n(&[3.0, 1.0, 4.0, 1.0, 5.0, 9.0]);
        assert_eq!(large(&vals, &Value::Number(1.0)).unwrap(), 9.0);
    }

    #[test]
    fn large_k2_returns_second_largest() {
        let vals = n(&[3.0, 1.0, 4.0, 1.0, 5.0, 9.0]);
        assert_eq!(large(&vals, &Value::Number(2.0)).unwrap(), 5.0);
    }

    #[test]
    fn large_k_equals_n_returns_smallest() {
        let vals = n(&[3.0, 1.0, 4.0, 1.0, 5.0, 9.0]);
        assert_eq!(large(&vals, &Value::Number(6.0)).unwrap(), 1.0);
    }

    #[test]
    fn large_single_value() {
        let vals = [Value::Number(5.0)];
        assert_eq!(large(&vals, &Value::Number(1.0)).unwrap(), 5.0);
    }

    #[test]
    fn large_all_same() {
        let vals = n(&[5.0, 5.0, 5.0]);
        assert_eq!(large(&vals, &Value::Number(2.0)).unwrap(), 5.0);
    }

    #[test]
    fn large_k_exceeds_n_returns_num() {
        let vals = n(&[1.0, 2.0, 3.0]);
        assert_eq!(large(&vals, &Value::Number(4.0)).unwrap_err(), CellError::Num);
    }

    #[test]
    fn large_k_zero_returns_num() {
        let vals = n(&[1.0, 2.0, 3.0]);
        assert_eq!(large(&vals, &Value::Number(0.0)).unwrap_err(), CellError::Num);
    }

    #[test]
    fn large_k_negative_returns_num() {
        let vals = n(&[1.0, 2.0, 3.0]);
        assert_eq!(large(&vals, &Value::Number(-1.0)).unwrap_err(), CellError::Num);
    }

    #[test]
    fn large_k_fractional_truncates() {
        let vals = n(&[1.0, 2.0, 3.0]);
        assert_eq!(large(&vals, &Value::Number(1.9)).unwrap(), 3.0);
    }

    #[test]
    fn large_empty_returns_num() {
        assert_eq!(large(&[], &Value::Number(1.0)).unwrap_err(), CellError::Num);
    }

    #[test]
    fn large_all_text_returns_num() {
        let vals = [Value::Text("a".into()), Value::Text("b".into())];
        assert_eq!(large(&vals, &Value::Number(1.0)).unwrap_err(), CellError::Num);
    }

    #[test]
    fn large_skips_text() {
        let vals = [Value::Number(1.0), Value::Text("text".into()), Value::Number(3.0)];
        assert_eq!(large(&vals, &Value::Number(1.0)).unwrap(), 3.0);
    }

    #[test]
    fn large_skips_bool() {
        let vals = [Value::Number(1.0), Value::Bool(true), Value::Number(3.0)];
        assert_eq!(large(&vals, &Value::Number(1.0)).unwrap(), 3.0);
    }

    #[test]
    fn large_propagates_error() {
        let vals = [Value::Number(1.0), Value::Error(CellError::Na), Value::Number(3.0)];
        assert_eq!(large(&vals, &Value::Number(1.0)).unwrap_err(), CellError::Na);
    }

    #[test]
    fn large_nan_input_returns_num() {
        let vals = [Value::Number(f64::NAN), Value::Number(1.0)];
        assert_eq!(large(&vals, &Value::Number(1.0)).unwrap_err(), CellError::Num);
    }

    #[test]
    fn large_infinity_input_returns_num() {
        let vals = [Value::Number(f64::INFINITY), Value::Number(1.0)];
        assert_eq!(large(&vals, &Value::Number(1.0)).unwrap_err(), CellError::Num);
    }

    // ===== SMALL =====

    #[test]
    fn small_k1_returns_min() {
        let vals = n(&[3.0, 1.0, 4.0, 1.0, 5.0, 9.0]);
        assert_eq!(small(&vals, &Value::Number(1.0)).unwrap(), 1.0);
    }

    #[test]
    fn small_k2_returns_duplicate() {
        let vals = n(&[3.0, 1.0, 4.0, 1.0, 5.0, 9.0]);
        assert_eq!(small(&vals, &Value::Number(2.0)).unwrap(), 1.0);
    }

    #[test]
    fn small_k3() {
        let vals = n(&[3.0, 1.0, 4.0, 1.0, 5.0, 9.0]);
        assert_eq!(small(&vals, &Value::Number(3.0)).unwrap(), 3.0);
    }

    #[test]
    fn small_single_value() {
        let vals = [Value::Number(5.0)];
        assert_eq!(small(&vals, &Value::Number(1.0)).unwrap(), 5.0);
    }

    #[test]
    fn small_negative_values() {
        let vals = n(&[-5.0, -3.0, -1.0]);
        assert_eq!(small(&vals, &Value::Number(1.0)).unwrap(), -5.0);
    }

    #[test]
    fn small_k_exceeds_n_returns_num() {
        let vals = n(&[1.0, 2.0, 3.0]);
        assert_eq!(small(&vals, &Value::Number(4.0)).unwrap_err(), CellError::Num);
    }

    /// Helper: build `Vec<Value::Number>` from f64 slice.
    fn n(vals: &[f64]) -> Vec<Value> {
        vals.iter().map(|&v| Value::Number(v)).collect()
    }

    // ===== RANK.EQ =====

    #[test]
    fn rank_eq_descending() {
        assert_close(rank_eq(3.0, &[5.0, 3.0, 1.0], false).unwrap(), 2.0);
    }

    #[test]
    fn rank_eq_ascending() {
        assert_close(rank_eq(3.0, &[1.0, 3.0, 5.0], true).unwrap(), 2.0);
    }

    #[test]
    fn rank_eq_highest_descending() {
        assert_close(rank_eq(5.0, &[5.0, 3.0, 1.0], false).unwrap(), 1.0);
    }

    #[test]
    fn rank_eq_lowest_descending() {
        assert_close(rank_eq(1.0, &[5.0, 3.0, 1.0], false).unwrap(), 3.0);
    }

    #[test]
    fn rank_eq_not_found_returns_na() {
        assert_eq!(rank_eq(4.0, &[1.0, 2.0, 3.0, 5.0], false).unwrap_err(), CellError::Na);
    }

    #[test]
    fn rank_eq_single_found() {
        assert_close(rank_eq(5.0, &[5.0], false).unwrap(), 1.0);
    }

    #[test]
    fn rank_eq_single_not_found() {
        assert_eq!(rank_eq(3.0, &[5.0], false).unwrap_err(), CellError::Na);
    }

    #[test]
    fn rank_eq_all_same() {
        assert_close(rank_eq(5.0, &[5.0, 5.0, 5.0], false).unwrap(), 1.0);
    }

    #[test]
    fn rank_eq_empty_returns_na() {
        assert_eq!(rank_eq(1.0, &[], false).unwrap_err(), CellError::Na);
    }

    #[test]
    fn rank_eq_duplicates_get_top_rank() {
        assert_close(rank_eq(3.0, &[5.0, 3.0, 3.0, 1.0], false).unwrap(), 2.0);
    }

    #[test]
    fn rank_eq_negative_ascending() {
        assert_close(rank_eq(-1.0, &[-3.0, -1.0, 0.0, 2.0], true).unwrap(), 2.0);
    }

    // exact f64 equality — no epsilon tolerance
    #[test]
    fn rank_eq_float_mismatch_returns_na() {
        assert_eq!(rank_eq(0.1_f64 + 0.2, &[0.3, 0.5], false).unwrap_err(), CellError::Na);
    }

    // ===== RANK.AVG =====

    #[test]
    fn rank_avg_with_duplicates() {
        assert_close(rank_avg(3.0, &[5.0, 3.0, 3.0, 1.0], false).unwrap(), 2.5);
    }

    #[test]
    fn rank_avg_ascending_duplicates() {
        assert_close(rank_avg(3.0, &[1.0, 3.0, 3.0, 5.0], true).unwrap(), 2.5);
    }

    #[test]
    fn rank_avg_no_duplicates() {
        assert_close(rank_avg(5.0, &[5.0, 3.0, 1.0], false).unwrap(), 1.0);
    }

    #[test]
    fn rank_avg_all_same() {
        assert_close(rank_avg(5.0, &[5.0, 5.0, 5.0], false).unwrap(), 2.0);
    }

    #[test]
    fn rank_avg_not_found() {
        assert_eq!(rank_avg(4.0, &[1.0, 2.0, 3.0], false).unwrap_err(), CellError::Na);
    }

    #[test]
    fn rank_avg_empty_returns_na() {
        assert_eq!(rank_avg(1.0, &[], false).unwrap_err(), CellError::Na);
    }

    // ===== EXPON.DIST =====

    #[test]
    fn expon_cdf_typical() {
        assert_close(expon_dist(1.0, 1.5, true).unwrap(), 0.776_869_839_851_570_2);
    }

    #[test]
    fn expon_pdf_typical() {
        assert_close(expon_dist(1.0, 1.5, false).unwrap(), 0.334_695_240_222_645_3);
    }

    #[test]
    fn expon_cdf_at_zero() {
        assert_close(expon_dist(0.0, 1.5, true).unwrap(), 0.0);
    }

    #[test]
    fn expon_pdf_at_zero() {
        assert_close(expon_dist(0.0, 1.5, false).unwrap(), 1.5);
    }

    #[test]
    fn expon_cdf_large_x() {
        assert_close(expon_dist(10.0, 1.0, true).unwrap(), 0.999_954_600_070_238);
    }

    #[test]
    fn expon_x_negative_returns_num() {
        assert_eq!(expon_dist(-1.0, 1.5, true).unwrap_err(), CellError::Num);
    }

    #[test]
    fn expon_lambda_zero_returns_num() {
        assert_eq!(expon_dist(1.0, 0.0, true).unwrap_err(), CellError::Num);
    }

    #[test]
    fn expon_lambda_negative_returns_num() {
        assert_eq!(expon_dist(1.0, -1.0, true).unwrap_err(), CellError::Num);
    }

    #[test]
    fn expon_very_large_lambda() {
        assert_close(expon_dist(1.0, 1000.0, true).unwrap(), 1.0);
    }

    #[test]
    fn expon_very_small_lambda() {
        assert_close(expon_dist(1.0, 0.001, true).unwrap(), 0.000_999_500_166_625_0);
    }

    #[test]
    fn expon_nan_x_returns_num() {
        assert_eq!(expon_dist(f64::NAN, 1.5, true).unwrap_err(), CellError::Num);
    }

    #[test]
    fn expon_nan_lambda_returns_num() {
        assert_eq!(expon_dist(1.0, f64::NAN, true).unwrap_err(), CellError::Num);
    }

    #[test]
    fn expon_infinity_x_returns_num() {
        assert_eq!(expon_dist(f64::INFINITY, 1.5, true).unwrap_err(), CellError::Num);
    }

    #[test]
    fn expon_infinity_lambda_returns_num() {
        assert_eq!(expon_dist(1.0, f64::INFINITY, true).unwrap_err(), CellError::Num);
    }

    // ===== POISSON.DIST =====

    #[test]
    fn poisson_pmf_typical() {
        let args = [Value::Number(3.0), Value::Number(5.0), Value::Bool(false)];
        assert_close(builtin_poisson_dist(&args).unwrap(), 0.140_373_895_814_280_5);
    }

    #[test]
    fn poisson_cdf_typical() {
        let args = [Value::Number(3.0), Value::Number(5.0), Value::Bool(true)];
        assert_close(builtin_poisson_dist(&args).unwrap(), 0.265_025_915_297_842_7);
    }

    #[test]
    fn poisson_pmf_zero() {
        let args = [Value::Number(0.0), Value::Number(5.0), Value::Bool(false)];
        assert_close(builtin_poisson_dist(&args).unwrap(), (-5.0_f64).exp());
    }

    #[test]
    fn poisson_cdf_zero() {
        let args = [Value::Number(0.0), Value::Number(5.0), Value::Bool(true)];
        assert_close(builtin_poisson_dist(&args).unwrap(), (-5.0_f64).exp());
    }

    #[test]
    fn poisson_pmf_equals_mean() {
        let args = [Value::Number(5.0), Value::Number(5.0), Value::Bool(false)];
        assert_close(builtin_poisson_dist(&args).unwrap(), 0.175_467_369_767_850_74);
    }

    #[test]
    fn poisson_mean_zero_x_zero() {
        let args = [Value::Number(0.0), Value::Number(0.0), Value::Bool(false)];
        assert_close(builtin_poisson_dist(&args).unwrap(), 1.0);
    }

    #[test]
    fn poisson_mean_zero_x_zero_cdf() {
        let args = [Value::Number(0.0), Value::Number(0.0), Value::Bool(true)];
        assert_close(builtin_poisson_dist(&args).unwrap(), 1.0);
    }

    #[test]
    fn poisson_x_negative_returns_num() {
        let args = [Value::Number(-1.0), Value::Number(5.0), Value::Bool(false)];
        assert_eq!(builtin_poisson_dist(&args).unwrap_err(), CellError::Num);
    }

    #[test]
    fn poisson_mean_negative_returns_num() {
        let args = [Value::Number(3.0), Value::Number(-1.0), Value::Bool(false)];
        assert_eq!(builtin_poisson_dist(&args).unwrap_err(), CellError::Num);
    }

    #[test]
    fn poisson_non_integer_x_truncates() {
        let args_frac = [Value::Number(3.7), Value::Number(5.0), Value::Bool(false)];
        let args_int = [Value::Number(3.0), Value::Number(5.0), Value::Bool(false)];
        assert_close(
            builtin_poisson_dist(&args_frac).unwrap(),
            builtin_poisson_dist(&args_int).unwrap(),
        );
    }

    #[test]
    fn poisson_large_mean() {
        let args = [Value::Number(100.0), Value::Number(100.0), Value::Bool(true)];
        let result = builtin_poisson_dist(&args).unwrap();
        assert!(result > 0.0 && result <= 1.0);
    }

    #[test]
    fn poisson_large_x() {
        let args = [Value::Number(200.0), Value::Number(100.0), Value::Bool(true)];
        let result = builtin_poisson_dist(&args).unwrap();
        assert!((result - 1.0).abs() < 1e-6);
    }

    #[test]
    fn poisson_propagates_error() {
        let args = [Value::Error(CellError::Na), Value::Number(5.0), Value::Bool(false)];
        assert_eq!(builtin_poisson_dist(&args).unwrap_err(), CellError::Na);
    }

    #[test]
    fn poisson_wrong_arg_count() {
        let args = [Value::Number(3.0), Value::Number(5.0)];
        assert_eq!(builtin_poisson_dist(&args).unwrap_err(), CellError::Value);
    }

    #[test]
    fn poisson_nan_x_returns_num() {
        let args = [Value::Number(f64::NAN), Value::Number(5.0), Value::Bool(false)];
        assert_eq!(builtin_poisson_dist(&args).unwrap_err(), CellError::Num);
    }

    #[test]
    fn poisson_infinity_x_returns_num() {
        let args = [Value::Number(f64::INFINITY), Value::Number(5.0), Value::Bool(true)];
        assert_eq!(builtin_poisson_dist(&args).unwrap_err(), CellError::Num);
    }

    #[test]
    fn poisson_nan_mean_returns_num() {
        let args = [Value::Number(3.0), Value::Number(f64::NAN), Value::Bool(false)];
        assert_eq!(builtin_poisson_dist(&args).unwrap_err(), CellError::Num);
    }

    #[test]
    fn poisson_infinity_mean_returns_num() {
        let args = [Value::Number(3.0), Value::Number(f64::INFINITY), Value::Bool(false)];
        assert_eq!(builtin_poisson_dist(&args).unwrap_err(), CellError::Num);
    }

    #[test]
    fn poisson_coerces_text_x_to_number() {
        let args = [Value::Text("3".into()), Value::Number(5.0), Value::Bool(false)];
        let expected = [Value::Number(3.0), Value::Number(5.0), Value::Bool(false)];
        assert_close(
            builtin_poisson_dist(&args).unwrap(),
            builtin_poisson_dist(&expected).unwrap(),
        );
    }

    #[test]
    fn poisson_coerces_bool_mean_to_number() {
        let args = [Value::Number(0.0), Value::Bool(true), Value::Bool(false)];
        let expected = [Value::Number(0.0), Value::Number(1.0), Value::Bool(false)];
        assert_close(
            builtin_poisson_dist(&args).unwrap(),
            builtin_poisson_dist(&expected).unwrap(),
        );
    }

    #[test]
    fn poisson_coerces_number_cumulative_to_bool() {
        let args = [Value::Number(3.0), Value::Number(5.0), Value::Number(1.0)];
        let expected = [Value::Number(3.0), Value::Number(5.0), Value::Bool(true)];
        assert_close(
            builtin_poisson_dist(&args).unwrap(),
            builtin_poisson_dist(&expected).unwrap(),
        );
    }

    #[test]
    fn poisson_non_numeric_text_returns_value() {
        let args = [Value::Text("abc".into()), Value::Number(5.0), Value::Bool(false)];
        assert_eq!(builtin_poisson_dist(&args).unwrap_err(), CellError::Value);
    }

    #[test]
    fn poisson_very_large_x_cdf_returns_one() {
        let args = [Value::Number(1e7), Value::Number(100.0), Value::Bool(true)];
        assert_close(builtin_poisson_dist(&args).unwrap(), 1.0);
    }

    // ===== ln_gamma =====

    #[test]
    fn ln_gamma_of_1_is_zero() {
        assert_close(ln_gamma(1.0), 0.0);
    }

    #[test]
    fn ln_gamma_of_2_is_zero() {
        assert_close(ln_gamma(2.0), 0.0);
    }

    #[test]
    fn ln_gamma_of_6_is_ln_120() {
        assert_close(ln_gamma(6.0), 120.0_f64.ln());
    }

    #[test]
    fn ln_gamma_half_is_ln_sqrt_pi() {
        assert_close(ln_gamma(0.5), std::f64::consts::PI.sqrt().ln());
    }

    #[test]
    fn ln_gamma_large_value() {
        let expected: f64 = (1..=100).map(|i: i32| f64::from(i).ln()).sum();
        assert!((ln_gamma(101.0) - expected).abs() < 1e-8);
    }

    // ===== T.DIST =====

    fn assert_approx(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-4,
            "expected {expected}, got {actual} (diff {})",
            (actual - expected).abs()
        );
    }

    #[test]
    fn t_dist_cdf_positive() {
        match builtin_t_dist(&[Value::Number(1.0), Value::Number(10.0), Value::Bool(true)]) {
            Value::Number(n) => assert_approx(n, 0.82955),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn t_dist_cdf_at_zero_is_half() {
        match builtin_t_dist(&[Value::Number(0.0), Value::Number(10.0), Value::Bool(true)]) {
            Value::Number(n) => assert_close(n, 0.5),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn t_dist_pdf_at_zero() {
        match builtin_t_dist(&[Value::Number(0.0), Value::Number(10.0), Value::Bool(false)]) {
            Value::Number(n) => assert_approx(n, 0.38909),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn t_dist_df_one_is_cauchy() {
        match builtin_t_dist(&[Value::Number(1.0), Value::Number(1.0), Value::Bool(true)]) {
            Value::Number(n) => assert_approx(n, 0.75),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn t_dist_non_integer_df_truncates() {
        let full = builtin_t_dist(&[Value::Number(1.0), Value::Number(10.0), Value::Bool(true)]);
        let trunc = builtin_t_dist(&[Value::Number(1.0), Value::Number(10.7), Value::Bool(true)]);
        assert_eq!(full, trunc);
    }

    #[test]
    fn t_dist_df_zero_returns_num() {
        assert_eq!(
            builtin_t_dist(&[Value::Number(1.0), Value::Number(0.0), Value::Bool(true)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn t_dist_propagates_error() {
        assert_eq!(
            builtin_t_dist(&[Value::Error(CellError::Na), Value::Number(10.0), Value::Bool(true)]),
            Value::Error(CellError::Na),
        );
    }

    #[test]
    fn t_dist_wrong_arg_count() {
        assert_eq!(
            builtin_t_dist(&[Value::Number(1.0), Value::Number(10.0)]),
            Value::Error(CellError::Value),
        );
    }

    // ===== T.DIST.RT =====

    #[test]
    fn t_dist_rt_positive() {
        match builtin_t_dist_rt(&[Value::Number(1.0), Value::Number(10.0)]) {
            Value::Number(n) => assert_approx(n, 0.17045),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn t_dist_rt_at_zero() {
        match builtin_t_dist_rt(&[Value::Number(0.0), Value::Number(10.0)]) {
            Value::Number(n) => assert_approx(n, 0.5),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn t_dist_rt_wrong_arg_count() {
        assert_eq!(builtin_t_dist_rt(&[Value::Number(1.0)]), Value::Error(CellError::Value),);
    }

    #[test]
    fn t_dist_rt_propagates_error() {
        assert_eq!(
            builtin_t_dist_rt(&[Value::Error(CellError::Na), Value::Number(10.0)]),
            Value::Error(CellError::Na),
        );
    }

    // ===== T.DIST.2T =====

    #[test]
    fn t_dist_2t_positive() {
        match builtin_t_dist_2t(&[Value::Number(1.0), Value::Number(10.0)]) {
            Value::Number(n) => assert_approx(n, 0.34090),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn t_dist_2t_at_zero_is_one() {
        match builtin_t_dist_2t(&[Value::Number(0.0), Value::Number(10.0)]) {
            Value::Number(n) => assert_approx(n, 1.0),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn t_dist_2t_x_negative_returns_num() {
        assert_eq!(
            builtin_t_dist_2t(&[Value::Number(-1.0), Value::Number(10.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn t_dist_2t_wrong_arg_count() {
        assert_eq!(builtin_t_dist_2t(&[Value::Number(1.0)]), Value::Error(CellError::Value),);
    }

    #[test]
    fn t_dist_2t_propagates_error() {
        assert_eq!(
            builtin_t_dist_2t(&[Value::Number(1.0), Value::Error(CellError::Na)]),
            Value::Error(CellError::Na),
        );
    }

    // ===== T.INV =====

    #[test]
    fn t_inv_median_is_zero() {
        match builtin_t_inv(&[Value::Number(0.5), Value::Number(10.0)]) {
            Value::Number(n) => assert!((n).abs() < 1e-8, "expected ~0, got {n}"),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn t_inv_ninety_five_pct() {
        match builtin_t_inv(&[Value::Number(0.95), Value::Number(10.0)]) {
            Value::Number(n) => assert_approx(n, 1.81246),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn t_inv_left_tail() {
        match builtin_t_inv(&[Value::Number(0.025), Value::Number(10.0)]) {
            Value::Number(n) => assert_approx(n, -2.22814),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn t_inv_p_zero_returns_num() {
        assert_eq!(
            builtin_t_inv(&[Value::Number(0.0), Value::Number(10.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn t_inv_p_one_returns_num() {
        assert_eq!(
            builtin_t_inv(&[Value::Number(1.0), Value::Number(10.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn t_inv_wrong_arg_count() {
        assert_eq!(builtin_t_inv(&[Value::Number(0.5)]), Value::Error(CellError::Value),);
    }

    #[test]
    fn t_inv_propagates_error() {
        assert_eq!(
            builtin_t_inv(&[Value::Error(CellError::Na), Value::Number(10.0)]),
            Value::Error(CellError::Na),
        );
    }

    // ===== T.INV.2T =====

    #[test]
    fn t_inv_2t_five_pct() {
        match builtin_t_inv_2t(&[Value::Number(0.05), Value::Number(10.0)]) {
            Value::Number(n) => assert_approx(n, 2.22814),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn t_inv_2t_at_one_is_zero() {
        match builtin_t_inv_2t(&[Value::Number(1.0), Value::Number(10.0)]) {
            Value::Number(n) => assert!(n.abs() < 1e-10, "expected 0, got {n}"),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn t_inv_2t_symmetric_with_t_inv() {
        let p = 0.05;
        let df = 10.0;
        let inv_2t = builtin_t_inv_2t(&[Value::Number(p), Value::Number(df)]);
        let inv_left = builtin_t_inv(&[Value::Number(p / 2.0), Value::Number(df)]);
        match (inv_2t, inv_left) {
            (Value::Number(a), Value::Number(b)) => assert_approx(a, -b),
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn t_inv_2t_p_zero_returns_num() {
        assert_eq!(
            builtin_t_inv_2t(&[Value::Number(0.0), Value::Number(10.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn t_inv_2t_p_above_one_returns_num() {
        assert_eq!(
            builtin_t_inv_2t(&[Value::Number(1.1), Value::Number(10.0)]),
            Value::Error(CellError::Num),
        );
    }

    #[test]
    fn t_inv_2t_wrong_arg_count() {
        assert_eq!(builtin_t_inv_2t(&[Value::Number(0.5)]), Value::Error(CellError::Value),);
    }

    #[test]
    fn t_inv_2t_propagates_error() {
        assert_eq!(
            builtin_t_inv_2t(&[Value::Number(0.5), Value::Error(CellError::Na)]),
            Value::Error(CellError::Na),
        );
    }
}
