//! Prelude plan: collect aggregate keys from rewritten ASTs and execute
//! the prelude pass to compute them.
//!
//! [`collect_aggregate_keys`] walks a rewritten AST for `PreludeRef`
//! nodes and returns the set of [`AggregateKey`]s needed. The
//! [`execute_prelude`] function streams the main sheet, folds each
//! column's values through [`FoldState`], and builds a filled
//! [`Prelude`].

use std::collections::HashMap;

use xlstream_core::{CellError, Value, XlStreamError};
use xlstream_io::Reader;
use xlstream_parse::{AggKind, AggregateKey, Ast, NodeView, PreludeKey};

use crate::prelude::Prelude;

/// Accumulator for streaming aggregate computation over a single column.
///
/// Fed values one at a time via [`feed`](FoldState::feed). After all
/// values are consumed, call [`finish`](FoldState::finish) with the
/// desired [`AggKind`] to extract the result.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_parse::AggKind;
/// use xlstream_eval::prelude_plan::FoldState;
///
/// let mut state = FoldState::new();
/// state.feed(&Value::Number(10.0));
/// state.feed(&Value::Number(20.0));
/// assert_eq!(state.finish(AggKind::Sum), Value::Number(30.0));
/// ```
#[derive(Debug, Clone)]
pub struct FoldState {
    sum: f64,
    count: u64,
    counta: u64,
    countblank: u64,
    min: Option<f64>,
    max: Option<f64>,
    product: f64,
    has_numeric: bool,
    /// Collected numeric values for median computation.
    nums: Vec<f64>,
    /// First error encountered.
    error: Option<CellError>,
}

impl FoldState {
    /// Create a new empty accumulator.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_eval::prelude_plan::FoldState;
    /// let state = FoldState::new();
    /// drop(state);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            sum: 0.0,
            count: 0,
            counta: 0,
            countblank: 0,
            min: None,
            max: None,
            product: 1.0,
            has_numeric: false,
            nums: Vec::new(),
            error: None,
        }
    }

    /// Feed a single cell value into the accumulator.
    ///
    /// Follows range semantics: numeric values accumulate, text and
    /// booleans are skipped for numeric aggregates but counted for
    /// COUNTA, errors are captured (first one wins).
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_core::Value;
    /// use xlstream_eval::prelude_plan::FoldState;
    ///
    /// let mut s = FoldState::new();
    /// s.feed(&Value::Number(5.0));
    /// s.feed(&Value::Text("x".into()));
    /// s.feed(&Value::Empty);
    /// ```
    pub fn feed(&mut self, v: &Value) {
        match v {
            Value::Error(e) => {
                if self.error.is_none() {
                    self.error = Some(*e);
                }
                // Errors count as non-empty for COUNTA
                self.counta += 1;
            }
            Value::Number(n) => {
                self.sum += n;
                self.count += 1;
                self.counta += 1;
                self.min = Some(self.min.map_or(*n, |cur| cur.min(*n)));
                self.max = Some(self.max.map_or(*n, |cur| cur.max(*n)));
                self.product *= n;
                self.has_numeric = true;
                self.nums.push(*n);
            }
            #[allow(clippy::cast_precision_loss)]
            Value::Integer(i) => {
                let f = *i as f64;
                self.sum += f;
                self.count += 1;
                self.counta += 1;
                self.min = Some(self.min.map_or(f, |cur| cur.min(f)));
                self.max = Some(self.max.map_or(f, |cur| cur.max(f)));
                self.product *= f;
                self.has_numeric = true;
                self.nums.push(f);
            }
            Value::Date(d) => {
                self.sum += d.serial;
                self.count += 1;
                self.counta += 1;
                self.min = Some(self.min.map_or(d.serial, |cur| cur.min(d.serial)));
                self.max = Some(self.max.map_or(d.serial, |cur| cur.max(d.serial)));
                self.product *= d.serial;
                self.has_numeric = true;
                self.nums.push(d.serial);
            }
            Value::Text(_) | Value::Bool(_) => {
                // Non-empty for COUNTA, skipped for numeric aggregates.
                self.counta += 1;
            }
            Value::Empty => {
                self.countblank += 1;
            }
        }
    }

    /// Produce the final aggregate value for the given kind.
    ///
    /// For error-propagating aggregates (SUM, AVERAGE, MIN, MAX,
    /// PRODUCT, MEDIAN), returns the first captured error if any.
    /// COUNT, COUNTA, and COUNTBLANK never propagate errors.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_core::Value;
    /// use xlstream_parse::AggKind;
    /// use xlstream_eval::prelude_plan::FoldState;
    ///
    /// let mut state = FoldState::new();
    /// state.feed(&Value::Number(3.0));
    /// state.feed(&Value::Number(7.0));
    /// assert_eq!(state.finish(AggKind::Average), Value::Number(5.0));
    /// ```
    #[must_use]
    pub fn finish(mut self, kind: AggKind) -> Value {
        match kind {
            AggKind::Sum => {
                if let Some(e) = self.error {
                    return Value::Error(e);
                }
                Value::Number(self.sum)
            }
            AggKind::Count =>
            {
                #[allow(clippy::cast_precision_loss)]
                Value::Number(self.count as f64)
            }
            AggKind::CountA =>
            {
                #[allow(clippy::cast_precision_loss)]
                Value::Number(self.counta as f64)
            }
            AggKind::CountBlank =>
            {
                #[allow(clippy::cast_precision_loss)]
                Value::Number(self.countblank as f64)
            }
            AggKind::Average => {
                if let Some(e) = self.error {
                    return Value::Error(e);
                }
                if self.count == 0 {
                    return Value::Error(CellError::Div0);
                }
                #[allow(clippy::cast_precision_loss)]
                Value::Number(self.sum / self.count as f64)
            }
            AggKind::Min => {
                if let Some(e) = self.error {
                    return Value::Error(e);
                }
                Value::Number(self.min.unwrap_or(0.0))
            }
            AggKind::Max => {
                if let Some(e) = self.error {
                    return Value::Error(e);
                }
                Value::Number(self.max.unwrap_or(0.0))
            }
            AggKind::Product => {
                if let Some(e) = self.error {
                    return Value::Error(e);
                }
                if self.has_numeric {
                    Value::Number(self.product)
                } else {
                    Value::Number(0.0)
                }
            }
            AggKind::Median => {
                if let Some(e) = self.error {
                    return Value::Error(e);
                }
                if self.nums.is_empty() {
                    return Value::Error(CellError::Num);
                }
                self.nums.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let mid = self.nums.len() / 2;
                if self.nums.len() % 2 == 0 {
                    Value::Number(f64::midpoint(self.nums[mid - 1], self.nums[mid]))
                } else {
                    Value::Number(self.nums[mid])
                }
            }
        }
    }
}

