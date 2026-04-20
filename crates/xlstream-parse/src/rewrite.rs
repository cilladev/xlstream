//! AST rewrite: replace aggregate/lookup sub-expressions with
//! `PreludeRef` nodes keyed by
//! [`PreludeKey`].

use crate::ast::{Ast, Node, NumLiteral};
use crate::classify::{Classification, ClassificationContext};
use crate::references::Reference;

/// Which aggregate operation a prelude pass must compute.
///
/// # Examples
///
/// ```
/// use xlstream_parse::AggKind;
/// let k = AggKind::Sum;
/// assert_eq!(k, AggKind::Sum);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AggKind {
    /// `SUM`
    Sum,
    /// `COUNT`
    Count,
    /// `COUNTA`
    CountA,
    /// `AVERAGE`
    Average,
    /// `MIN`
    Min,
    /// `MAX`
    Max,
    /// `PRODUCT`
    Product,
    /// `MEDIAN`
    Median,
    /// `COUNTBLANK`
    CountBlank,
}

/// Which lookup strategy the prelude must prepare.
///
/// # Examples
///
/// ```
/// use xlstream_parse::LookupKind;
/// let k = LookupKind::VLookup;
/// assert_eq!(k, LookupKind::VLookup);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LookupKind {
    /// `VLOOKUP`
    VLookup,
    /// `HLOOKUP`
    HLookup,
    /// `XLOOKUP`
    XLookup,
    /// `INDEX` + `MATCH` combo (reserved for Phase 8; not constructed today).
    IndexMatch,
}

/// Key identifying a single prelude-computed aggregate scalar.
///
/// # Examples
///
/// ```
/// use xlstream_parse::{AggKind, AggregateKey};
/// let k = AggregateKey { kind: AggKind::Sum, sheet: None, column: 1 };
/// assert_eq!(k.column, 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AggregateKey {
    /// The aggregate function.
    pub kind: AggKind,
    /// Sheet name (`None` = current streaming sheet).
    pub sheet: Option<String>,
    /// 1-based column index.
    pub column: u32,
}

/// Key identifying a prelude-loaded lookup index.
///
/// `key_index` and `value_index` are 1-based. For `VLookup`/`XLookup`
/// they are column indices; for `HLookup` they are row indices.
///
/// # Examples
///
/// ```
/// use xlstream_parse::{LookupKind, LookupKey};
/// let k = LookupKey {
///     kind: LookupKind::VLookup,
///     sheet: "Region Info".into(),
///     key_index: 1,
///     value_index: 2,
/// };
/// assert_eq!(k.sheet, "Region Info");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LookupKey {
    /// The lookup strategy.
    pub kind: LookupKind,
    /// Sheet name the lookup reads from.
    pub sheet: String,
    /// 1-based index of the search key (column for VLOOKUP, row for HLOOKUP).
    pub key_index: u32,
    /// 1-based index of the return value (column for VLOOKUP, row for HLOOKUP).
    pub value_index: u32,
}

/// Discriminant for prelude data: either an aggregate scalar or a lookup
/// index.
///
/// # Examples
///
/// ```
/// use xlstream_parse::{AggKind, AggregateKey, PreludeKey};
/// let pk = PreludeKey::Aggregate(AggregateKey {
///     kind: AggKind::Sum,
///     sheet: None,
///     column: 1,
/// });
/// assert!(matches!(pk, PreludeKey::Aggregate(_)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PreludeKey {
    /// A prelude-computed aggregate scalar.
    Aggregate(AggregateKey),
    /// A prelude-loaded lookup index.
    Lookup(LookupKey),
}

/// Rewrite an AST by replacing aggregate/lookup sub-expressions with
/// `Node::PreludeRef` nodes.
///
/// Only rewrites formulas classified as `AggregateOnly`, `LookupOnly`, or
/// `Mixed`. `RowLocal` and `Unsupported` pass through untouched.
///
/// # Examples
///
/// ```
/// use xlstream_parse::{classify, parse, rewrite, Classification, ClassificationContext};
/// let ast = parse("SUM(A:A)").unwrap();
/// let ctx = ClassificationContext::for_cell("Sheet1", 2, 5);
/// let verdict = classify(&ast, &ctx);
/// let rewritten = rewrite(ast, &ctx, &verdict);
/// assert!(format!("{rewritten:?}").contains("PreludeRef"));
/// ```
#[must_use]
pub fn rewrite(ast: Ast, ctx: &ClassificationContext, verdict: &Classification) -> Ast {
    match verdict {
        Classification::Unsupported(_) | Classification::RowLocal => ast,
        Classification::AggregateOnly | Classification::LookupOnly | Classification::Mixed => {
            let new_root = rewrite_node(ast.root, ctx);
            Ast { upstream: ast.upstream, root: new_root }
        }
    }
}

