//! Named range resolution — rewrites `Reference::Named` nodes in the AST
//! by looking up workbook-level defined names and parsing their values.

use std::collections::{HashMap, HashSet};
use std::hash::BuildHasher;

use crate::ast::{Ast, Node};
use crate::references::Reference;

/// Resolve all named range references in `ast` using the provided
/// name-to-value map.
///
/// Names are matched case-insensitively. The `names` map must use
/// lowercase keys. Unresolved names (not in `names`) are left as
/// `Reference::Named` for the classifier to reject.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use xlstream_parse::{parse, resolve_named_ranges};
///
/// let ast = parse("SUM(SalesData)").unwrap();
/// let mut names = HashMap::new();
/// names.insert("salesdata".to_string(), "Sheet1!$A:$A".to_string());
/// let resolved = resolve_named_ranges(ast, &names);
/// let dbg = format!("{resolved:?}");
/// assert!(dbg.contains("Range"), "expected resolved range in: {dbg}");
/// ```
#[must_use]
pub fn resolve_named_ranges<S: BuildHasher>(ast: Ast, names: &HashMap<String, String, S>) -> Ast {
    if names.is_empty() {
        return ast;
    }
    let mut resolving = HashSet::new();
    let new_root = resolve_node(ast.root, names, &mut resolving);
    Ast { upstream: ast.upstream, root: new_root }
}

