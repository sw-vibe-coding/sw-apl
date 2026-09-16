//! Expression evaluation.

use apl_parse::{Expr, parse};
use apl_prims_scalar::{dyadic, monadic};
use apl_value::{AplError, AplResult, Array, ErrorKind, Number};

use crate::workspace::Workspace;

/// Parse and evaluate one line. `Ok(None)` when there is nothing to
/// display: a blank line, a comment, or a top-level assignment.
///
/// # Errors
/// Any lexical, syntax, or evaluation error, with a caret.
pub fn eval_line(ws: &mut Workspace, line: &str) -> AplResult<Option<Array>> {
    let Some(expr) = parse(line)? else {
        return Ok(None);
    };
    let silent = matches!(expr, Expr::Assign { .. });
    let value = eval_expr(ws, &expr)?;
    Ok(if silent { None } else { Some(value) })
}

/// Evaluate an expression tree.
///
/// # Errors
/// VALUE ERROR for unknown names; primitive errors carry the glyph's
/// position as the caret.
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
    }
}

/// Function application: the right argument is evaluated first, as
/// on a terminal reading right to left.
fn eval_apply(ws: &mut Workspace, expr: &Expr) -> AplResult<Array> {
    match expr {
        Expr::Monadic { f, pos, right } => {
            let r = eval_expr(ws, right)?;
            apply_monadic(*f, &r).map_err(|e| e.at(*pos))
        }
        Expr::Dyadic {
            f,
            pos,
            left,
            right,
        } => {
            let r = eval_expr(ws, right)?;
            let l = eval_expr(ws, left)?;
            dyadic(*f, &l, &r).map_err(|e| e.at(*pos))
        }
        _ => unreachable!("eval_apply only receives applications"),
    }
}

/// Monadic dispatch: the mixed functions handled here, the rest are
/// scalar.
fn apply_monadic(f: char, r: &Array) -> AplResult<Array> {
    match f {
        '⍴' => Ok(Array::vector(
            r.shape
                .iter()
                .map(|&n| Number::Int(i64::try_from(n).unwrap_or(i64::MAX)))
                .collect(),
        )),
        _ => monadic(f, r),
    }
}
