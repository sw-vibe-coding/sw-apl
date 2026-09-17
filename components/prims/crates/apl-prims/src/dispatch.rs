//! Glyph to primitive: mixed functions and roll here, scalar
//! functions as the fallback.

use apl_prims_join::{catenate, resolve_axis};
use apl_prims_mixed::{iota, ravel, reshape, shape};
use apl_prims_scalar::{dyadic, monadic};
use apl_value::{AplError, AplResult, Array, ErrorKind};

use crate::random::{Env, roll};

/// `f r`.
///
/// # Errors
/// Whatever the primitive reports (RANK, LENGTH, DOMAIN, NOT
/// IMPLEMENTED), without a caret.
pub fn apply_monadic(f: char, r: &Array, env: &mut Env) -> AplResult<Array> {
    match f {
        '⍳' => iota(r, env.io),
        '⍴' => Ok(shape(r)),
        ',' => Ok(ravel(r)),
        '?' => roll(r, env),
        _ => monadic(f, r),
    }
}

/// `l f r`, with the evaluated axis bracket when one was written.
///
/// # Errors
/// Whatever the primitive reports (RANK, LENGTH, DOMAIN, INDEX for a
/// bad axis, NOT IMPLEMENTED), without a caret.
pub fn apply_dyadic(
    f: char,
    l: &Array,
    r: &Array,
    axis: Option<&Array>,
    env: &Env,
) -> AplResult<Array> {
    match f {
        ',' => {
            let rank = l.shape.len().max(r.shape.len());
            catenate(l, r, resolve_axis(axis, env.io, rank)?)
        }
        _ if axis.is_some() => Err(AplError::new(ErrorKind::NotImplemented)),
        '⍴' => reshape(l, r),
        _ => dyadic(f, l, r),
    }
}
