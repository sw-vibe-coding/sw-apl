//! The domino primitive: what APL gives it and what it hands back.

use apl_value::{AplError, AplResult, Array, Data, Number};

use crate::house::solve;

/// `⌹B`: the inverse of B, or its left inverse when B has more rows
/// than columns -- `(⌹B)+.×B` is the identity either way. A scalar
/// inverts to its reciprocal.
///
/// It is the identity matrix divided by B, which is the same thing
/// and saves saying it twice.
///
/// # Errors
/// DOMAIN ERROR for characters, for a singular B, and for a B with
/// more columns than rows; RANK ERROR above rank 2.
pub fn matrix_inverse(b: &Array) -> AplResult<Array> {
    let (values, rows, cols) = numbers(b)?;
    let mut identity = vec![0.0; rows * rows];
    for i in 0..rows {
        identity[i * rows + i] = 1.0;
    }
    let solved = solve(&values, rows, cols, &identity, rows)?;
    // The result is cols by rows -- the shape reversed -- and keeps
    // the rank it was given: a vector inverts to a vector.
    let shape = match b.shape.len() {
        0 => Vec::new(),
        1 => vec![rows],
        _ => vec![cols, rows],
    };
    Ok(shaped(&solved, shape))
}

/// `A⌹B`: the X for which `B+.×X` is A. When B has more rows than
/// columns there is no exact X, and the answer is the least squares
/// one -- the fit that makes the sum of the squared residuals as
/// small as it can be.
///
/// # Errors
/// As [`matrix_inverse`], and LENGTH ERROR when A and B do not have
/// the same number of rows.
pub fn matrix_divide(a: &Array, b: &Array) -> AplResult<Array> {
    let (right, rows, cols) = numbers(b)?;
    let (left, left_rows, wide) = numbers(a)?;
    if rows != left_rows {
        return Err(AplError::new(apl_value::ErrorKind::Length));
    }
    let solved = solve(&right, rows, cols, &left, wide)?;
    // An axis that came from a vector or a scalar is not in the
    // answer: a matrix over a matrix is a matrix, a vector over a
    // matrix is a vector, and a vector over a vector is the one
    // number that fits it.
    let mut shape = Vec::new();
    if b.shape.len() > 1 {
        shape.push(cols);
    }
    if a.shape.len() > 1 {
        shape.push(wide);
    }
    Ok(shaped(&solved, shape))
}

/// An array of rank 2 or less as row-major doubles with its rows and
/// columns: a vector is one column, a scalar is one by one.
///
/// # Errors
/// DOMAIN ERROR for characters and for more columns than rows; RANK
/// ERROR above rank 2.
fn numbers(a: &Array) -> AplResult<(Vec<f64>, usize, usize)> {
    let kind = |k| Err(AplError::new(k));
    let Data::Num(values) = &a.data else {
        return kind(apl_value::ErrorKind::Domain);
    };
    let (rows, cols) = match a.shape.as_slice() {
        [] => (1, 1),
        [n] => (*n, 1),
        [n, m] => (*n, *m),
        _ => return kind(apl_value::ErrorKind::Rank),
    };
    if cols > rows {
        return kind(apl_value::ErrorKind::Domain);
    }
    Ok((values.iter().map(|n| n.as_f64()).collect(), rows, cols))
}

/// Doubles as an array of the given shape, each demoted to an
/// integer where it is exactly one, as every other primitive does.
fn shaped(values: &[f64], shape: Vec<usize>) -> Array {
    let data = Data::Num(values.iter().map(|x| Number::from_f64(*x)).collect());
    Array { shape, data }
}
