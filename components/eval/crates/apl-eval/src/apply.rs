//! Function application: primitive and derived functions.

use apl_ast::{Expr, Function};
use apl_prims::{Env, apply_dyadic, apply_monadic, reduce};
use apl_value::{AplError, AplResult, Array, ErrorKind};

use crate::eval::eval_expr;
use crate::workspace::Workspace;

/// The value of an axis bracket, when one was written.
pub fn eval_axis(ws: &mut Workspace, axis: Option<&Expr>) -> AplResult<Option<Array>> {
    axis.map(|a| eval_expr(ws, a)).transpose()
}

/// `func right`. An axis is not implemented yet for monadic forms.
pub fn monadic(
    func: &Function,
    axis: Option<&Array>,
    r: &Array,
    env: &mut Env,
) -> AplResult<Array> {
    match func {
        _ if axis.is_some() => Err(AplError::new(ErrorKind::NotImplemented)),
        Function::Prim(f) => apply_monadic(*f, r, env),
        Function::Reduce { f, first: false } => reduce(*f, r),
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
    env: &Env,
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
