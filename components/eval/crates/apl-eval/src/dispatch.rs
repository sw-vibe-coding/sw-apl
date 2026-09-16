//! Glyph to primitive: mixed functions here, scalar functions as the
//! fallback.

use apl_prims_mixed::{catenate, iota, ravel, reshape, shape};
use apl_prims_scalar::{dyadic, monadic};
use apl_value::{AplResult, Array};

/// `f r`.
pub fn apply_monadic(f: char, r: &Array, io: i64) -> AplResult<Array> {
    match f {
        '⍳' => iota(r, io),
        '⍴' => Ok(shape(r)),
        ',' => Ok(ravel(r)),
        _ => monadic(f, r),
    }
}

/// `l f r`.
pub fn apply_dyadic(f: char, l: &Array, r: &Array) -> AplResult<Array> {
    match f {
        '⍴' => reshape(l, r),
        ',' => catenate(l, r),
        _ => dyadic(f, l, r),
    }
}
