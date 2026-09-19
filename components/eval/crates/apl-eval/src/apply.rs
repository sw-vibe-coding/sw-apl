//! Function application: evaluating the arguments and the axis, then
//! dispatching a primitive or derived function.

use apl_ast::{Expr, Function};
use apl_call::value;
use apl_ibeam::{argument, ibeam};
use apl_prims::{Env, apply_dyadic, apply_monadic, axis_index, inner, outer, reduce, scan};
use apl_value::{AplError, AplResult, Array, ErrorKind};
use apl_workspace::{Workspace, free, used};

use crate::eval::{eval_expr, eval_line};

/// `f right`: the right argument is evaluated first, as on a terminal
/// reading right to left; then the axis.
///
/// The I-beam is answered here rather than in `apl-prims`, because
/// what it reports is the workspace's: its clock and its state
/// indicator, innermost first, which is what `⌶26` and `⌶27` read.
/// It is answered only when no bracket was written, since an axis on
/// a glyph that takes none is a SYNTAX ERROR the dispatch reports.
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
    if *func == Function::Prim('⌶') && axis.is_none() {
        let si = ws.si().iter().rev();
        let lines: Vec<i64> = si.filter_map(|a| a.line.try_into().ok()).collect();
        let held = used(&ws.saved.vars, &ws.saved.funcs, &ws.saved.groups);
        let left = i64::try_from(free(ws.quota, held)).unwrap_or(i64::MAX);
        let got = ibeam(argument(&r)?, (ws.clock)(), ws.signed_on, &lines, left);
        return got.map_err(|e| e.at(*pos));
    }
    let axis = axis.as_deref().map(|a| eval_expr(ws, a)).transpose()?;
    monadic(func, axis.as_ref(), &r, &mut ws.saved.env).map_err(|e| e.at(*pos))
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
    dyadic(func, axis.as_ref(), &l, &r, &mut ws.saved.env).map_err(|e| e.at(*pos))
}

/// `func right`, with the evaluated axis: a primitive, or a reduce
/// or scan along the last axis, the first axis, or the bracket.
fn monadic(func: &Function, axis: Option<&Array>, r: &Array, env: &mut Env) -> AplResult<Array> {
    let rank = r.shape.len();
    match func {
        Function::Prim(f) => apply_monadic(*f, r, axis, env),
        Function::Reduce { f, first } => reduce(*f, r, axis_index(axis, *first, rank, env.io)?),
        Function::Scan { f, first } => scan(*f, r, axis_index(axis, *first, rank, env.io)?),
        // An inner or outer product with one argument, or a defined
        // function that got this far: no such monadic function, so
        // the sentence does not parse.
        _ => Err(AplError::new(ErrorKind::Syntax)),
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
