//! Function application: evaluating the arguments and the axis, then
//! dispatching a primitive or derived function.

use apl_ast::{Expr, Function};
use apl_call::value;
use apl_prims::{Env, apply_dyadic, apply_monadic, axis_index, inner, outer, reduce, scan};
use apl_value::{AplError, AplResult, Array, ErrorKind};
use apl_workspace::Workspace;

use crate::eval::{eval_expr, eval_line};

/// `f right`: the right argument is evaluated first, as on a terminal
/// reading right to left; then the axis.
///
/// # Errors
/// Evaluation errors, with the glyph's position as the caret.
pub fn eval_monadic(ws: &mut Workspace, expr: &Expr) -> AplResult<Array> {
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
    if let Function::Defined(name) = func {
        return value(ws, name, *pos, (None, Some(r)), eval_line);
    }
    let axis = axis.as_deref().map(|a| eval_expr(ws, a)).transpose()?;
    monadic(func, axis.as_ref(), &r, &mut ws.env).map_err(|e| e.at(*pos))
}

/// `left f right`: right, then left, then the axis.
///
/// # Errors
/// Evaluation errors, with the glyph's position as the caret.
pub fn eval_dyadic(ws: &mut Workspace, expr: &Expr) -> AplResult<Array> {
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
    if let Function::Defined(name) = func {
        return value(ws, name, *pos, (Some(l), Some(r)), eval_line);
    }
    let axis = axis.as_deref().map(|a| eval_expr(ws, a)).transpose()?;
    dyadic(func, axis.as_ref(), &l, &r, &mut ws.env).map_err(|e| e.at(*pos))
}

/// `func right`, with the evaluated axis: a primitive, or a reduce
/// or scan along the last axis, the first axis, or the bracket.
fn monadic(func: &Function, axis: Option<&Array>, r: &Array, env: &mut Env) -> AplResult<Array> {
    let rank = r.shape.len();
    match func {
        Function::Prim(f) => apply_monadic(*f, r, axis, env),
        Function::Reduce { f, first } => reduce(*f, r, axis_index(axis, *first, rank, env.io)?),
        Function::Scan { f, first } => scan(*f, r, axis_index(axis, *first, rank, env.io)?),
        _ => Err(AplError::new(ErrorKind::NotImplemented)),
    }
}

/// `left func right`, with the evaluated axis: a primitive, an inner
/// product, or an outer product (the products take no axis).
fn dyadic(
    func: &Function,
    axis: Option<&Array>,
    l: &Array,
    r: &Array,
    env: &mut Env,
) -> AplResult<Array> {
    match func {
        Function::Prim(f) => apply_dyadic(*f, l, r, axis, env),
        Function::Inner { f, g } if axis.is_none() => inner(*f, *g, l, r),
        Function::Outer { f } if axis.is_none() => outer(*f, l, r),
        _ => Err(AplError::new(ErrorKind::Syntax)),
    }
}
