//! Index generator and integer-argument parsing.

use apl_prims_scalar::numbers;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

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
///
/// # Errors
/// DOMAIN ERROR for negatives and fractions.
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

/// A scalar or vector left argument as integers.
///
/// # Errors
/// RANK ERROR above rank 1; DOMAIN ERROR for characters or fractions.
pub fn int_vector(l: &Array) -> AplResult<Vec<i64>> {
    if l.shape.len() > 1 {
        return Err(AplError::new(ErrorKind::Rank));
    }
    let Data::Num(v) = &l.data else {
        return Err(AplError::new(ErrorKind::Domain));
    };
    v.iter()
        .map(|&n| match n {
            Number::Int(i) => Ok(i),
            Number::Float(_) => Err(AplError::new(ErrorKind::Domain)),
        })
        .collect()
}
