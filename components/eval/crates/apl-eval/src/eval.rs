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
        Expr::Monadic { .. } | Expr::Dyadic { .. } => eval_apply(ws, expr),
        Expr::QuadOut { .. }
        | Expr::QuadIn(_)
        | Expr::QuoteQuadOut { .. }
        | Expr::QuoteQuadIn(_) => eval_quad(ws, expr),
        Expr::Index { pos, .. } | Expr::IndexedAssign { pos, .. } | Expr::Branch { pos, .. } => {
            Err(AplError::new(ErrorKind::NotImplemented).at(*pos))
        }
        Expr::Mixed(_) => Err(AplError::new(ErrorKind::Syntax)),
    }
}

/// Function application: the right argument is evaluated first, as
/// on a terminal reading right to left.
fn eval_apply(ws: &mut Workspace, expr: &Expr) -> AplResult<Array> {
    match expr {
        Expr::Monadic {
            func,
            pos,
            axis,
            right,
        } => {
            let r = eval_expr(ws, right)?;
            apply::monadic(func, axis.is_some(), &r, &mut ws.env).map_err(|e| e.at(*pos))
        }
        Expr::Dyadic {
            func,
            pos,
            axis,
            left,
            right,
        } => {
            let r = eval_expr(ws, right)?;
            let l = eval_expr(ws, left)?;
            apply::dyadic(func, axis.is_some(), &l, &r).map_err(|e| e.at(*pos))
        }
        _ => unreachable!("eval_apply only receives applications"),
    }
}

/// Quad forms: quad output and quad input.
fn eval_quad(ws: &mut Workspace, expr: &Expr) -> AplResult<Array> {
    match expr {
        Expr::QuadOut { value, .. } => {
            let v = eval_expr(ws, value)?;
            ws.output.push(v.clone());
            Ok(v)
        }
        Expr::QuadIn(pos) | Expr::QuoteQuadIn(pos) | Expr::QuoteQuadOut { pos, .. } => {
            Err(AplError::new(ErrorKind::NotImplemented).at(*pos))
        }
        _ => unreachable!("eval_quad only receives quad forms"),
    }
}