fn resolve_node<S: BuildHasher>(
    node: Node,
    names: &HashMap<String, String, S>,
    resolving: &mut HashSet<String>,
) -> Node {
    match node {
        Node::Reference(Reference::Named(ref name)) => {
            let key = name.to_ascii_lowercase();
            if resolving.contains(&key) {
                return node;
            }
            match names.get(&key) {
                Some(value) => {
                    resolving.insert(key.clone());
                    let resolved = match crate::parse(value) {
                        Ok(parsed) => resolve_node(parsed.root, names, resolving),
                        Err(_) => node,
                    };
                    resolving.remove(&key);
                    resolved
                }
                None => node,
            }
        }
        Node::Function { name, args } => Node::Function {
            name,
            args: args.into_iter().map(|a| resolve_node(a, names, resolving)).collect(),
        },
        Node::BinaryOp { op, left, right } => Node::BinaryOp {
            op,
            left: Box::new(resolve_node(*left, names, resolving)),
            right: Box::new(resolve_node(*right, names, resolving)),
        },
        Node::UnaryOp { op, expr } => {
            Node::UnaryOp { op, expr: Box::new(resolve_node(*expr, names, resolving)) }
        }
        Node::Array(rows) => Node::Array(
            rows.into_iter()
                .map(|row| row.into_iter().map(|n| resolve_node(n, names, resolving)).collect())
                .collect(),
        ),
        other @ (Node::Literal(_)
        | Node::Text(_)
        | Node::Error(_)
        | Node::Reference(_)
        | Node::PreludeRef(_)) => other,
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    fn names(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_ascii_lowercase(), (*v).to_string())).collect()
    }

    fn resolve(formula: &str, name_pairs: &[(&str, &str)]) -> Ast {
        let ast = crate::parse(formula).unwrap();
        resolve_named_ranges(ast, &names(name_pairs))
    }

    fn dbg_root(ast: &Ast) -> String {
        format!("{:?}", ast.root)
    }

    // -- Range resolution --

    #[test]
    fn range_named_range_resolves_to_range() {
        let ast = resolve("SUM(SalesData)", &[("SalesData", "Sheet1!$A:$A")]);
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Range"), "expected Range in: {dbg}");
        assert!(!dbg.contains("Named"), "should not contain Named: {dbg}");
    }

    #[test]
    fn bounded_range_resolves() {
        let ast = resolve("SUM(MyRange)", &[("MyRange", "Sheet1!$A$1:$A$100")]);
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Range"), "expected Range: {dbg}");
    }

    // -- Cell resolution --

    #[test]
    fn cell_named_range_resolves_to_cell() {
        let ast = resolve("MyCell+1", &[("MyCell", "Sheet1!$A$1")]);
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Cell"), "expected Cell in: {dbg}");
        assert!(!dbg.contains("Named"), "should not contain Named: {dbg}");
    }

    // -- Constant resolution --

    #[test]
    fn numeric_constant_resolves_to_literal() {
        let ast = resolve("TaxRate*B2", &[("TaxRate", "0.15")]);
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("0.15") || dbg.contains("Literal"), "expected literal: {dbg}");
        assert!(!dbg.contains("Named"), "should not contain Named: {dbg}");
    }

    #[test]
    fn text_constant_resolves_to_text() {
        let ast = resolve("TaxRate", &[("TaxRate", "\"EMEA\"")]);
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Text") || dbg.contains("EMEA"), "expected text: {dbg}");
    }

    // -- Case insensitivity --

    #[test]
    fn case_insensitive_lookup() {
        let ast = resolve("SUM(salesdata)", &[("SalesData", "Sheet1!$A:$A")]);
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Range"), "case-insensitive lookup failed: {dbg}");
    }

    #[test]
    fn uppercase_lookup() {
        let ast = resolve("SUM(SALESDATA)", &[("SalesData", "Sheet1!$A:$A")]);
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Range"), "uppercase lookup failed: {dbg}");
    }

    // -- Unknown / missing --

    #[test]
    fn unknown_name_stays_as_named() {
        let ast = resolve("SUM(UnknownRange)", &[("SalesData", "Sheet1!$A:$A")]);
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Named"), "unknown should stay Named: {dbg}");
    }

    #[test]
    fn empty_names_map_is_noop() {
        let original = crate::parse("SUM(SalesData)").unwrap();
        let resolved = resolve_named_ranges(original.clone(), &HashMap::new());
        assert_eq!(format!("{:?}", original.root), format!("{:?}", resolved.root));
    }

    // -- Nested formulas --

    #[test]
    fn multiple_named_ranges_in_one_formula() {
        let ast = resolve(
            "SUMIF(RegionCol, \"EMEA\", AmountCol)",
            &[("RegionCol", "Sheet1!$A:$A"), ("AmountCol", "Sheet1!$B:$B")],
        );
        let dbg = dbg_root(&ast);
        assert!(!dbg.contains("Named"), "both should resolve: {dbg}");
    }

    #[test]
    fn named_range_inside_if() {
        let ast = resolve("IF(A2>0, SUM(SalesData), 0)", &[("SalesData", "Sheet1!$A:$A")]);
        let dbg = dbg_root(&ast);
        assert!(!dbg.contains("Named"), "nested named ref should resolve: {dbg}");
    }

    // -- Circular --

    #[test]
    fn circular_named_range_does_not_loop() {
        let ast = resolve("SelfRef", &[("SelfRef", "SelfRef")]);
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Named"), "circular should stay Named: {dbg}");
    }

    // -- Cross-sheet --

    #[test]
    fn cross_sheet_range_resolves() {
        let ast = resolve("SUM(RegionData)", &[("RegionData", "Lookup!$B:$B")]);
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Range"), "expected Range: {dbg}");
        assert!(dbg.contains("Lookup"), "expected sheet name 'Lookup': {dbg}");
    }

    // -- Sheet name with spaces --

    #[test]
    fn sheet_name_with_spaces_resolves() {
        let ast = resolve("SUM(SpaceName)", &[("SpaceName", "'Sheet Name'!$A:$A")]);
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Range"), "expected Range: {dbg}");
        assert!(dbg.contains("Sheet Name"), "expected sheet name: {dbg}");
    }

    // -- Computed formula as named range value --

    #[test]
    fn formula_value_named_range_resolves() {
        let ast = resolve("MyTotal", &[("MyTotal", "SUM(A1:A10)")]);
        let dbg = dbg_root(&ast);
        assert!(dbg.contains("Function"), "expected Function node: {dbg}");
        assert!(!dbg.contains("Named"), "should not contain Named: {dbg}");
    }
}
