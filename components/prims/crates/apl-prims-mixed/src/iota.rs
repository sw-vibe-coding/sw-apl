//! The index generator.

use apl_prims_scalar::numbers;
use apl_value::{AplError, AplResult, Array, ErrorKind, Number};

use crate::counts::non_negative_int;

/// `⍳n`: the first `n` indexes counting from the index origin `io`.
///
/// `n` is a scalar or a one-element vector, as APL\360 takes it. The
/// shape of a vector is a one-element vector, so `⍳⍴V` -- the idiom
/// iota serves more than any other, and the one the nub is built on --
/// needs the second.
///
/// # Errors
/// RANK ERROR unless `r` is a scalar or a one-element vector: a higher
/// rank is refused even with one element, as is a vector of any other
/// length. DOMAIN ERROR unless it is a non-negative integer.
pub fn iota(r: &Array, io: i64) -> AplResult<Array> {
    if r.shape.len() > 1 || r.shape.iter().product::<usize>() != 1 {
        return Err(AplError::new(ErrorKind::Rank));
    }
    let n = non_negative_int(numbers(r)?[0])?;
    Ok(Array::vector((0..n).map(|i| Number::Int(io + i)).collect()))
}
