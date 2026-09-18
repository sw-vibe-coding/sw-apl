//! The index generator.

use apl_prims_scalar::numbers;
use apl_value::{AplError, AplResult, Array, ErrorKind, Number};

use crate::counts::non_negative_int;

/// `⍳n`: the first `n` indexes counting from the index origin `io`.
///
/// # Errors
/// RANK ERROR unless `r` is a scalar; DOMAIN ERROR unless it is a
/// non-negative integer.
pub fn iota(r: &Array, io: i64) -> AplResult<Array> {
    if !r.shape.is_empty() {
        return Err(AplError::new(ErrorKind::Rank));
    }
    let n = non_negative_int(numbers(r)?[0])?;
    Ok(Array::vector((0..n).map(|i| Number::Int(io + i)).collect()))
}
