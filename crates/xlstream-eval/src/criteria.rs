//! Criteria parsing and matching for conditional aggregates (SUMIF, COUNTIF,
//! AVERAGEIF).
//!
//! Excel criteria strings encode an operator and a value in a single string.
//! [`Criteria::parse`] converts that string into a structured enum, and
//! [`Criteria::matches`] tests a [`Value`] against it.

use xlstream_core::{coerce, Value};

/// A parsed criteria expression for conditional aggregate functions.
///
/// Built from a criteria string via [`Criteria::parse`]. Tested against cell
/// values via [`Criteria::matches`].
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::Criteria;
///
/// let c = Criteria::parse(">10");
/// assert!(c.matches(&Value::Number(20.0)));
/// assert!(!c.matches(&Value::Number(5.0)));
/// ```
#[derive(Debug, Clone)]
pub enum Criteria {
    /// Exact equality (default when no operator prefix).
    Equals(Value),
    /// `<>value` — not equal.
    NotEquals(Value),
    /// `>number` — strictly greater.
    Greater(f64),
    /// `>=number` — greater or equal.
    GreaterOrEq(f64),
    /// `<number` — strictly less.
    Less(f64),
    /// `<=number` — less or equal.
    LessOrEq(f64),
    /// Wildcard pattern with `*` and `?`.
    Wildcard(WildcardPattern),
    /// Empty string criteria — matches blank/empty cells.
    Blank,
    /// `<>` with no value — matches non-blank cells.
    NonBlank,
}

/// A compiled wildcard pattern supporting `*` (zero or more chars) and `?`
/// (exactly one char). Case-insensitive matching.
///
/// # Examples
///
/// ```
/// use xlstream_eval::criteria::WildcardPattern;
///
/// let p = WildcardPattern::new("a*z");
/// assert!(p.matches("abcz"));
/// assert!(!p.matches("abc"));
/// ```
#[derive(Debug, Clone)]
pub struct WildcardPattern {
    /// The original pattern, lowercased.
    pattern: String,
}

impl WildcardPattern {
    /// Compile a wildcard pattern string. The pattern is lowercased for
    /// case-insensitive matching.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_eval::criteria::WildcardPattern;
    /// let p = WildcardPattern::new("Hello*");
    /// assert!(p.matches("hello world"));
    /// ```
    #[must_use]
    pub fn new(pattern: &str) -> Self {
        Self { pattern: pattern.to_ascii_lowercase() }
    }

    /// Test whether `text` matches this wildcard pattern.
    ///
    /// Uses DP matching: `*` matches zero or more characters, `?` matches
    /// exactly one character. Case-insensitive.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_eval::criteria::WildcardPattern;
    /// let p = WildcardPattern::new("a?c");
    /// assert!(p.matches("abc"));
    /// assert!(!p.matches("ac"));
    /// ```
    #[must_use]
    pub fn matches(&self, text: &str) -> bool {
        let text_lower = text.to_ascii_lowercase();
        let pat: Vec<char> = self.pattern.chars().collect();
        let text_chars: Vec<char> = text_lower.chars().collect();

        let m = pat.len();
        let n = text_chars.len();

        // dp[i][j] = pattern[0..i] matches text[0..j]
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;

        // Leading *s can match empty text
        for (i, &pc) in pat.iter().enumerate() {
            if pc == '*' {
                dp[i + 1][0] = dp[i][0];
            } else {
                break;
            }
        }

        for (i, &pc) in pat.iter().enumerate() {
            for (j, &tc) in text_chars.iter().enumerate() {
                match pc {
                    '*' => {
                        // * matches zero chars (dp[i][j+1]) or one more char (dp[i+1][j])
                        dp[i + 1][j + 1] = dp[i][j + 1] || dp[i + 1][j];
                    }
                    '?' => {
                        dp[i + 1][j + 1] = dp[i][j];
                    }
                    c => {
                        dp[i + 1][j + 1] = dp[i][j] && c == tc;
                    }
                }
            }
        }

        dp[m][n]
    }
}