impl Default for FoldState {
    fn default() -> Self {
        Self::new()
    }
}

/// Walk a rewritten AST and collect all [`AggregateKey`]s referenced by
/// `PreludeRef(Aggregate(_))` nodes.
///
/// # Examples
///
/// ```
/// use xlstream_parse::{parse, classify, rewrite, ClassificationContext};
/// use xlstream_eval::prelude_plan::collect_aggregate_keys;
///
/// let ast = parse("SUM(A:A)").unwrap();
/// let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
/// let verdict = classify(&ast, &ctx);
/// let rewritten = rewrite(ast, &ctx, &verdict);
/// let keys = collect_aggregate_keys(&rewritten);
/// assert_eq!(keys.len(), 1);
/// ```
#[must_use]
pub fn collect_aggregate_keys(ast: &Ast) -> Vec<AggregateKey> {
    let mut keys = Vec::new();
    collect_keys_recursive(ast.root(), &mut keys);
    keys
}

fn collect_keys_recursive(node: xlstream_parse::NodeRef<'_>, keys: &mut Vec<AggregateKey>) {
    match node.view() {
        NodeView::PreludeRef(PreludeKey::Aggregate(agg_key)) => {
            keys.push(agg_key.clone());
        }
        NodeView::BinaryOp { .. } => {
            if let Some(left) = node.left() {
                collect_keys_recursive(left, keys);
            }
            if let Some(right) = node.right() {
                collect_keys_recursive(right, keys);
            }
        }
        NodeView::UnaryOp { .. } => {
            if let Some(operand) = node.operand() {
                collect_keys_recursive(operand, keys);
            }
        }
        NodeView::Function { .. } => {
            for arg in node.args() {
                collect_keys_recursive(arg, keys);
            }
        }
        NodeView::Array { .. } => {
            for row in node.array_cells() {
                for cell in row {
                    collect_keys_recursive(cell, keys);
                }
            }
        }
        _ => {}
    }
}

