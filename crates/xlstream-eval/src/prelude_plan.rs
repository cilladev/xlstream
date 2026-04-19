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
#[derive(Debug, Clone)]
pub(crate) struct FoldState {
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
    #[must_use]
    pub(crate) fn new() -> Self {
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
    pub(crate) fn feed(&mut self, v: &Value) {
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
    #[must_use]
    pub(crate) fn finish(mut self, kind: AggKind) -> Value {
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

/// Walk a rewritten AST and collect all [`MultiConditionalAggKey`]s
/// needed by SUMIFS, COUNTIFS, AVERAGEIFS function nodes.
#[must_use]
pub(crate) fn collect_multi_conditional_keys(
    ast: &Ast,
) -> Vec<crate::prelude::MultiConditionalAggKey> {
    let mut keys = Vec::new();
    collect_multi_keys_recursive(ast.root(), &mut keys);
    keys
}

fn collect_multi_keys_recursive(
    node: xlstream_parse::NodeRef<'_>,
    keys: &mut Vec<crate::prelude::MultiConditionalAggKey>,
) {
    match node.view() {
        NodeView::Function { name } => {
            let upper = name.to_ascii_uppercase();
            let normalized = upper.strip_prefix("_XLFN.").unwrap_or(&upper);
            match normalized {
                "SUMIFS" => {
                    if let Some(key) = extract_sumifs_key(node) {
                        keys.push(key);
                    }
                }
                "COUNTIFS" => {
                    if let Some(key) = extract_countifs_key(node) {
                        keys.push(key);
                    }
                }
                "AVERAGEIFS" => {
                    if let Some(key) = extract_averageifs_key(node) {
                        keys.push(key);
                    }
                }
                _ => {}
            }
            // Also recurse into args in case of nested function calls.
            for arg in node.args() {
                collect_multi_keys_recursive(arg, keys);
            }
        }
        NodeView::BinaryOp { .. } => {
            if let Some(left) = node.left() {
                collect_multi_keys_recursive(left, keys);
            }
            if let Some(right) = node.right() {
                collect_multi_keys_recursive(right, keys);
            }
        }
        NodeView::UnaryOp { .. } => {
            if let Some(operand) = node.operand() {
                collect_multi_keys_recursive(operand, keys);
            }
        }
        NodeView::Array { .. } => {
            for row in node.array_cells() {
                for cell in row {
                    collect_multi_keys_recursive(cell, keys);
                }
            }
        }
        _ => {}
    }
}

/// Extract column from a range-ref node (whole-column like `A:A`).
fn extract_range_col(node: xlstream_parse::NodeRef<'_>) -> Option<u32> {
    match node.view() {
        NodeView::RangeRef { start_col: Some(sc), end_col: Some(ec), .. } if sc == ec => Some(sc),
        _ => None,
    }
}

/// Extract a `MultiConditionalAggKey` from a SUMIFS function node.
/// `SUMIFS(sum_range, crit_range1, crit1, crit_range2, crit2, ...)`
fn extract_sumifs_key(
    node: xlstream_parse::NodeRef<'_>,
) -> Option<crate::prelude::MultiConditionalAggKey> {
    let args = node.args();
    if args.len() < 3 || (args.len() - 1) % 2 != 0 {
        return None;
    }
    let sum_col = extract_range_col(args[0])?;
    let num_pairs = (args.len() - 1) / 2;
    let mut criteria_cols = Vec::with_capacity(num_pairs);
    for i in 0..num_pairs {
        let col = extract_range_col(args[1 + i * 2])?;
        criteria_cols.push(col);
    }
    Some(crate::prelude::MultiConditionalAggKey {
        kind: AggKind::Sum,
        sum_col,
        criteria_cols,
        sheet: None,
    })
}

/// Extract a `MultiConditionalAggKey` from a COUNTIFS function node.
/// `COUNTIFS(crit_range1, crit1, crit_range2, crit2, ...)`
fn extract_countifs_key(
    node: xlstream_parse::NodeRef<'_>,
) -> Option<crate::prelude::MultiConditionalAggKey> {
    let args = node.args();
    if args.len() < 2 || args.len() % 2 != 0 {
        return None;
    }
    let num_pairs = args.len() / 2;
    let mut criteria_cols = Vec::with_capacity(num_pairs);
    for i in 0..num_pairs {
        let col = extract_range_col(args[i * 2])?;
        criteria_cols.push(col);
    }
    Some(crate::prelude::MultiConditionalAggKey {
        kind: AggKind::Count,
        sum_col: 0,
        criteria_cols,
        sheet: None,
    })
}

/// Extract a `MultiConditionalAggKey` from an AVERAGEIFS function node.
/// `AVERAGEIFS(avg_range, crit_range1, crit1, crit_range2, crit2, ...)`
fn extract_averageifs_key(
    node: xlstream_parse::NodeRef<'_>,
) -> Option<crate::prelude::MultiConditionalAggKey> {
    let args = node.args();
    if args.len() < 3 || (args.len() - 1) % 2 != 0 {
        return None;
    }
    let sum_col = extract_range_col(args[0])?;
    let num_pairs = (args.len() - 1) / 2;
    let mut criteria_cols = Vec::with_capacity(num_pairs);
    for i in 0..num_pairs {
        let col = extract_range_col(args[1 + i * 2])?;
        criteria_cols.push(col);
    }
    Some(crate::prelude::MultiConditionalAggKey {
        kind: AggKind::Average,
        sum_col,
        criteria_cols,
        sheet: None,
    })
}

/// Walk an AST and collect all [`BoundedRangeKey`]s referenced by
/// `RangeRef` nodes with bounded rows and a single column.
///
/// These are bounded ranges inside range-expanding functions that
/// survive the rewrite pass (they are NOT rewritten to `PreludeRef`).
///
/// # Examples
///
/// ```
/// use xlstream_parse::parse;
/// use xlstream_eval::prelude_plan::collect_bounded_range_keys;
///
/// let ast = parse("IRR(A2:A10)").unwrap();
/// let keys = collect_bounded_range_keys(&ast);
/// assert_eq!(keys.len(), 1);
/// assert_eq!(keys[0].col, 1);
/// assert_eq!(keys[0].start_row, 2);
/// assert_eq!(keys[0].end_row, 10);
/// ```
#[must_use]
pub fn collect_bounded_range_keys(ast: &Ast) -> Vec<crate::prelude::BoundedRangeKey> {
    let mut keys = Vec::new();
    collect_range_keys_recursive(ast.root(), &mut keys);
    keys
}

fn collect_range_keys_recursive(
    node: xlstream_parse::NodeRef<'_>,
    keys: &mut Vec<crate::prelude::BoundedRangeKey>,
) {
    match node.view() {
        NodeView::RangeRef {
            sheet,
            start_row: Some(sr),
            end_row: Some(er),
            start_col: Some(sc),
            end_col: Some(ec),
        } if sc == ec => {
            keys.push(crate::prelude::BoundedRangeKey {
                sheet: sheet.map(ToString::to_string),
                col: sc,
                start_row: sr,
                end_row: er,
            });
        }
        NodeView::BinaryOp { .. } => {
            if let Some(left) = node.left() {
                collect_range_keys_recursive(left, keys);
            }
            if let Some(right) = node.right() {
                collect_range_keys_recursive(right, keys);
            }
        }
        NodeView::UnaryOp { .. } => {
            if let Some(operand) = node.operand() {
                collect_range_keys_recursive(operand, keys);
            }
        }
        NodeView::Function { .. } => {
            for arg in node.args() {
                collect_range_keys_recursive(arg, keys);
            }
        }
        NodeView::Array { .. } => {
            for row in node.array_cells() {
                for cell in row {
                    collect_range_keys_recursive(cell, keys);
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
/// Also computes multi-criteria conditional aggregate groupby tables
/// for SUMIFS/COUNTIFS/AVERAGEIFS, and collects bounded range values
/// for range-expanding functions.
///
/// Returns `(prelude, data_row_count)` where `data_row_count` is the
/// number of non-header rows streamed during the prelude pass.
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
/// let (prelude, _count) = execute_prelude(&mut reader, "Sheet1", &keys, &[], &[]).unwrap();
/// ```
#[allow(clippy::too_many_lines)]
pub fn execute_prelude(
    reader: &mut Reader,
    main_sheet: &str,
    simple_keys: &[AggregateKey],
    multi_keys: &[crate::prelude::MultiConditionalAggKey],
    range_keys: &[crate::prelude::BoundedRangeKey],
) -> Result<(Prelude, u32), XlStreamError> {
    if simple_keys.is_empty() && multi_keys.is_empty() && range_keys.is_empty() {
        return Ok((Prelude::empty(), 0));
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

    // Collect all columns needed for multi-conditional keys.
    let mut multi_needed_cols: std::collections::HashSet<u32> = std::collections::HashSet::new();
    for mk in multi_keys {
        if mk.sum_col > 0 {
            multi_needed_cols.insert(mk.sum_col);
        }
        for &c in &mk.criteria_cols {
            multi_needed_cols.insert(c);
        }
    }

    // Multi-conditional accumulators: for each key, a map from composite
    // criteria key to FoldState.
    let mut multi_folds: HashMap<usize, HashMap<String, FoldState>> = HashMap::new();
    for i in 0..multi_keys.len() {
        multi_folds.insert(i, HashMap::new());
    }

    // Initialize bounded range collectors.
    let mut range_collectors: HashMap<crate::prelude::BoundedRangeKey, Vec<Value>> = HashMap::new();
    for rk in range_keys {
        let capacity = (rk.end_row.saturating_sub(rk.start_row) + 1) as usize;
        range_collectors.insert(rk.clone(), Vec::with_capacity(capacity));
    }

    // Stream the sheet, skipping header row.
    let mut stream = reader.cells(main_sheet)?;
    let mut header_skipped = false;
    let mut calamine_row_idx: u32 = 0;
    let mut data_row_count: u32 = 0;

    while let Some((_row_idx, row_values)) = stream.next_row()? {
        let excel_row = calamine_row_idx + 1; // 1-based
        calamine_row_idx += 1;

        if !header_skipped {
            header_skipped = true;
            continue;
        }

        data_row_count = data_row_count.saturating_add(1);

        // Feed simple aggregate folds.
        for (&col, fold) in &mut col_folds {
            let idx = (col as usize).saturating_sub(1);
            let val = row_values.get(idx).unwrap_or(&Value::Empty);
            fold.feed(val);
        }

        // Collect bounded range values.
        for (rk, collector) in &mut range_collectors {
            if excel_row >= rk.start_row && excel_row <= rk.end_row {
                let idx = (rk.col as usize).saturating_sub(1);
                let val = row_values.get(idx).cloned().unwrap_or(Value::Empty);
                collector.push(val);
            }
        }

        // Feed multi-conditional folds.
        for (i, mk) in multi_keys.iter().enumerate() {
            // Build composite key from criteria columns.
            let mut composite_parts: Vec<String> = Vec::with_capacity(mk.criteria_cols.len());
            for &cc in &mk.criteria_cols {
                let idx = (cc as usize).saturating_sub(1);
                let val = row_values.get(idx).unwrap_or(&Value::Empty);
                composite_parts.push(xlstream_core::coerce::to_text(val).to_ascii_lowercase());
            }
            let composite = composite_parts.join("\0");

            // Get the sum/avg column value (for COUNTIFS, sum_col=0 so we
            // feed a Number(1.0) to count).
            let feed_val = if mk.sum_col > 0 {
                let idx = (mk.sum_col as usize).saturating_sub(1);
                row_values.get(idx).unwrap_or(&Value::Empty)
            } else {
                // COUNTIFS: count rows, feed 1.0
                &Value::Number(1.0)
            };

            // Pre-populated for all indices in the loop above.
            let Some(folds_map) = multi_folds.get_mut(&i) else {
                continue;
            };
            folds_map.entry(composite).or_insert_with(FoldState::new).feed(feed_val);
        }
    }

    // Finish simple aggregate folds.
    let mut aggregates: HashMap<AggregateKey, Value> = HashMap::new();
    for (col, fold) in col_folds {
        let kinds = col_kinds.get(&col).map_or(&[][..], |v| v.as_slice());
        for (kind, key) in kinds {
            let result = fold.clone().finish(*kind);
            aggregates.insert(key.clone(), result);
        }
    }

    // Finish multi-conditional folds.
    let mut multi_aggs: HashMap<crate::prelude::MultiConditionalAggKey, HashMap<String, Value>> =
        HashMap::new();
    for (i, mk) in multi_keys.iter().enumerate() {
        // COUNTIFS feeds 1.0 per matching row, so finishing with Sum
        // yields the count.
        let finish_kind = match mk.kind {
            AggKind::Count | AggKind::Sum => AggKind::Sum,
            AggKind::Average => AggKind::Average,
            other => other,
        };
        let folds_map = multi_folds.remove(&i).unwrap_or_default();
        let mut inner: HashMap<String, Value> = HashMap::new();
        for (composite_key, fold) in folds_map {
            inner.insert(composite_key, fold.finish(finish_kind));
        }
        multi_aggs.insert(mk.clone(), inner);
    }

    let prelude = if multi_aggs.is_empty() {
        Prelude::with_aggregates(aggregates)
    } else {
        Prelude::with_all(aggregates, HashMap::new(), multi_aggs)
    };

    if range_collectors.is_empty() {
        Ok((prelude, data_row_count))
    } else {
        Ok((prelude.with_cached_ranges(range_collectors), data_row_count))
    }
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
