//! Expression evaluation.

use apl_parse::{Expr, parse};
use apl_prims::{apply_dyadic, apply_monadic, reduce};
use apl_value::{AplError, AplResult, Array, ErrorKind};

use crate::system::{get_system, set_system};
use crate::workspace::Workspace;

/// Parse and evaluate one line. `Ok(None)` when there is nothing to
/// display: a blank line, a comment, or a top-level assignment
/// (including `⎕←`, which displays through the output buffer).
///
/// # Errors
/// Any lexical, syntax, or evaluation error, with a caret.
pub fn eval_line(ws: &mut Workspace, line: &str) -> AplResult<Option<Array>> {
    let Some(expr) = parse(line)? else {
        return Ok(None);
    };
    let silent = matches!(
        expr,
        Expr::Assign { .. } | Expr::SysAssign { .. } | Expr::QuadOut { .. }
    );
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
        Expr::Monadic { .. } | Expr::Dyadic { .. } | Expr::Reduce { .. } => eval_apply(ws, expr),
        _ => eval_system(ws, expr),
    }
}

/// Function application: the right argument is evaluated first, as
/// on a terminal reading right to left.
fn eval_apply(ws: &mut Workspace, expr: &Expr) -> AplResult<Array> {
    match expr {
        Expr::Monadic { f, pos, right } => {
            let r = eval_expr(ws, right)?;
            apply_monadic(*f, &r, ws.io).map_err(|e| e.at(*pos))
        }
        Expr::Dyadic {
            f,
            pos,
            left,
            right,
        } => {
            let r = eval_expr(ws, right)?;
            let l = eval_expr(ws, left)?;
            apply_dyadic(*f, &l, &r).map_err(|e| e.at(*pos))
        }
        Expr::Reduce { f, pos, right } => {
            let r = eval_expr(ws, right)?;
            reduce(*f, &r).map_err(|e| e.at(*pos))
        }
        _ => unreachable!("eval_apply only receives applications"),
    }
}

/// Quad forms: system variables and quad output/input.
fn eval_system(ws: &mut Workspace, expr: &Expr) -> AplResult<Array> {
    match expr {
        Expr::SysName(name, pos) => get_system(ws, name).map_err(|e| e.at(*pos)),
        Expr::SysAssign { name, pos, value } => {
            let v = eval_expr(ws, value)?;
            set_system(ws, name, &v).map_err(|e| e.at(*pos))?;
            Ok(v)
        }
        Expr::QuadOut { value, .. } => {
            let v = eval_expr(ws, value)?;
            ws.output.push(v.clone());
            Ok(v)
        }
        Expr::QuadIn(pos) => Err(AplError::new(ErrorKind::NotImplemented).at(*pos)),
        _ => unreachable!("eval_system only receives quad forms"),
    }
}
