//! [`LookupValue`] — type-aware, case-insensitive lookup key.

use std::hash::{Hash, Hasher};

use xlstream_core::Value;

/// f64 wrapper with total ordering and Hash support.
///
/// Normalizes `-0.0` to `+0.0` (Excel treats them as equal). Uses
/// `to_bits()` for Eq and Hash.
///
/// # Examples
///
/// ```
/// use xlstream_eval::lookup::value::OrderedF64;
/// let a = OrderedF64::new(1.5);
/// let b = OrderedF64::new(1.5);
/// assert_eq!(a, b);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct OrderedF64(f64);

impl OrderedF64 {
    /// Wrap an f64, normalizing `-0.0` to `+0.0`.
    #[must_use]
    pub fn new(v: f64) -> Self {
        if v == 0.0 {
            Self(0.0)
        } else {
            Self(v)
        }
    }
}

impl PartialEq for OrderedF64 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}
impl Eq for OrderedF64 {}

impl Hash for OrderedF64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl PartialOrd for OrderedF64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

/// Case-folded text for case-insensitive lookup matching.
///
/// # Examples
///
/// ```
/// use xlstream_eval::lookup::value::CaseFoldedText;
/// let a = CaseFoldedText::new("Hello");
/// let b = CaseFoldedText::new("hello");
/// assert_eq!(a, b);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CaseFoldedText(Box<str>);

impl CaseFoldedText {
    /// Lowercase `s` and store.
    #[must_use]
    pub fn new(s: &str) -> Self {
        Self(s.to_ascii_lowercase().into_boxed_str())
    }
}

/// Type-aware lookup key matching Excel's exact-match semantics.
///
/// - Number `1` and Text `"1"` are different keys.
/// - Text `"ABC"` and `"abc"` are the same key.
/// - Empty cells and errors cannot be lookup keys.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::lookup::LookupValue;
///
/// let key = LookupValue::from_value(&Value::Number(42.0));
/// assert!(key.is_some());
/// assert!(LookupValue::from_value(&Value::Empty).is_none());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LookupValue {
    /// Numeric key (f64 with total ordering).
    Number(OrderedF64),
    /// Case-insensitive text key.
    Text(CaseFoldedText),
    /// Boolean key.
    Bool(bool),
}

impl LookupValue {
    /// Convert a cell value to a lookup key.
    ///
    /// Returns `None` for `Empty` and `Error` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_core::Value;
    /// use xlstream_eval::lookup::LookupValue;
    ///
    /// let k = LookupValue::from_value(&Value::Text("hello".into()));
    /// assert!(k.is_some());
    /// ```
    #[must_use]
    pub fn from_value(v: &Value) -> Option<Self> {
        match v {
            Value::Number(n) => Some(Self::Number(OrderedF64::new(*n))),
            #[allow(clippy::cast_precision_loss)]
            Value::Integer(i) => Some(Self::Number(OrderedF64::new(*i as f64))),
            Value::Text(s) => Some(Self::Text(CaseFoldedText::new(s))),
            Value::Bool(b) => Some(Self::Bool(*b)),
            Value::Date(d) => Some(Self::Number(OrderedF64::new(d.serial))),
            Value::Empty | Value::Error(_) => None,
        }
    }
}

impl PartialOrd for LookupValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LookupValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        fn type_tier(v: &LookupValue) -> u8 {
            match v {
                LookupValue::Number(_) => 0,
                LookupValue::Text(_) => 1,
                LookupValue::Bool(_) => 2,
            }
        }
        let tier_cmp = type_tier(self).cmp(&type_tier(other));
        if tier_cmp != Ordering::Equal {
            return tier_cmp;
        }
        match (self, other) {
            (LookupValue::Number(a), LookupValue::Number(b)) => a.cmp(b),
            (LookupValue::Text(a), LookupValue::Text(b)) => a.cmp(b),
            (LookupValue::Bool(a), LookupValue::Bool(b)) => a.cmp(b),
            _ => Ordering::Equal,
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use std::collections::HashMap;

    use xlstream_core::Value;

    use super::*;

    #[test]
    fn number_and_text_one_differ() {
        let n = LookupValue::from_value(&Value::Number(1.0)).unwrap();
        let t = LookupValue::from_value(&Value::Text("1".into())).unwrap();
        assert_ne!(n, t);
    }

    #[test]
    fn text_case_insensitive() {
        let upper = LookupValue::from_value(&Value::Text("ABC".into())).unwrap();
        let lower = LookupValue::from_value(&Value::Text("abc".into())).unwrap();
        let mixed = LookupValue::from_value(&Value::Text("AbC".into())).unwrap();
        assert_eq!(upper, lower);
        assert_eq!(upper, mixed);
    }

    #[test]
    fn text_case_insensitive_hashes_equal() {
        let upper = LookupValue::from_value(&Value::Text("Hello".into())).unwrap();
        let lower = LookupValue::from_value(&Value::Text("hello".into())).unwrap();
        let mut map: HashMap<LookupValue, i32> = HashMap::new();
        map.insert(upper, 42);
        assert_eq!(map.get(&lower), Some(&42));
    }

    #[test]
    fn number_hashes_consistently() {
        let a = LookupValue::from_value(&Value::Number(7.25)).unwrap();
        let b = LookupValue::from_value(&Value::Number(7.25)).unwrap();
        let mut map: HashMap<LookupValue, i32> = HashMap::new();
        map.insert(a, 99);
        assert_eq!(map.get(&b), Some(&99));
    }

    #[test]
    fn bool_true_and_false_differ() {
        let t = LookupValue::from_value(&Value::Bool(true)).unwrap();
        let f = LookupValue::from_value(&Value::Bool(false)).unwrap();
        assert_ne!(t, f);
    }

    #[test]
    fn empty_returns_none() {
        assert!(LookupValue::from_value(&Value::Empty).is_none());
    }

    #[test]
    fn error_returns_none() {
        assert!(LookupValue::from_value(&Value::Error(xlstream_core::CellError::Na)).is_none());
    }

    #[test]
    fn integer_converts_to_number() {
        let i = LookupValue::from_value(&Value::Integer(42)).unwrap();
        let n = LookupValue::from_value(&Value::Number(42.0)).unwrap();
        assert_eq!(i, n);
    }

    #[test]
    fn date_converts_to_number() {
        let d = LookupValue::from_value(&Value::Date(xlstream_core::ExcelDate { serial: 44927.0 }))
            .unwrap();
        let n = LookupValue::from_value(&Value::Number(44927.0)).unwrap();
        assert_eq!(d, n);
    }

    #[test]
    fn negative_zero_equals_positive_zero() {
        let neg = LookupValue::from_value(&Value::Number(-0.0)).unwrap();
        let pos = LookupValue::from_value(&Value::Number(0.0)).unwrap();
        assert_eq!(neg, pos);
    }

    #[test]
    fn number_ordering() {
        let a = LookupValue::from_value(&Value::Number(1.0)).unwrap();
        let b = LookupValue::from_value(&Value::Number(2.0)).unwrap();
        assert!(a < b);
    }

    #[test]
    fn different_types_use_tier_ordering() {
        let num = LookupValue::from_value(&Value::Number(999.0)).unwrap();
        let text = LookupValue::from_value(&Value::Text("a".into())).unwrap();
        let b = LookupValue::from_value(&Value::Bool(false)).unwrap();
        assert!(num < text);
        assert!(text < b);
    }
}