/// Execute the prelude pass: stream the main sheet, fold column values
/// through [`FoldState`] accumulators, and build a filled [`Prelude`].
///
/// Skips the first row (header). Each subsequent row feeds the relevant
/// column values into accumulators. After the stream is exhausted,
/// finishes each accumulator and stores the result.
///
/// # Errors
///
/// Returns `Err(XlStreamError::Xlsx)` if the sheet cannot be read.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use xlstream_io::Reader;
/// use xlstream_parse::{AggKind, AggregateKey};
/// use xlstream_eval::prelude_plan::execute_prelude;
///
/// let mut reader = Reader::open(Path::new("workbook.xlsx")).unwrap();
/// let keys = vec![AggregateKey { kind: AggKind::Sum, sheet: None, column: 1 }];
/// let prelude = execute_prelude(&mut reader, "Sheet1", &keys).unwrap();
/// ```
pub fn execute_prelude(
    reader: &mut Reader,
    main_sheet: &str,
    simple_keys: &[AggregateKey],
) -> Result<Prelude, XlStreamError> {
    if simple_keys.is_empty() {
        return Ok(Prelude::empty());
    }

    // Group keys by column — multiple agg kinds may target the same column.
    let mut col_kinds: HashMap<u32, Vec<(AggKind, AggregateKey)>> = HashMap::new();
    for key in simple_keys {
        col_kinds.entry(key.column).or_default().push((key.kind, key.clone()));
    }

    // One FoldState per column (shared across all kinds for that column).
    let mut col_folds: HashMap<u32, FoldState> = HashMap::new();
    for &col in col_kinds.keys() {
        col_folds.insert(col, FoldState::new());
    }

    // Stream the sheet, skipping header row.
    let mut stream = reader.cells(main_sheet)?;
    let mut header_skipped = false;

    while let Some((_row_idx, row_values)) = stream.next_row()? {
        if !header_skipped {
            header_skipped = true;
            continue;
        }

        for (&col, fold) in &mut col_folds {
            // col is 1-based
            let idx = (col as usize).saturating_sub(1);
            let val = row_values.get(idx).unwrap_or(&Value::Empty);
            fold.feed(val);
        }
    }

    // Finish each fold and produce aggregate results.
    let mut aggregates: HashMap<AggregateKey, Value> = HashMap::new();
    for (col, fold) in col_folds {
        let kinds = col_kinds.get(&col).map_or(&[][..], |v| v.as_slice());
        for (kind, key) in kinds {
            let result = fold.clone().finish(*kind);
            aggregates.insert(key.clone(), result);
        }
    }

    Ok(Prelude::with_aggregates(aggregates))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use xlstream_core::{CellError, Value};
    use xlstream_parse::AggKind;

    use super::*;

    // ===== FoldState: SUM =====

    #[test]
    fn fold_sum_numbers() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(1.0));
        s.feed(&Value::Number(2.0));
        s.feed(&Value::Number(3.0));
        assert_eq!(s.finish(AggKind::Sum), Value::Number(6.0));
    }

    #[test]
    fn fold_sum_empty() {
        let s = FoldState::new();
        assert_eq!(s.finish(AggKind::Sum), Value::Number(0.0));
    }

    #[test]
    fn fold_sum_skips_text() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(5.0));
        s.feed(&Value::Text("x".into()));
        assert_eq!(s.finish(AggKind::Sum), Value::Number(5.0));
    }

    #[test]
    fn fold_sum_propagates_error() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(1.0));
        s.feed(&Value::Error(CellError::Div0));
        assert_eq!(s.finish(AggKind::Sum), Value::Error(CellError::Div0));
    }

    // ===== FoldState: COUNT =====

    #[test]
    fn fold_count_numbers() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(1.0));
        s.feed(&Value::Text("x".into()));
        s.feed(&Value::Number(2.0));
        assert_eq!(s.finish(AggKind::Count), Value::Number(2.0));
    }

    #[test]
    fn fold_count_empty() {
        let s = FoldState::new();
        assert_eq!(s.finish(AggKind::Count), Value::Number(0.0));
    }

    // ===== FoldState: COUNTA =====

    #[test]
    fn fold_counta_counts_non_empty() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(1.0));
        s.feed(&Value::Empty);
        s.feed(&Value::Text("x".into()));
        s.feed(&Value::Bool(true));
        assert_eq!(s.finish(AggKind::CountA), Value::Number(3.0));
    }

    #[test]
    fn fold_counta_counts_errors() {
        let mut s = FoldState::new();
        s.feed(&Value::Error(CellError::Na));
        s.feed(&Value::Empty);
        assert_eq!(s.finish(AggKind::CountA), Value::Number(1.0));
    }

    // ===== FoldState: COUNTBLANK =====

    #[test]
    fn fold_countblank_counts_empty() {
        let mut s = FoldState::new();
        s.feed(&Value::Empty);
        s.feed(&Value::Number(0.0));
        s.feed(&Value::Empty);
        assert_eq!(s.finish(AggKind::CountBlank), Value::Number(2.0));
    }

    // ===== FoldState: AVERAGE =====

    #[test]
    fn fold_average_numbers() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(10.0));
        s.feed(&Value::Number(20.0));
        assert_eq!(s.finish(AggKind::Average), Value::Number(15.0));
    }

    #[test]
    fn fold_average_empty_returns_div0() {
        let s = FoldState::new();
        assert_eq!(s.finish(AggKind::Average), Value::Error(CellError::Div0));
    }

    #[test]
    fn fold_average_propagates_error() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(1.0));
        s.feed(&Value::Error(CellError::Na));
        assert_eq!(s.finish(AggKind::Average), Value::Error(CellError::Na));
    }

    // ===== FoldState: MIN =====

    #[test]
    fn fold_min_numbers() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(5.0));
        s.feed(&Value::Number(1.0));
        s.feed(&Value::Number(3.0));
        assert_eq!(s.finish(AggKind::Min), Value::Number(1.0));
    }

    #[test]
    fn fold_min_empty_returns_zero() {
        let s = FoldState::new();
        assert_eq!(s.finish(AggKind::Min), Value::Number(0.0));
    }

    // ===== FoldState: MAX =====

    #[test]
    fn fold_max_numbers() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(5.0));
        s.feed(&Value::Number(1.0));
        s.feed(&Value::Number(9.0));
        assert_eq!(s.finish(AggKind::Max), Value::Number(9.0));
    }

    #[test]
    fn fold_max_propagates_error() {
        let mut s = FoldState::new();
        s.feed(&Value::Error(CellError::Ref));
        assert_eq!(s.finish(AggKind::Max), Value::Error(CellError::Ref));
    }

    // ===== FoldState: PRODUCT =====

    #[test]
    fn fold_product_numbers() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(2.0));
        s.feed(&Value::Number(3.0));
        s.feed(&Value::Number(4.0));
        assert_eq!(s.finish(AggKind::Product), Value::Number(24.0));
    }

    #[test]
    fn fold_product_empty_returns_zero() {
        let s = FoldState::new();
        assert_eq!(s.finish(AggKind::Product), Value::Number(0.0));
    }

    #[test]
    fn fold_product_propagates_error() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(2.0));
        s.feed(&Value::Error(CellError::Num));
        assert_eq!(s.finish(AggKind::Product), Value::Error(CellError::Num));
    }

    // ===== FoldState: MEDIAN =====

    #[test]
    fn fold_median_odd() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(3.0));
        s.feed(&Value::Number(1.0));
        s.feed(&Value::Number(2.0));
        assert_eq!(s.finish(AggKind::Median), Value::Number(2.0));
    }

    #[test]
    fn fold_median_even() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(1.0));
        s.feed(&Value::Number(2.0));
        s.feed(&Value::Number(3.0));
        s.feed(&Value::Number(4.0));
        assert_eq!(s.finish(AggKind::Median), Value::Number(2.5));
    }

    #[test]
    fn fold_median_empty_returns_num_error() {
        let s = FoldState::new();
        assert_eq!(s.finish(AggKind::Median), Value::Error(CellError::Num));
    }

    #[test]
    fn fold_median_propagates_error() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(1.0));
        s.feed(&Value::Error(CellError::Na));
        assert_eq!(s.finish(AggKind::Median), Value::Error(CellError::Na));
    }

    #[test]
    fn fold_skips_bool_for_numeric_aggs() {
        let mut s = FoldState::new();
        s.feed(&Value::Number(10.0));
        s.feed(&Value::Bool(true));
        assert_eq!(s.finish(AggKind::Sum), Value::Number(10.0));
    }

    // ===== collect_aggregate_keys =====

    #[test]
    fn collect_keys_from_rewritten_sum() {
        let ast = xlstream_parse::parse("SUM(A:A)").unwrap();
        let ctx = xlstream_parse::ClassificationContext::for_cell("Sheet1", 2, 5);
        let verdict = xlstream_parse::classify(&ast, &ctx);
        let rewritten = xlstream_parse::rewrite(ast, &ctx, &verdict);
        let keys = collect_aggregate_keys(&rewritten);
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].kind, AggKind::Sum);
        assert_eq!(keys[0].column, 1);
    }

    #[test]
    fn collect_keys_empty_for_row_local() {
        let ast = xlstream_parse::parse("A1+B1").unwrap();
        let ctx = xlstream_parse::ClassificationContext::for_cell("Sheet1", 1, 3);
        let verdict = xlstream_parse::classify(&ast, &ctx);
        let rewritten = xlstream_parse::rewrite(ast, &ctx, &verdict);
        let keys = collect_aggregate_keys(&rewritten);
        assert!(keys.is_empty());
    }
}
