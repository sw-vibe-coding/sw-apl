//! Mixed-radix conversion.

use apl_prims_scalar::numbers;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

use crate::place::{digits_of, weights};

/// `l⊥r`: the value of the digits `r` in the radix `l`. A vector of
/// digits decodes to a scalar; the columns of a matrix decode to a
/// vector. A scalar on either side extends to the other's length.
///
/// # Errors
/// RANK ERROR above rank 1 (`l`) or 2 (`r`); LENGTH ERROR when the
/// radix and digit counts differ.
pub fn decode(l: &Array, r: &Array) -> AplResult<Array> {
    if l.shape.len() > 1 || r.shape.len() > 2 {
        return Err(AplError::new(ErrorKind::Rank));
    }
    let (radix, digits) = (numbers(l)?, numbers(r)?);
    let (rows, cols) = layout(r, radix.len());
    // The APL\360 User's Manual, page 3.42: either argument may be a
    // scalar or a one-element vector; otherwise the lengths agree.
    if radix.len() != 1 && radix.len() != rows {
        return Err(AplError::new(ErrorKind::Length));
    }
    let base: Vec<f64> = (0..rows).map(|i| radix[i % radix.len()].as_f64()).collect();
    let w = weights(&base);
    let digit = |i: usize, c: usize| digits[(i * cols + c) % digits.len()].as_f64();
    let values = (0..cols)
        .map(|c| Number::from_f64((0..rows).map(|i| digit(i, c) * w[i]).sum()))
        .collect();
    let shape = if r.shape.len() == 2 {
        vec![cols]
    } else {
        vec![]
    };
    Array::new(shape, Data::Num(values))
}

/// Digit rows and columns of `r`: a scalar or a one-element vector
/// supplies `n` equal
/// digits, a vector one column, a matrix one column per column.
fn layout(r: &Array, n: usize) -> (usize, usize) {
    match r.shape.as_slice() {
        [] | [1] => (n.max(1), 1),
        [rows] => (*rows, 1),
        [rows, cols] => (*rows, *cols),
        _ => unreachable!("rank checked by the caller"),
    }
}

/// `l⊤r`: the digits of each element of `r` in the radix `l`; the
/// result shape is the radix shape followed by the value shape.
///
/// # Errors
/// RANK ERROR when `l` is above rank 1; DOMAIN ERROR for characters.
pub fn encode(l: &Array, r: &Array) -> AplResult<Array> {
    if l.shape.len() > 1 {
        return Err(AplError::new(ErrorKind::Rank));
    }
    let (radix, values) = (numbers(l)?, numbers(r)?);
    let per_value: Vec<Vec<f64>> = values
        .iter()
        .map(|v| digits_of(radix, v.as_f64()))
        .collect();
    let data = (0..radix.len())
        .flat_map(|i| per_value.iter().map(move |d| Number::from_f64(d[i])))
        .collect();
    let shape = [l.shape.as_slice(), r.shape.as_slice()].concat();
    Array::new(shape, Data::Num(data))
}
