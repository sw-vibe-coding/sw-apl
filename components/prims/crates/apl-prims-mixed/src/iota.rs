//! Index generator.

use apl_prims_scalar::numbers;
use apl_value::{AplError, AplResult, Array, ErrorKind, Number};

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

/// A number that must be a non-negative integer (floats that are
/// integral are accepted).
pub fn non_negative_int(n: Number) -> AplResult<i64> {
    let x = n.as_f64();
    if x < 0.0 || x.fract() != 0.0 {
        return Err(AplError::new(ErrorKind::Domain));
    }
    match Number::from_f64(x) {
        Number::Int(i) => Ok(i),
        Number::Float(_) => Err(AplError::new(ErrorKind::Domain)),
    }
}