impl Criteria {
    /// Parse a criteria string into a [`Criteria`] value.
    ///
    /// Supported formats:
    /// - `""` (empty) -> [`Criteria::Blank`]
    /// - `"<>"` -> [`Criteria::NonBlank`]
    /// - `">N"`, `">=N"`, `"<N"`, `"<=N"` -> numeric comparison
    /// - `"<>val"` -> [`Criteria::NotEquals`]
    /// - `"=val"` -> [`Criteria::Equals`]
    /// - Text containing `*` or `?` -> [`Criteria::Wildcard`]
    /// - Plain text/number -> [`Criteria::Equals`]
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_eval::Criteria;
    /// let c = Criteria::parse(">=100");
    /// assert!(matches!(c, Criteria::GreaterOrEq(n) if (n - 100.0).abs() < f64::EPSILON));
    /// ```
    #[must_use]
    pub fn parse(s: &str) -> Self {
        let trimmed = s.trim();

        if trimmed.is_empty() {
            return Criteria::Blank;
        }

        // <>value or just <>
        if let Some(rest) = trimmed.strip_prefix("<>") {
            if rest.is_empty() {
                return Criteria::NonBlank;
            }
            return Criteria::NotEquals(parse_criteria_value(rest));
        }

        // >= or <=
        if let Some(rest) = trimmed.strip_prefix(">=") {
            if let Ok(n) = rest.trim().parse::<f64>() {
                if n.is_finite() {
                    return Criteria::GreaterOrEq(n);
                }
            }
            return Criteria::Equals(parse_criteria_value(trimmed));
        }
        if let Some(rest) = trimmed.strip_prefix("<=") {
            if let Ok(n) = rest.trim().parse::<f64>() {
                if n.is_finite() {
                    return Criteria::LessOrEq(n);
                }
            }
            return Criteria::Equals(parse_criteria_value(trimmed));
        }

        // > or <
        if let Some(rest) = trimmed.strip_prefix('>') {
            if let Ok(n) = rest.trim().parse::<f64>() {
                if n.is_finite() {
                    return Criteria::Greater(n);
                }
            }
            return Criteria::Equals(parse_criteria_value(trimmed));
        }
        if let Some(rest) = trimmed.strip_prefix('<') {
            if let Ok(n) = rest.trim().parse::<f64>() {
                if n.is_finite() {
                    return Criteria::Less(n);
                }
            }
            return Criteria::Equals(parse_criteria_value(trimmed));
        }

        // =value
        if let Some(rest) = trimmed.strip_prefix('=') {
            return Criteria::Equals(parse_criteria_value(rest));
        }

        // Wildcard check
        if trimmed.contains('*') || trimmed.contains('?') {
            return Criteria::Wildcard(WildcardPattern::new(trimmed));
        }

        // Plain value
        Criteria::Equals(parse_criteria_value(trimmed))
    }

    /// Test whether `v` matches this criteria.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_core::Value;
    /// use xlstream_eval::Criteria;
    ///
    /// let c = Criteria::parse("hello");
    /// assert!(c.matches(&Value::Text("Hello".into())));
    /// assert!(!c.matches(&Value::Number(1.0)));
    /// ```
    #[must_use]
    pub fn matches(&self, v: &Value) -> bool {
        match self {
            Criteria::Blank => is_blank(v),
            Criteria::NonBlank => !is_blank(v),
            Criteria::Equals(target) => criteria_equal(v, target),
            Criteria::NotEquals(target) => !criteria_equal(v, target),
            Criteria::Greater(n) => numeric_for_compare(v).is_some_and(|vn| vn > *n),
            Criteria::GreaterOrEq(n) => numeric_for_compare(v).is_some_and(|vn| vn >= *n),
            Criteria::Less(n) => numeric_for_compare(v).is_some_and(|vn| vn < *n),
            Criteria::LessOrEq(n) => numeric_for_compare(v).is_some_and(|vn| vn <= *n),
            Criteria::Wildcard(pat) => {
                let text = coerce::to_text(v);
                pat.matches(&text)
            }
        }
    }
}

