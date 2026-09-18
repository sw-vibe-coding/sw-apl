//! Glyph to primitive: mixed functions and the random functions
//! here, scalar functions as the fallback.

use apl_prims_join::{catenate, resolve_axis};
use apl_prims_mask::{compress, expand};
use apl_prims_mixed::{iota, ravel, reshape, shape};
use apl_prims_radix::{decode, encode};
use apl_prims_scalar::{dyadic, monadic};
use apl_prims_search::{grade, index_of, membership};
use apl_prims_select::{axis_index, drop, reverse, rotate, take, transpose};
use apl_value::{AplError, AplResult, Array, ErrorKind};

use crate::axis::no_axis;
use crate::random::{Env, deal, roll};

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
        _ if axis.is_some() => Err(no_axis(f)),
        '⍳' => iota(r, env.io),
        '⍴' => Ok(shape(r)),
        ',' => Ok(ravel(r)),
        '⍉' => transpose(None, r, env.io),
        '?' => roll(r, env),
        '⍋' => grade(r, false, env.io),
        '⍒' => grade(r, true, env.io),
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
    env: &mut Env,
) -> AplResult<Array> {
    let rank = r.shape.len();
    match f {
        ',' => catenate(l, r, resolve_axis(axis, env.io, l.shape.len().max(rank))?),
        '⌽' | '⊖' | '/' | '⌿' | '\\' | '⍀' => {
            let first = matches!(f, '⊖' | '⌿' | '⍀');
            axis_dyadic(f, l, r, axis_index(axis, first, rank, env.io)?)
        }
        _ if axis.is_some() => Err(no_axis(f)),
        // The I-beam is monadic: its argument selects a system value,
        // and there is nothing for a left one to mean.
        '⌶' => Err(AplError::new(ErrorKind::Domain)),
        _ => mixed_dyadic(f, l, r, env).unwrap_or_else(|| dyadic(f, l, r)),
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

/// The dyadic mixed functions that take no axis; `None` for a
/// scalar function glyph.
fn mixed_dyadic(f: char, l: &Array, r: &Array, env: &mut Env) -> Option<AplResult<Array>> {
    Some(match f {
        '⍴' => reshape(l, r),
        '↑' => take(l, r),
        '↓' => drop(l, r),
        '⍉' => transpose(Some(l), r, env.io),
        '∊' => Ok(membership(l, r)),
        '⍳' => index_of(l, r, env.io),
        '⊥' => decode(l, r),
        '⊤' => encode(l, r),
        '?' => deal(l, r, env),
        _ => return None,
    })
}
