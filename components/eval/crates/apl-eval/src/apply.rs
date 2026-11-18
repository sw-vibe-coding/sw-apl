//! Function application: evaluating the arguments and the axis, then
//! dispatching a primitive or derived function.

use apl_ast::{Expr, Function};
use apl_c80_memory::system as c80;
use apl_call::value;
use apl_d83_dyadic::dyadic as pc;
use apl_execute as execute;
use apl_modes::{Feature, Profile};
use apl_prims::{apply_dyadic, apply_monadic, axis_index, inner, outer, reduce, scan};
use apl_value::ErrorKind::{self, Syntax};
use apl_value::{AplError, AplResult, Array};
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
/// which only (B) lexes, is `apl-sysfns`'s. Scan is a mode's feature: in (A),
/// as in APL\360, a function before a slope is an expansion with no
/// array on its left, a SYNTAX ERROR when the line runs. (C)'s status
/// and conversion, `⌶` and `#` there, and its peek, poke and Call are
/// `apl-c80-memory`'s.
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
    let (profile, at, bare) = (ws.mode.profile(), |e: AplError| e.at(*pos), axis.is_none());
    match func {
        Function::Defined(name) => return value(ws, name, *pos, (None, Some(r)), eval_line),
        Function::System(name) => return apl_sysfns::monadic(ws, name, &r).map_err(at),
        Function::Prim('⌶' | '#' | '⌷') if bare => return c80(ws, func, None, &r).map_err(at),
        Function::Prim('⍎') if bare => return execute::value(ws, &r, *pos, eval_line),
        Function::Scan { .. } if !profile.has(Feature::Scan) => {
            return Err(at(AplError::new(Syntax)));
        }
        _ => {}
    }
    let axis = axis.as_deref().map(|a| eval_expr(ws, a)).transpose()?;
    applied(func, axis.as_ref(), (None, &r), &mut ws.saved, profile).map_err(at)
}

/// `left f right`: right, then left, then the axis. What (D) adds --
/// `⎕EA` and the grades with a collating sequence -- is
/// `apl-d83-dyadic`'s, tried first when no bracket was written.
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
    if axis.is_none()
        && let Some(done) = pc(ws, func, *pos, (&l, &r), eval_line)
    {
        return done;
    }
    dyadic(ws, func, (*pos, axis.as_deref()), l, r)
}

/// `l func r`, evaluated, as the core, (B) and (C) answer it: a
/// defined function, a system function, (C)'s conversion and Call,
/// or a primitive with its axis evaluated now.
fn dyadic(
    ws: &mut Workspace,
    func: &Function,
    (pos, axis): (usize, Option<&Expr>),
    l: Array,
    r: Array,
) -> AplResult<Array> {
    let (at, bare) = (|e: AplError| e.at(pos), axis.is_none());
    match func {
        Function::Defined(name) => return value(ws, name, pos, (Some(l), Some(r)), eval_line),
        Function::System(name) => return apl_sysfns::dyadic(ws, name, &l, &r).map_err(at),
        Function::Prim('#' | '⌷') if bare => return c80(ws, func, Some(&l), &r).map_err(at),
        _ => {}
    }
    let axis = axis.map(|a| eval_expr(ws, a)).transpose()?;
    let profile = ws.mode.profile();
    applied(func, axis.as_ref(), (Some(&l), &r), &mut ws.saved, profile).map_err(at)
}

/// `left func right`, or `func right` with no left, with the evaluated
/// axis: a primitive, (B)'s format, a reduce or scan along the last
/// axis, the first or the bracket, or an inner or outer product (which
/// take no axis), once the mode's cuts allow it. Anything else -- a
/// product with one argument, a defined function that got this far --
/// is no such function, and the sentence does not parse.
fn applied(
    func: &Function,
    axis: Option<&Array>,
    (left, r): (Option<&Array>, &Array),
    saved: &mut Saved,
    profile: Profile,
) -> AplResult<Array> {
    apl_c80_cuts::mixed(profile, func, axis, left, r)?;
    let (rank, env) = (r.shape.len(), &mut saved.env);
    let io = env.io;
    let along = |first: bool| axis_index(axis, first, rank, io);
    match (func, left) {
        (Function::Prim('⍕'), None) if axis.is_none() => {
            apl_format::monadic(r, saved.print.precision())
        }
        (Function::Prim('⍕'), Some(l)) if axis.is_none() => apl_format::dyadic(l, r),
        (Function::Prim(f), None) => apply_monadic(*f, r, axis, env),
        (Function::Prim(f), Some(l)) => apply_dyadic(*f, l, r, axis, env),
        (Function::Reduce { f, first }, None) => reduce(*f, r, along(*first)?, env.ct),
        (Function::Scan { f, first }, None) => scan(*f, r, along(*first)?, env.ct),
        (Function::Inner { f, g }, Some(l)) if axis.is_none() => inner(*f, *g, l, r, env.ct),
        (Function::Outer { f }, Some(l)) if axis.is_none() => outer(*f, l, r, env.ct),
        _ => Err(AplError::new(ErrorKind::Syntax)),
    }
}
