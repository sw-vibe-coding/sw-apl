//! Function application: evaluating the arguments and the axis, then
//! dispatching a primitive or derived function.

use apl_ast::{Expr, Function};
use apl_call::value;
use apl_execute as execute;
use apl_ibeam::system_value;
use apl_prims::{apply_dyadic, apply_monadic, axis_index, inner, outer, reduce, scan};
use apl_value::{AplError, AplResult, Array, ErrorKind};
use apl_workspace::{Saved, Workspace};

use crate::eval::{eval_expr, eval_line};

/// `f right`: the right argument is evaluated first, as on a terminal
/// reading right to left; then the axis.
///
/// The I-beam is answered here rather than in `apl-prims`, because
/// what it reports is the workspace's: its clock and its state
/// indicator, innermost first, which is what `⌶26` and `⌶27` read.
/// Execute is too, because it runs a line in the workspace. Each is
/// answered only when no bracket was written, since an axis on a
/// glyph that takes none is a SYNTAX ERROR the dispatch reports. The
/// lexer lets execute and format through only in (B); format is not
/// implemented yet, and says so with NONCE ERROR. A system function,
/// which only (B) lexes, is `apl-sysfns`'s.
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
    match func {
        Function::Defined(name) => return value(ws, name, *pos, (None, Some(r)), eval_line),
        Function::System(name) => return apl_sysfns::monadic(ws, name, &r).map_err(|e| e.at(*pos)),
        Function::Prim('⌶') if axis.is_none() => {
            return system_value(ws, &r).map_err(|e| e.at(*pos));
        }
        Function::Prim('⍎') if axis.is_none() => return execute::value(ws, &r, *pos, eval_line),
        _ => {}
    }
    let axis = axis.as_deref().map(|a| eval_expr(ws, a)).transpose()?;
    monadic(func, axis.as_ref(), &r, &mut ws.saved).map_err(|e| e.at(*pos))
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
    match func {
        Function::Defined(name) => return value(ws, name, *pos, (Some(l), Some(r)), eval_line),
        Function::System(name) => {
            return apl_sysfns::dyadic(ws, name, &l, &r).map_err(|e| e.at(*pos));
        }
        _ => {}
    }
    let axis = axis.as_deref().map(|a| eval_expr(ws, a)).transpose()?;
    dyadic(func, axis.as_ref(), &l, &r, &mut ws.saved).map_err(|e| e.at(*pos))
}

/// `func right`, with the evaluated axis: a primitive, (B)'s format,
/// or a reduce or scan along the last axis, the first axis, or the
/// bracket.
fn monadic(
    func: &Function,
    axis: Option<&Array>,
    r: &Array,
    saved: &mut Saved,
) -> AplResult<Array> {
    let (rank, env) = (r.shape.len(), &mut saved.env);
    match func {
        Function::Prim('⍕') if axis.is_none() => apl_format::monadic(r, saved.print.precision()),
        Function::Prim(f) => apply_monadic(*f, r, axis, env),
        Function::Reduce { f, first } => {
            reduce(*f, r, axis_index(axis, *first, rank, env.io)?, env.ct)
        }
        Function::Scan { f, first } => scan(*f, r, axis_index(axis, *first, rank, env.io)?, env.ct),
        // An inner or outer product with one argument, or a defined
        // function that got this far: no such monadic function, so
        // the sentence does not parse.
        _ => Err(AplError::new(ErrorKind::Syntax)),
    }
}

/// `left func right`, with the evaluated axis: a primitive, (B)'s
/// format, an inner product, or an outer product (the products take
/// no axis).
fn dyadic(
    func: &Function,
    axis: Option<&Array>,
    l: &Array,
    r: &Array,
    saved: &mut Saved,
) -> AplResult<Array> {
    let env = &mut saved.env;
    match func {
        Function::Prim('⍕') if axis.is_none() => apl_format::dyadic(l, r),
        Function::Prim(f) => apply_dyadic(*f, l, r, axis, env),
        Function::Inner { f, g } if axis.is_none() => inner(*f, *g, l, r, env.ct),
        Function::Outer { f } if axis.is_none() => outer(*f, l, r, env.ct),
        _ => Err(AplError::new(ErrorKind::Syntax)),
    }
}
