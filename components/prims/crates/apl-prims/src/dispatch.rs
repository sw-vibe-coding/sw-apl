//! Glyph to primitive: mixed functions and roll here, scalar
//! functions as the fallback.

use apl_prims_mixed::{catenate, iota, ravel, reshape, shape};
use apl_prims_scalar::{dyadic, monadic};
use apl_value::{AplResult, Array};

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

/// `l f r`.
///
/// # Errors
/// Whatever the primitive reports (RANK, LENGTH, DOMAIN, NOT
/// IMPLEMENTED), without a caret.
pub fn apply_dyadic(f: char, l: &Array, r: &Array) -> AplResult<Array> {
    match f {
        '⍴' => reshape(l, r),
        ',' => catenate(l, r),
        _ => dyadic(f, l, r),
    }
}
