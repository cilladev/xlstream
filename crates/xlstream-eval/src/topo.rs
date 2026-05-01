//! Topological sort for formula column evaluation order.
//!
//! Uses Kahn's algorithm on the formula-to-formula dependency graph.
//! Data columns (non-formula) are free — they're already in the row.

use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::BuildHasher;

use xlstream_core::XlStreamError;

/// Build evaluation order for formula columns using Kahn's algorithm.
///
/// `formula_deps`: `(col_index, vec_of_col_indices_it_references)`.
/// `formula_cols`: set of columns that contain formulas.
///
/// Only formula-to-formula edges matter. References to data columns are
/// free (already in the row) and are filtered out.
///
/// # Errors
///
/// Returns [`XlStreamError::CircularReference`] if the DAG has a cycle.
///
/// # Examples
///
/// ```
/// use std::collections::HashSet;
/// use xlstream_eval::topo_sort;
/// let deps = vec![(1u32, vec![0u32]), (2, vec![1])];
/// let formula_cols: HashSet<u32> = [1, 2].into_iter().collect();
/// let order = topo_sort(&deps, &formula_cols).unwrap();
/// assert_eq!(order, vec![1, 2]);
/// ```
pub fn topo_sort<S: BuildHasher>(
    formula_deps: &[(u32, Vec<u32>)],
    formula_cols: &HashSet<u32, S>,
) -> Result<Vec<u32>, XlStreamError> {
    // Initialise in-degree for every formula column.
    let mut in_degree: HashMap<u32, usize> = formula_cols.iter().map(|&c| (c, 0)).collect();
    let mut successors: HashMap<u32, Vec<u32>> = HashMap::new();

    for (col, deps) in formula_deps {
        for &dep in deps {
            if formula_cols.contains(&dep) {
                *in_degree.entry(*col).or_insert(0) += 1;
                successors.entry(dep).or_default().push(*col);
            }
        }
    }

    // Seed queue with zero-degree nodes, sorted for determinism.
    let mut initial: Vec<u32> =
        in_degree.iter().filter(|(_, &deg)| deg == 0).map(|(&col, _)| col).collect();
    initial.sort_unstable();

    let mut queue: VecDeque<u32> = VecDeque::from(initial);
    let mut order = Vec::with_capacity(formula_cols.len());

    while let Some(col) = queue.pop_front() {
        order.push(col);
        if let Some(succs) = successors.get(&col) {
            // Sort successors for deterministic ordering when multiple
            // nodes become ready at the same time.
            let mut sorted_succs: Vec<u32> = succs.clone();
            sorted_succs.sort_unstable();
            for s in sorted_succs {
                if let Some(deg) = in_degree.get_mut(&s) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(s);
                    }
                }
            }
        }
    }

    if order.len() != formula_cols.len() {
        let mut remaining: Vec<String> = formula_cols
            .iter()
            .filter(|c| !order.contains(c))
            .map(|c| format!("col {c}"))
            .collect();
        remaining.sort();
        return Err(XlStreamError::CircularReference { cells: remaining });
    }

    Ok(order)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use std::collections::HashSet;

    use xlstream_core::XlStreamError;

    use super::*;

    #[test]
    fn linear_chain() {
        // col 0 = data, col 1 (formula) deps col 0, col 2 (formula) deps col 1
        let deps = vec![(1u32, vec![0u32]), (2, vec![1])];
        let formula_cols: HashSet<u32> = [1, 2].into_iter().collect();
        let order = topo_sort(&deps, &formula_cols).unwrap();
        assert_eq!(order, vec![1, 2]);
    }

    #[test]
    fn diamond() {
        // A(0) = data, B(1) deps A, C(2) deps A, D(3) deps B+C
        let deps = vec![(1u32, vec![0u32]), (2, vec![0]), (3, vec![1, 2])];
        let formula_cols: HashSet<u32> = [1, 2, 3].into_iter().collect();
        let order = topo_sort(&deps, &formula_cols).unwrap();
        // B and C before D; sorted tie-breaking gives [1, 2, 3]
        let d_pos = order.iter().position(|&c| c == 3).unwrap();
        let b_pos = order.iter().position(|&c| c == 1).unwrap();
        let c_pos = order.iter().position(|&c| c == 2).unwrap();
        assert!(b_pos < d_pos, "B must come before D");
        assert!(c_pos < d_pos, "C must come before D");
    }

    #[test]
    fn cycle_returns_error() {
        // B deps C, C deps B — cycle
        let deps = vec![(1u32, vec![2u32]), (2, vec![1])];
        let formula_cols: HashSet<u32> = [1, 2].into_iter().collect();
        let err = topo_sort(&deps, &formula_cols).unwrap_err();
        assert!(
            matches!(err, XlStreamError::CircularReference { .. }),
            "expected CircularReference, got {err:?}",
        );
    }

    #[test]
    fn independent_columns() {
        // Two formula columns with no inter-formula deps
        let deps = vec![(1u32, vec![0u32]), (2, vec![0])];
        let formula_cols: HashSet<u32> = [1, 2].into_iter().collect();
        let order = topo_sort(&deps, &formula_cols).unwrap();
        assert_eq!(order.len(), 2);
        assert!(order.contains(&1));
        assert!(order.contains(&2));
    }

    #[test]
    fn single_formula_column() {
        let deps = vec![(1u32, vec![0u32])];
        let formula_cols: HashSet<u32> = [1].into_iter().collect();
        let order = topo_sort(&deps, &formula_cols).unwrap();
        assert_eq!(order, vec![1]);
    }

    #[test]
    fn empty_input() {
        let deps: Vec<(u32, Vec<u32>)> = vec![];
        let formula_cols: HashSet<u32> = HashSet::new();
        let order = topo_sort(&deps, &formula_cols).unwrap();
        assert!(order.is_empty());
    }

    #[test]
    fn self_cycle_returns_error() {
        let deps = vec![(1u32, vec![1u32])];
        let formula_cols: HashSet<u32> = [1].into_iter().collect();
        let err = topo_sort(&deps, &formula_cols).unwrap_err();
        assert!(matches!(err, XlStreamError::CircularReference { .. }));
    }

    #[test]
    fn self_edge_filtered_before_sort_succeeds() {
        let deps = vec![(1u32, vec![0u32])];
        let formula_cols: HashSet<u32> = [1].into_iter().collect();
        let order = topo_sort(&deps, &formula_cols).unwrap();
        assert_eq!(order, vec![1]);
    }

    #[test]
    fn self_edge_filtered_with_other_formula_dep() {
        let deps = vec![(1u32, vec![0u32]), (2, vec![1])];
        let formula_cols: HashSet<u32> = [1, 2].into_iter().collect();
        let order = topo_sort(&deps, &formula_cols).unwrap();
        assert_eq!(order, vec![1, 2]);
    }

    #[test]
    fn cross_column_same_row_circular_still_fails() {
        let deps = vec![(1u32, vec![2u32]), (2, vec![1])];
        let formula_cols: HashSet<u32> = [1, 2].into_iter().collect();
        let err = topo_sort(&deps, &formula_cols).unwrap_err();
        assert!(matches!(err, XlStreamError::CircularReference { .. }));
    }
}