fn rewrite_node(node: Node, ctx: &ClassificationContext) -> Node {
    match node {
        Node::Function { ref name, ref args } => {
            let upper = name.to_uppercase();
            if crate::sets::is_aggregate(&upper) {
                rewrite_aggregate(name, args, ctx)
            } else if crate::sets::is_lookup(&upper) {
                rewrite_lookup(name, args, ctx).unwrap_or_else(|| recurse_function(node, ctx))
            } else {
                recurse_function(node, ctx)
            }
        }
        Node::BinaryOp { op, left, right } => Node::BinaryOp {
            op,
            left: Box::new(rewrite_node(*left, ctx)),
            right: Box::new(rewrite_node(*right, ctx)),
        },
        Node::UnaryOp { op, expr } => {
            Node::UnaryOp { op, expr: Box::new(rewrite_node(*expr, ctx)) }
        }
        Node::Array(rows) => Node::Array(
            rows.into_iter()
                .map(|row| row.into_iter().map(|n| rewrite_node(n, ctx)).collect())
                .collect(),
        ),
        // Leaves pass through.
        other @ (Node::Literal(_)
        | Node::Text(_)
        | Node::Error(_)
        | Node::Reference(_)
        | Node::PreludeRef(_)) => other,
    }
}

fn recurse_function(node: Node, ctx: &ClassificationContext) -> Node {
    match node {
        Node::Function { name, args } => {
            Node::Function { name, args: args.into_iter().map(|a| rewrite_node(a, ctx)).collect() }
        }
        other => other,
    }
}

/// Try to extract an [`AggKind`] from a function name.
///
/// Conditional aggregates (SUMIF, COUNTIF, AVERAGEIF, *IFS) are
/// excluded — they carry criteria the prelude can't discard. They fall
/// through to `recurse_function_owned` and keep their Function node
/// intact. Phase 7 handles them with per-criteria groupby tables.
fn agg_kind_for(name: &str) -> Option<AggKind> {
    match name.to_uppercase().as_str() {
        "SUM" => Some(AggKind::Sum),
        "COUNT" => Some(AggKind::Count),
        "COUNTA" => Some(AggKind::CountA),
        "AVERAGE" => Some(AggKind::Average),
        "MIN" => Some(AggKind::Min),
        "MAX" => Some(AggKind::Max),
        "PRODUCT" => Some(AggKind::Product),
        "MEDIAN" => Some(AggKind::Median),
        "COUNTBLANK" => Some(AggKind::CountBlank),
        _ => None,
    }
}

/// Try to build an [`AggregateKey`] from a single range reference node.
fn aggregate_key_from_range(kind: AggKind, reference: &Reference) -> Option<AggregateKey> {
    match reference {
        Reference::Range { sheet, start_col: Some(sc), end_col: Some(ec), .. } if sc == ec => {
            Some(AggregateKey { kind, sheet: sheet.clone(), column: *sc })
        }
        _ => None,
    }
}

fn rewrite_aggregate(name: &str, args: &[Node], ctx: &ClassificationContext) -> Node {
    let Some(kind) = agg_kind_for(name) else {
        return recurse_function_owned(name, args, ctx);
    };

    // Single-arg whole-column: collapse the entire Function node.
    if args.len() == 1 {
        if let Some(key) = try_aggregate_key(kind, &args[0]) {
            return Node::PreludeRef(PreludeKey::Aggregate(key));
        }
    }

    // Multi-arg: keep Function node, replace range children with PreludeRef.
    let new_args: Vec<Node> = args
        .iter()
        .map(|arg| {
            if let Some(key) = try_aggregate_key(kind, arg) {
                Node::PreludeRef(PreludeKey::Aggregate(key))
            } else {
                rewrite_node(arg.clone(), ctx)
            }
        })
        .collect();
    Node::Function { name: name.to_owned(), args: new_args }
}

fn try_aggregate_key(kind: AggKind, node: &Node) -> Option<AggregateKey> {
    match node {
        Node::Reference(r) => aggregate_key_from_range(kind, r),
        _ => None,
    }
}

fn recurse_function_owned(name: &str, args: &[Node], ctx: &ClassificationContext) -> Node {
    Node::Function {
        name: name.to_owned(),
        args: args.iter().map(|a| rewrite_node(a.clone(), ctx)).collect(),
    }
}

fn rewrite_lookup(_name: &str, _args: &[Node], _ctx: &ClassificationContext) -> Option<Node> {
    None
}

