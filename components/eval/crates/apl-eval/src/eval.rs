//! Expression evaluation.

use apl_ast::Expr;
use apl_parse::parse;
use apl_value::{AplError, AplResult, Array, ErrorKind};

use crate::apply;
use crate::workspace::{Output, Workspace};

/// Parse and evaluate one line. `Output::Nothing` when there is
/// nothing to display: a blank line, a comment, an assignment
/// (including `⎕←`, which displays through the output buffer).
///
/// # Errors
/// Any lexical, syntax, or evaluation error, with a caret.
pub fn eval_line(ws: &mut Workspace, line: &str) -> AplResult<Output> {
    let Some(expr) = parse(line)? else {
        return Ok(Output::Nothing);
    };
    if let Expr::Mixed(parts) = &expr {
        let values = parts
            .iter()
            .map(|p| eval_expr(ws, p))
            .collect::<AplResult<_>>()?;
        return Ok(Output::Mixed(values));
    }
    let silent = matches!(
        expr,
        Expr::Assign { .. }
            | Expr::IndexedAssign { .. }
            | Expr::QuadOut { .. }
            | Expr::QuoteQuadOut { .. }
    );
    let value = eval_expr(ws, &expr)?;
    Ok(if silent {
        Output::Nothing
    } else {
        Output::Value(value)
    })
}

/// Evaluate an expression tree.
///
/// # Errors
/// VALUE ERROR for unknown names; primitive errors carry the glyph's
/// position as the caret; NOT IMPLEMENTED for indexing and branch.
pub fn eval_expr(ws: &mut Workspace, expr: &Expr) -> AplResult<Array> {
    match expr {
        Expr::Literal(a) => Ok(a.clone()),
        Expr::Name(n, pos) => ws
            .get(n)
            .cloned()
            .ok_or_else(|| AplError::new(ErrorKind::Value).at(*pos)),
        Expr::Assign { name, value, .. } => {
            let v = eval_expr(ws, value)?;
            ws.set(name, v.clone());
            Ok(v)
        }
        Expr::Monadic { .. } => eval_monadic(ws, expr),
        Expr::Dyadic { .. } => eval_dyadic(ws, expr),
        Expr::QuadOut { .. }
        | Expr::QuadIn(_)
        | Expr::QuoteQuadOut { .. }
        | Expr::QuoteQuadIn(_) => apply::quad(ws, expr),
        Expr::Index { pos, .. } | Expr::IndexedAssign { pos, .. } | Expr::Branch { pos, .. } => {
            Err(AplError::new(ErrorKind::NotImplemented).at(*pos))
        }
        Expr::Mixed(_) => Err(AplError::new(ErrorKind::Syntax)),
    }
}

/// `f right`: the right argument is evaluated first, as on a terminal
/// reading right to left; then the axis.
fn eval_monadic(ws: &mut Workspace, expr: &Expr) -> AplResult<Array> {
    let Expr::Monadic {
        func,
        pos,
        axis,
        right,
    } = expr
    else {
        unreachable!("eval_monadic only receives monadic applications")
    };
    let r = eval_expr(ws, right)?;
    let axis = apply::eval_axis(ws, axis.as_deref())?;
    apply::monadic(func, axis.as_ref(), &r, &mut ws.env).map_err(|e| e.at(*pos))
}

/// `left f right`: right, then left, then the axis.
fn eval_dyadic(ws: &mut Workspace, expr: &Expr) -> AplResult<Array> {
    let Expr::Dyadic {
        func,
        pos,
        axis,
        left,
        right,
    } = expr
    else {
        unreachable!("eval_dyadic only receives dyadic applications")
    };
    let r = eval_expr(ws, right)?;
    let l = eval_expr(ws, left)?;
    let axis = apply::eval_axis(ws, axis.as_deref())?;
    apply::dyadic(func, axis.as_ref(), &l, &r, &mut ws.env).map_err(|e| e.at(*pos))
}
