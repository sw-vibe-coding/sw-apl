//! Glyph to primitive: mixed functions and roll here, scalar
//! functions as the fallback.

use apl_prims_join::{catenate, resolve_axis};
use apl_prims_mask::{compress, expand};
use apl_prims_mixed::{iota, ravel, reshape, shape};
use apl_prims_scalar::{dyadic, monadic};
use apl_prims_search::{index_of, membership};
use apl_prims_select::{axis_index, drop, reverse, rotate, take, transpose};
use apl_value::{AplError, AplResult, Array, ErrorKind};

use crate::random::{Env, roll};

/// `f r`, with the evaluated axis bracket when one was written.
///
/// # Errors
/// Whatever the primitive reports (RANK, LENGTH, DOMAIN, INDEX for a
/// bad axis, NOT IMPLEMENTED), without a caret.
pub fn apply_monadic(f: char, r: &Array, axis: Option<&Array>, env: &mut Env) -> AplResult<Array> {
    let rank = r.shape.len();
    match f {
        '⌽' => Ok(reverse(r, axis_index(axis, false, rank, env.io)?)),
        '⊖' => Ok(reverse(r, axis_index(axis, true, rank, env.io)?)),
        _ if axis.is_some() => Err(AplError::new(ErrorKind::NotImplemented)),
        '⍳' => iota(r, env.io),
        '⍴' => Ok(shape(r)),
        ',' => Ok(ravel(r)),
        '⍉' => transpose(None, r, env.io),
        '?' => roll(r, env),
        _ => monadic(f, r),
    }
}

/// `l f r`, with the evaluated axis bracket when one was written.
///
/// # Errors
/// As for [`apply_monadic`].
pub fn apply_dyadic(
    f: char,
    l: &Array,
    r: &Array,
    axis: Option<&Array>,
    env: &Env,
) -> AplResult<Array> {
    let rank = r.shape.len();
    match f {
        ',' => catenate(l, r, resolve_axis(axis, env.io, l.shape.len().max(rank))?),
        '⌽' | '⊖' | '/' | '⌿' | '\\' | '⍀' => {
            let first = matches!(f, '⊖' | '⌿' | '⍀');
            axis_dyadic(f, l, r, axis_index(axis, first, rank, env.io)?)
        }
        _ if axis.is_some() => Err(AplError::new(ErrorKind::NotImplemented)),
        '⍴' => reshape(l, r),
        '↑' => take(l, r),
        '↓' => drop(l, r),
        '⍉' => transpose(Some(l), r, env.io),
        '∊' => Ok(membership(l, r)),
        '⍳' => index_of(l, r, env.io),
        _ => dyadic(f, l, r),
    }
}

/// The dyadic functions that work along one axis.
fn axis_dyadic(f: char, l: &Array, r: &Array, k: usize) -> AplResult<Array> {
    match f {
        '⌽' | '⊖' => rotate(l, r, k),
        '/' | '⌿' => compress(l, r, k),
        _ => expand(l, r, k),
    }
}
