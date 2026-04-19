//! [`Interpreter`] — formula evaluator. Walks the AST via [`NodeView`] and
//! resolves values against the current row.

use xlstream_core::{CellError, Value};
use xlstream_parse::{NodeRef, NodeView};

use crate::prelude::Prelude;
use crate::scope::RowScope;

/// Formula evaluator. Walks the AST via [`NodeView`] and resolves values
/// against the current row scope.
///
/// Handles literals, same-row cell references, all operators
/// (arithmetic, comparison, concatenation, unary), conditional builtins,
/// and prelude-resolved aggregate references.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::{Interpreter, Prelude, RowScope};
/// use xlstream_parse::parse;
///
/// let prelude = Prelude::empty();
/// let interp = Interpreter::new(&prelude);
/// let ast = parse("42").unwrap();
/// let scope = RowScope::new(&[], 1);
/// assert_eq!(interp.eval(ast.root(), &scope), Value::Number(42.0));
/// ```
pub struct Interpreter<'ctx> {
    prelude: &'ctx Prelude,
}

impl<'ctx> Interpreter<'ctx> {
    /// Build an interpreter backed by the given prelude data.
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_eval::{Interpreter, Prelude};
    /// let prelude = Prelude::empty();
    /// let _interp = Interpreter::new(&prelude);
    /// ```
    #[must_use]
    pub fn new(prelude: &'ctx Prelude) -> Self {
        Self { prelude }
    }

    /// Evaluate a single AST node against the current row.
    ///
    /// Returns a [`Value`]. Never errors at the library level — unsupported
    /// constructs produce cell-level errors (`#VALUE!`, `#REF!`, `#NAME?`).
    ///
    /// # Examples
    ///
    /// ```
    /// use xlstream_core::Value;
    /// use xlstream_eval::{Interpreter, Prelude, RowScope};
    /// use xlstream_parse::parse;
    ///
    /// let prelude = Prelude::empty();
    /// let interp = Interpreter::new(&prelude);
    /// let ast = parse("TRUE").unwrap();
    /// let scope = RowScope::new(&[], 0);
    /// assert_eq!(interp.eval(ast.root(), &scope), Value::Bool(true));
    /// ```
    #[must_use]
    pub fn eval(&self, node: NodeRef<'_>, scope: &RowScope<'_>) -> Value {
        match node.view() {
            NodeView::Number(n) => Value::Number(n),
            NodeView::Bool(b) => Value::Bool(b),
            NodeView::Text(s) => Value::Text(s.into()),
            NodeView::Error(e) => Value::Error(e),

            // Cell ref: classifier guaranteed same-row. Use col only.
            NodeView::CellRef { col, .. } => scope.get(col),

            // Ranges outside functions -> #REF!
            NodeView::RangeRef { .. } => Value::Error(CellError::Ref),

            // Named/External/Table -> #NAME?
            NodeView::NamedRef(_) | NodeView::ExternalRef { .. } | NodeView::TableRef { .. } => {
                Value::Error(CellError::Name)
            }

            NodeView::BinaryOp { op } => {
                let (Some(left_node), Some(right_node)) = (node.left(), node.right()) else {
                    return Value::Error(CellError::Value);
                };
                let left = self.eval(left_node, scope);
                let right = self.eval(right_node, scope);
                crate::ops::eval_binary(op, &left, &right)
            }

            NodeView::UnaryOp { op } => {
                let Some(operand_node) = node.operand() else {
                    return Value::Error(CellError::Value);
                };
                let operand = self.eval(operand_node, scope);
                crate::ops::eval_unary(op, &operand)
            }

            NodeView::Function { name } => {
                let args = node.args();
                if let Some(result) = crate::builtins::dispatch(name, &args, self, scope) {
                    result
                } else {
                    Value::Error(CellError::Value)
                }
            }

            NodeView::Array { .. } => Value::Error(CellError::Value),

            NodeView::PreludeRef(key) => match key {
                xlstream_parse::PreludeKey::Aggregate(agg_key) => {
                    self.prelude.get_aggregate(agg_key)
                }
                xlstream_parse::PreludeKey::Lookup(_) => Value::Error(CellError::Value),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

    use xlstream_core::{CellError, Value};
    use xlstream_parse::parse;

    use super::*;

    fn make_interp(prelude: &Prelude) -> Interpreter<'_> {
        Interpreter::new(prelude)
    }

    #[test]
    fn eval_number_literal() {
        let prelude = Prelude::empty();
        let interp = make_interp(&prelude);
        let ast = parse("42").unwrap();
        let scope = RowScope::new(&[], 0);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Number(42.0));
    }

    #[test]
    fn eval_text_literal() {
        let prelude = Prelude::empty();
        let interp = make_interp(&prelude);
        let ast = parse("\"hello\"").unwrap();
        let scope = RowScope::new(&[], 0);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Text("hello".into()));
    }

    #[test]
    fn eval_bool_literal() {
        let prelude = Prelude::empty();
        let interp = make_interp(&prelude);
        let ast = parse("TRUE").unwrap();
        let scope = RowScope::new(&[], 0);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Bool(true));
    }

    #[test]
    fn eval_error_literal() {
        let prelude = Prelude::empty();
        let interp = make_interp(&prelude);
        let ast = parse("#REF!").unwrap();
        let scope = RowScope::new(&[], 0);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Error(CellError::Ref));
    }

    #[test]
    fn eval_cell_ref_returns_row_value() {
        let prelude = Prelude::empty();
        let interp = make_interp(&prelude);
        // A1 parses as col=1, row=1
        let ast = parse("A1").unwrap();
        let row = vec![Value::Number(99.0), Value::Text("x".into())];
        let scope = RowScope::new(&row, 0);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Number(99.0));
    }

    #[test]
    fn eval_cell_ref_out_of_bounds() {
        let prelude = Prelude::empty();
        let interp = make_interp(&prelude);
        // Z1 = col 26, row 1 — well beyond a 2-element row.
        let ast = parse("Z1").unwrap();
        let row = vec![Value::Number(1.0), Value::Number(2.0)];
        let scope = RowScope::new(&row, 0);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Error(CellError::Ref),);
    }

    #[test]
    fn eval_function_returns_value_error() {
        let prelude = Prelude::empty();
        let interp = make_interp(&prelude);
        let ast = parse("SUM(A1:A10)").unwrap();
        let scope = RowScope::new(&[], 0);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Error(CellError::Value),);
    }

    #[test]
    fn eval_binary_add() {
        let prelude = Prelude::empty();
        let interp = make_interp(&prelude);
        let ast = parse("1+2").unwrap();
        let scope = RowScope::new(&[], 0);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Number(3.0));
    }

    #[test]
    fn eval_unary_negate() {
        let prelude = Prelude::empty();
        let interp = make_interp(&prelude);
        let ast = parse("-5").unwrap();
        let scope = RowScope::new(&[], 0);
        assert_eq!(interp.eval(ast.root(), &scope), Value::Number(-5.0));
    }
}
