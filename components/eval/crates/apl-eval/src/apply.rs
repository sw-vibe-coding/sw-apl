//! Function application: primitive and derived functions.

use apl_ast::Function;
use apl_prims::{Env, apply_dyadic, apply_monadic, reduce};
use apl_value::{AplError, AplResult, Array, ErrorKind};

/// `func right`. An axis is not implemented yet.
pub fn monadic(func: &Function, has_axis: bool, r: &Array, env: &mut Env) -> AplResult<Array> {
    match func {
        _ if has_axis => Err(AplError::new(ErrorKind::NotImplemented)),
        Function::Prim(f) => apply_monadic(*f, r, env),
        Function::Reduce { f, first: false } => reduce(*f, r),
        _ => Err(AplError::new(ErrorKind::NotImplemented)),
    }
}

/// `left func right`. Inner and outer products and axes are not
/// implemented yet.
pub fn dyadic(func: &Function, has_axis: bool, l: &Array, r: &Array) -> AplResult<Array> {
    match func {
        Function::Prim(f) if !has_axis => apply_dyadic(*f, l, r),
        _ => Err(AplError::new(ErrorKind::NotImplemented)),
    }
}