/// Test whether a value is blank (empty or empty text).
fn is_blank(v: &Value) -> bool {
    match v {
        Value::Empty => true,
        Value::Text(s) => s.is_empty(),
        _ => false,
    }
}

/// Parse a criteria value string into a [`Value`]. Tries number first, then
/// falls back to text.
fn parse_criteria_value(s: &str) -> Value {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Value::Empty;
    }
    if let Ok(n) = trimmed.parse::<f64>() {
        if n.is_finite() {
            return Value::Number(n);
        }
    }
    if trimmed.eq_ignore_ascii_case("true") {
        return Value::Bool(true);
    }
    if trimmed.eq_ignore_ascii_case("false") {
        return Value::Bool(false);
    }
    Value::Text(trimmed.into())
}

/// Extract a numeric value for comparison operators. Returns `None` for
/// text, errors, and empty values (they don't participate in numeric
/// comparisons).
fn numeric_for_compare(v: &Value) -> Option<f64> {
    match v {
        Value::Number(n) => Some(*n),
        #[allow(clippy::cast_precision_loss)]
        Value::Integer(i) => Some(*i as f64),
        Value::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
        Value::Date(d) => Some(d.serial),
        Value::Empty | Value::Text(_) | Value::Error(_) => None,
    }
}

/// Case-insensitive equality for criteria matching. Numbers compare by value,
/// text compares case-insensitively, booleans compare directly.
#[allow(clippy::float_cmp)]
fn criteria_equal(v: &Value, target: &Value) -> bool {
    match (v, target) {
        (Value::Number(a), Value::Number(b)) => a == b,
        #[allow(clippy::cast_precision_loss)]
        (Value::Number(a), Value::Integer(b)) => *a == *b as f64,
        #[allow(clippy::cast_precision_loss)]
        (Value::Integer(a), Value::Number(b)) => *a as f64 == *b,
        (Value::Integer(a), Value::Integer(b)) => a == b,
        (Value::Text(a), Value::Text(b)) => a.eq_ignore_ascii_case(b),
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::Empty, Value::Empty) => true,
        (Value::Empty, Value::Number(n)) | (Value::Number(n), Value::Empty) => *n == 0.0,
        (Value::Empty, Value::Text(s)) | (Value::Text(s), Value::Empty) => s.is_empty(),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use super::*;

    // -- Operator parsing --

    #[test]
    fn parse_greater_than() {
        let c = Criteria::parse(">10");
        assert!(matches!(c, Criteria::Greater(n) if (n - 10.0).abs() < f64::EPSILON));
    }

    #[test]
    fn parse_greater_or_eq() {
        let c = Criteria::parse(">=5");
        assert!(matches!(c, Criteria::GreaterOrEq(n) if (n - 5.0).abs() < f64::EPSILON));
    }

    #[test]
    fn parse_less_than() {
        let c = Criteria::parse("<3");
        assert!(matches!(c, Criteria::Less(n) if (n - 3.0).abs() < f64::EPSILON));
    }

    #[test]
    fn parse_less_or_eq() {
        let c = Criteria::parse("<=7");
        assert!(matches!(c, Criteria::LessOrEq(n) if (n - 7.0).abs() < f64::EPSILON));
    }

    #[test]
    fn parse_not_equals_number() {
        let c = Criteria::parse("<>5");
        assert!(
            matches!(c, Criteria::NotEquals(Value::Number(n)) if (n - 5.0).abs() < f64::EPSILON)
        );
    }

    #[test]
    fn parse_not_equals_text() {
        let c = Criteria::parse("<>hello");
        assert!(matches!(c, Criteria::NotEquals(Value::Text(_))));
    }

    #[test]
    fn parse_equals_explicit() {
        let c = Criteria::parse("=42");
        assert!(matches!(c, Criteria::Equals(Value::Number(n)) if (n - 42.0).abs() < f64::EPSILON));
    }

    #[test]
    fn parse_equals_implicit_number() {
        let c = Criteria::parse("100");
        assert!(
            matches!(c, Criteria::Equals(Value::Number(n)) if (n - 100.0).abs() < f64::EPSILON)
        );
    }

    #[test]
    fn parse_equals_implicit_text() {
        let c = Criteria::parse("hello");
        assert!(matches!(c, Criteria::Equals(Value::Text(ref s)) if s.as_ref() == "hello"));
    }

    // -- Blank / NonBlank --

    #[test]
    fn parse_empty_is_blank() {
        let c = Criteria::parse("");
        assert!(matches!(c, Criteria::Blank));
    }

    #[test]
    fn parse_not_equals_empty_is_nonblank() {
        let c = Criteria::parse("<>");
        assert!(matches!(c, Criteria::NonBlank));
    }

    #[test]
    fn blank_matches_empty() {
        let c = Criteria::parse("");
        assert!(c.matches(&Value::Empty));
        assert!(c.matches(&Value::Text("".into())));
        assert!(!c.matches(&Value::Number(0.0)));
    }

    #[test]
    fn nonblank_matches_nonempty() {
        let c = Criteria::parse("<>");
        assert!(!c.matches(&Value::Empty));
        assert!(c.matches(&Value::Number(0.0)));
        assert!(c.matches(&Value::Text("x".into())));
    }

    // -- Wildcard --

    #[test]
    fn parse_wildcard_star() {
        let c = Criteria::parse("a*");
        assert!(matches!(c, Criteria::Wildcard(_)));
    }

    #[test]
    fn wildcard_star_matches() {
        let c = Criteria::parse("a*z");
        assert!(c.matches(&Value::Text("abcz".into())));
        assert!(c.matches(&Value::Text("az".into())));
        assert!(!c.matches(&Value::Text("abc".into())));
    }

    #[test]
    fn wildcard_question_matches() {
        let c = Criteria::parse("a?c");
        assert!(c.matches(&Value::Text("abc".into())));
        assert!(!c.matches(&Value::Text("ac".into())));
        assert!(!c.matches(&Value::Text("abbc".into())));
    }

    #[test]
    fn wildcard_case_insensitive() {
        let c = Criteria::parse("HELLO*");
        assert!(c.matches(&Value::Text("hello world".into())));
        assert!(c.matches(&Value::Text("HELLO".into())));
    }

    // -- Matching --

    #[test]
    fn greater_than_matches_number() {
        let c = Criteria::parse(">10");
        assert!(c.matches(&Value::Number(20.0)));
        assert!(!c.matches(&Value::Number(10.0)));
        assert!(!c.matches(&Value::Number(5.0)));
    }

    #[test]
    fn greater_than_skips_text() {
        let c = Criteria::parse(">10");
        assert!(!c.matches(&Value::Text("abc".into())));
    }

    #[test]
    fn equals_text_case_insensitive() {
        let c = Criteria::parse("hello");
        assert!(c.matches(&Value::Text("Hello".into())));
        assert!(c.matches(&Value::Text("HELLO".into())));
        assert!(!c.matches(&Value::Text("world".into())));
    }

    #[test]
    fn not_equals_number() {
        let c = Criteria::parse("<>5");
        assert!(c.matches(&Value::Number(3.0)));
        assert!(!c.matches(&Value::Number(5.0)));
    }

    #[test]
    fn less_or_eq_boundary() {
        let c = Criteria::parse("<=10");
        assert!(c.matches(&Value::Number(10.0)));
        assert!(c.matches(&Value::Number(5.0)));
        assert!(!c.matches(&Value::Number(11.0)));
    }

    #[test]
    fn greater_or_eq_boundary() {
        let c = Criteria::parse(">=10");
        assert!(c.matches(&Value::Number(10.0)));
        assert!(c.matches(&Value::Number(15.0)));
        assert!(!c.matches(&Value::Number(9.0)));
    }
}