/// Extract `(sheet, start_col, end_col)` from a range reference node.
fn extract_range_info(node: &Node) -> Option<(String, u32, u32)> {
    match node {
        Node::Reference(Reference::Range {
            sheet, start_col: Some(sc), end_col: Some(ec), ..
        }) => Some((sheet.clone().unwrap_or_default(), *sc, *ec)),
        _ => None,
    }
}

/// Extract a positive integer (>= 1) as `u32` from a numeric literal node.
fn extract_positive_u32(node: &Node) -> Option<u32> {
    match node {
        Node::Literal(NumLiteral::Number(n)) => {
            let rounded = n.round();
            #[allow(clippy::float_cmp)]
            if *n == rounded && rounded >= 1.0 && rounded <= f64::from(u32::MAX) {
                // SAFETY: range-checked above — rounded is in [1.0, u32::MAX].
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                Some(rounded as u32)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Walk an AST and extract [`LookupKey`]s from lookup function calls.
///
/// Used by the prelude loader to know which sheets to load and which
/// columns to index. Does not modify the AST.
///
/// # Examples
///
/// ```
/// use xlstream_parse::{parse, collect_lookup_keys, LookupKind};
/// let ast = parse("VLOOKUP(A1, 'Regions'!A:C, 2, FALSE)").unwrap();
/// let keys = collect_lookup_keys(&ast);
/// assert_eq!(keys.len(), 1);
/// assert_eq!(keys[0].kind, LookupKind::VLookup);
/// ```
#[must_use]
pub fn collect_lookup_keys(ast: &Ast) -> Vec<LookupKey> {
    let mut keys = Vec::new();
    collect_from_node(&ast.root, &mut keys);
    keys
}

fn collect_from_node(node: &Node, keys: &mut Vec<LookupKey>) {
    match node {
        Node::Function { name, args } => {
            let upper = name.to_uppercase();
            match upper.as_str() {
                "VLOOKUP" => {
                    if let Some(key) = extract_vlookup_key(args) {
                        keys.push(key);
                    }
                }
                "HLOOKUP" => {
                    if let Some(key) = extract_hlookup_key(args) {
                        keys.push(key);
                    }
                }
                "XLOOKUP" => {
                    if let Some(key) = extract_xlookup_key(args) {
                        keys.push(key);
                    }
                }
                "MATCH" | "XMATCH" => {
                    if args.len() >= 2 {
                        if let Some((sheet, col, _)) = extract_range_info(&args[1]) {
                            keys.push(LookupKey {
                                kind: LookupKind::VLookup,
                                sheet,
                                key_index: col,
                                value_index: col,
                            });
                        }
                    }
                }
                "INDEX" => {
                    if !args.is_empty() {
                        if let Some((sheet, col, _)) = extract_range_info(&args[0]) {
                            keys.push(LookupKey {
                                kind: LookupKind::VLookup,
                                sheet,
                                key_index: col,
                                value_index: col,
                            });
                        }
                    }
                }
                _ => {}
            }
            for arg in args {
                collect_from_node(arg, keys);
            }
        }
        Node::BinaryOp { left, right, .. } => {
            collect_from_node(left, keys);
            collect_from_node(right, keys);
        }
        Node::UnaryOp { expr, .. } => collect_from_node(expr, keys),
        Node::Array(rows) => {
            for row in rows {
                for cell in row {
                    collect_from_node(cell, keys);
                }
            }
        }
        Node::Literal(_)
        | Node::Text(_)
        | Node::Error(_)
        | Node::Reference(_)
        | Node::PreludeRef(_) => {}
    }
}

fn extract_vlookup_key(args: &[Node]) -> Option<LookupKey> {
    if args.len() < 3 {
        return None;
    }
    let (sheet, key_column, _end_col) = extract_range_info(&args[1])?;
    let col_offset = extract_positive_u32(&args[2])?;
    let value_column = key_column + col_offset - 1;
    Some(LookupKey {
        kind: LookupKind::VLookup,
        sheet,
        key_index: key_column,
        value_index: value_column,
    })
}

fn extract_hlookup_key(args: &[Node]) -> Option<LookupKey> {
    if args.len() < 3 {
        return None;
    }
    let (sheet, start_col, _end_col) = extract_range_info(&args[1])?;
    let row_offset = extract_positive_u32(&args[2])?;
    Some(LookupKey {
        kind: LookupKind::HLookup,
        sheet,
        key_index: start_col,
        value_index: row_offset,
    })
}

fn extract_xlookup_key(args: &[Node]) -> Option<LookupKey> {
    if args.len() < 3 {
        return None;
    }
    let (sheet, key_column, _) = extract_range_info(&args[1])?;
    let (_, value_column, _) = extract_range_info(&args[2])?;
    Some(LookupKey {
        kind: LookupKind::XLookup,
        sheet,
        key_index: key_column,
        value_index: value_column,
    })
}
