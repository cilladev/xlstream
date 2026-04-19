//! The [`evaluate`] entry point and [`EvaluateSummary`] return type.

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::Instant;

use xlstream_core::{col_row_to_a1, Value, XlStreamError};
use xlstream_io::{Reader, Writer};
use xlstream_parse::{
    classify, extract_references, parse, rewrite, Ast, Classification, ClassificationContext,
    Reference,
};

use crate::interp::Interpreter;
use crate::prelude::Prelude;
use crate::scope::RowScope;
use crate::topo::topo_sort;

/// Summary of a completed evaluation run.
///
/// # Examples
///
/// ```
/// use xlstream_eval::EvaluateSummary;
/// let s = EvaluateSummary::default();
/// assert_eq!(s.rows_processed, 0);
/// assert_eq!(s.formulas_evaluated, 0);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EvaluateSummary {
    /// Total rows written across all sheets (including header rows).
    pub rows_processed: u64,
    /// Total formula cells evaluated.
    pub formulas_evaluated: u64,
    /// Wall-clock duration of the full evaluation run.
    pub duration: std::time::Duration,
}

/// Evaluate every formula in `input`, write results to `output`, and return
/// an [`EvaluateSummary`].
///
/// Finds the first sheet that contains formulas, classifies them, builds an
/// intra-row topo evaluation order, then streams rows through the interpreter
/// and writes results. All other sheets are copied verbatim.
///
/// `workers` is reserved for Phase 10 row parallelism; currently ignored.
///
/// # Errors
///
/// - [`XlStreamError::Xlsx`] if `input` cannot be opened or parsed.
/// - [`XlStreamError::XlsxWrite`] if `output` cannot be written.
/// - [`XlStreamError::FormulaParse`] if a formula cannot be parsed.
/// - [`XlStreamError::Unsupported`] if a formula cannot be streamed.
/// - [`XlStreamError::CircularReference`] if formula columns form a cycle.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use xlstream_eval::evaluate;
/// let err = evaluate(Path::new("missing.xlsx"), Path::new("out.xlsx"), None).unwrap_err();
/// assert!(matches!(err, xlstream_core::XlStreamError::Xlsx(_)));
/// ```
#[must_use = "evaluation results must be inspected for errors"]
pub fn evaluate(
    input: &Path,
    output: &Path,
    _workers: Option<usize>,
) -> Result<EvaluateSummary, XlStreamError> {
    let start = Instant::now();

    let mut reader = Reader::open(input)?;
    let sheet_names = reader.sheet_names();

    // Find first sheet with formulas.
    let mut main_sheet: Option<String> = None;
    let mut main_formulas: Vec<(u32, u32, String)> = Vec::new();
    for name in &sheet_names {
        let formulas = reader.formulas(name)?;
        if !formulas.is_empty() {
            main_sheet = Some(name.clone());
            main_formulas = formulas;
            break;
        }
    }

    // Parse + classify formula columns; build topo order.
    let (topo_order, col_asts) = if let Some(ref main) = main_sheet {
        build_eval_plan(main, &main_formulas)?
    } else {
        (Vec::new(), HashMap::new())
    };

    // Collect aggregate keys from all rewritten ASTs and run prelude.
    let mut all_agg_keys = Vec::new();
    for ast in col_asts.values() {
        all_agg_keys.extend(crate::prelude_plan::collect_aggregate_keys(ast));
    }
    // Deduplicate keys.
    all_agg_keys.sort_by(|a, b| format!("{a:?}").cmp(&format!("{b:?}")));
    all_agg_keys.dedup();

    let prelude = if all_agg_keys.is_empty() {
        Prelude::empty()
    } else if let Some(ref main) = main_sheet {
        crate::prelude_plan::execute_prelude(&mut reader, main, &all_agg_keys)?
    } else {
        Prelude::empty()
    };
    let interp = Interpreter::new(&prelude);
    let mut writer = Writer::create(output)?;
    let mut rows_processed: u64 = 0;
    let mut formulas_evaluated: u64 = 0;

    for name in &sheet_names {
        let mut sh = writer.add_sheet(name)?;
        let mut stream = reader.cells(name)?;
        let is_main = main_sheet.as_deref() == Some(name.as_str());
        let mut header_pending = is_main;

        while let Some((row_idx, mut row_values)) = stream.next_row()? {
            if header_pending {
                // First row of the main sheet is headers — write verbatim.
                sh.write_row(row_idx, &row_values)?;
                header_pending = false;
                rows_processed += 1;
                continue;
            }

            if is_main {
                for &fcol in &topo_order {
                    let Some(ast) = col_asts.get(&fcol) else {
                        continue;
                    };
                    let fcol_idx = fcol as usize;
                    if fcol_idx >= row_values.len() {
                        row_values.resize(fcol_idx + 1, Value::Empty);
                    }
                    // Inner block ensures the immutable borrow of row_values
                    // through RowScope is released before the mutation below.
                    let result = {
                        let scope = RowScope::new(&row_values, row_idx);
                        interp.eval(ast.root(), &scope)
                    };
                    row_values[fcol_idx] = result;
                    formulas_evaluated += 1;
                }
            }

            sh.write_row(row_idx, &row_values)?;
            rows_processed += 1;
        }

        drop(sh);
    }

    writer.finish()?;

    Ok(EvaluateSummary { rows_processed, formulas_evaluated, duration: start.elapsed() })
}

