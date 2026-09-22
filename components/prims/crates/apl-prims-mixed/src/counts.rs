//! An argument that has to be a count: how many, how long, or
//! which of two. Shared by the mixed functions that take one.

use apl_scalar_arith::tolerant_round;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, FUZZ, Number};

/// A number that is a whole number within the fuzz, as the whole
/// number it is. The fuzz is APL\360's fixed one in both modes: `⎕CT`
/// is the tolerance of the relations, floor and ceiling, not of a
/// count.
///
/// The manual: "For operations such as floor and ceiling, and in
/// comparisons, a 'fuzz' of about 1E¯13 is applied in order to avoid
/// anomalous results that might otherwise be engendered by doing
/// decimal arithmetic on a binary machine." Counting is such an
/// operation. `(0.1+0.2)×10` prints as 3, floors to 3 and compares
/// equal to 3, and `⍳` of it refusing would be exactly the anomaly
/// that sentence is about.
fn whole(n: Number) -> Option<i64> {
    match Number::from_f64(tolerant_round(n.as_f64(), FUZZ)?) {
        Number::Int(i) => Some(i),
        Number::Float(_) => None,
    }
}

/// A number that must be a non-negative integer (floats that are
/// integral are accepted).
///
/// # Errors
/// DOMAIN ERROR for negatives and fractions.
pub fn non_negative_int(n: Number) -> AplResult<i64> {
    match whole(n) {
        Some(i) if i >= 0 => Ok(i),
        _ => Err(AplError::new(ErrorKind::Domain)),
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
        .map(|&n| whole(n).ok_or_else(|| AplError::new(ErrorKind::Domain)))
        .collect()
}

/// A scalar or vector of 0s and 1s as booleans.
///
/// # Errors
/// RANK ERROR above rank 1; DOMAIN ERROR for anything but 0 and 1.
pub fn bool_vector(l: &Array) -> AplResult<Vec<bool>> {
    if l.shape.len() > 1 {
        return Err(AplError::new(ErrorKind::Rank));
    }
    let Data::Num(v) = &l.data else {
        return Err(AplError::new(ErrorKind::Domain));
    };
    v.iter()
        .map(|&n| match n {
            Number::Int(0) => Ok(false),
            Number::Int(1) => Ok(true),
            _ => Err(AplError::new(ErrorKind::Domain)),
        })
        .collect()
}
