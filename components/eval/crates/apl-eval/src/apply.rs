//! Function application: primitive and derived functions.

use apl_ast::{Expr, Function};
use apl_prims::{Env, apply_dyadic, apply_monadic, axis_index, reduce, scan};
use apl_value::{AplError, AplResult, Array, ErrorKind};

use crate::eval::eval_expr;
use crate::workspace::Workspace;

/// The value of an axis bracket, when one was written.
pub fn eval_axis(ws: &mut Workspace, axis: Option<&Expr>) -> AplResult<Option<Array>> {
    axis.map(|a| eval_expr(ws, a)).transpose()
}

/// `func right`, with the evaluated axis: a primitive, or a reduce
/// or scan along the last axis, the first axis, or the bracket.
pub fn monadic(
    func: &Function,
    axis: Option<&Array>,
    r: &Array,
    env: &mut Env,
) -> AplResult<Array> {
    let rank = r.shape.len();
    match func {
        Function::Prim(f) => apply_monadic(*f, r, axis, env),
        Function::Reduce { f, first } => reduce(*f, r, axis_index(axis, *first, rank, env.io)?),
        Function::Scan { f, first } => scan(*f, r, axis_index(axis, *first, rank, env.io)?),
        _ => Err(AplError::new(ErrorKind::NotImplemented)),
    }
}

/// `left func right`, with the evaluated axis. Inner and outer
/// products are not implemented yet.
pub fn dyadic(
    func: &Function,
    axis: Option<&Array>,
    l: &Array,
    r: &Array,
    env: &mut Env,
) -> AplResult<Array> {
    match func {
        Function::Prim(f) => apply_dyadic(*f, l, r, axis, env),
        _ => Err(AplError::new(ErrorKind::NotImplemented)),
    }
}

/// Quad forms: quad output and quad input.
///
/// # Errors
/// NOT IMPLEMENTED for input and quote-quad output, for now.
pub fn quad(ws: &mut Workspace, expr: &Expr) -> AplResult<Array> {
    match expr {
        Expr::QuadOut { value, .. } => {
            let v = eval_expr(ws, value)?;
            ws.output.push(v.clone());
            Ok(v)
        }
        Expr::QuadIn(pos) | Expr::QuoteQuadIn(pos) | Expr::QuoteQuadOut { pos, .. } => {
            Err(AplError::new(ErrorKind::NotImplemented).at(*pos))
        }
        _ => unreachable!("quad only receives quad forms"),
    }
}