/// Parse + classify all formula columns for `main_sheet`. Returns the
/// topo-sorted column evaluation order and the per-column [`Ast`] cache.
fn build_eval_plan(
    main_sheet: &str,
    all_formulas: &[(u32, u32, String)],
) -> Result<(Vec<u32>, HashMap<u32, Ast>), XlStreamError> {
    // First occurrence per column (0-based col → (0-based row, formula text)).
    let mut col_formula: HashMap<u32, (u32, String)> = HashMap::new();
    for (row, col, text) in all_formulas {
        col_formula.entry(*col).or_insert_with(|| (*row, text.clone()));
    }

    let mut col_asts: HashMap<u32, Ast> = HashMap::new();
    for (&col, (row, text)) in &col_formula {
        // calamine strips the leading '='; strip defensively to handle either.
        let formula_str = text.strip_prefix('=').unwrap_or(text.as_str());
        let ast = parse(formula_str)?;

        // Classify at first-occurrence position (convert 0-based to 1-based).
        let ctx = ClassificationContext::for_cell(main_sheet, row + 1, col + 1);
        let verdict = classify(&ast, &ctx);
        if let Classification::Unsupported(ref reason) = verdict {
            return Err(XlStreamError::Unsupported {
                address: format!("{}!{}", main_sheet, col_row_to_a1(col + 1, row + 1)),
                formula: text.clone(),
                reason: reason.to_string(),
                doc_link: reason.doc_link(),
            });
        }

        // Rewrite aggregate/lookup sub-expressions to PreludeRef nodes.
        let rewritten = rewrite(ast, &ctx, &verdict);
        col_asts.insert(col, rewritten);
    }

    // Build intra-row dependency graph; extract formula-column-to-formula-column edges.
    let formula_cols: HashSet<u32> = col_asts.keys().copied().collect();
    let deps: Vec<(u32, Vec<u32>)> = col_asts
        .iter()
        .map(|(&col, ast)| {
            let refs = extract_references(ast);
            let dep_cols: Vec<u32> = refs
                .cells
                .iter()
                .filter_map(|r| match r {
                    Reference::Cell { col: ref_col, .. } => {
                        // Convert 1-based reference to 0-based column index.
                        Some(ref_col.saturating_sub(1))
                    }
                    _ => None,
                })
                .collect();
            (col, dep_cols)
        })
        .collect();

    let topo_order = topo_sort(&deps, &formula_cols)?;
    Ok((topo_order, col_asts))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn summary_default_fields_are_zero() {
        let s = EvaluateSummary::default();
        assert_eq!(s.rows_processed, 0);
        assert_eq!(s.formulas_evaluated, 0);
        assert_eq!(s.duration, std::time::Duration::ZERO);
    }

    #[test]
    fn evaluate_nonexistent_file_returns_xlsx_error() {
        let err = evaluate(Path::new("this_file_does_not_exist.xlsx"), Path::new("out.xlsx"), None)
            .unwrap_err();
        assert!(matches!(err, XlStreamError::Xlsx(_)), "expected Xlsx error, got {err:?}");
    }
}
