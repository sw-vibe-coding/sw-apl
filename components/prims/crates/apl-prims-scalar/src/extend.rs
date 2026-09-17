//! Scalar extension: pair up elements, check rank and length.

use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

use crate::dispatch::{DYADIC, MONADIC, apply_dyadic, apply_monadic};

/// Apply monadic scalar function `f` to every element of `r`.
///
/// # Errors
/// DOMAIN ERROR from the function; NOT IMPLEMENTED for other glyphs.
pub fn monadic(f: char, r: &Array) -> AplResult<Array> {
    if !MONADIC.contains(f) {
        return Err(AplError::new(ErrorKind::NotImplemented));
    }
    let out = numbers(r)?
        .iter()
        .map(|&x| apply_monadic(f, x))
        .collect::<AplResult<Vec<_>>>()?;
    Ok(Array {
        shape: r.shape.clone(),
        data: Data::Num(out),
    })
}

/// Apply dyadic scalar function `f` with scalar extension.
///
/// # Errors
/// RANK ERROR when ranks differ (and neither is a scalar), LENGTH
/// ERROR when shapes differ, DOMAIN ERROR from the function.
pub fn dyadic(f: char, l: &Array, r: &Array) -> AplResult<Array> {
    if !DYADIC.contains(f) {
        return Err(AplError::new(ErrorKind::NotImplemented));
    }
    let (ln, rn) = (numbers(l)?, numbers(r)?);
    let shape = agree(l, r)?;
    let n = shape.iter().product::<usize>();
    // After `agree`, each side is either a scalar (one element,
    // reused) or has exactly `n` elements.
    let pick = |v: &[Number], i: usize| v[if v.len() == 1 { 0 } else { i }];
    let out = (0..n)
        .map(|i| apply_dyadic(f, pick(ln, i), pick(rn, i)))
        .collect::<AplResult<Vec<_>>>()?;
    Ok(Array {
        shape,
        data: Data::Num(out),
    })
}

/// The result shape of a dyadic scalar function.
fn agree(l: &Array, r: &Array) -> AplResult<Vec<usize>> {
    if l.shape.is_empty() {
        return Ok(r.shape.clone());
    }
    if r.shape.is_empty() {
        return Ok(l.shape.clone());
    }
    if l.shape.len() != r.shape.len() {
        return Err(AplError::new(ErrorKind::Rank));
    }
    if l.shape != r.shape {
        return Err(AplError::new(ErrorKind::Length));
    }
    Ok(l.shape.clone())
}

/// The numeric elements of `a`.
///
/// # Errors
/// DOMAIN ERROR for character data.
pub fn numbers(a: &Array) -> AplResult<&[Number]> {
    match &a.data {
        Data::Num(v) => Ok(v),
        Data::Char(_) => Err(AplError::new(ErrorKind::Domain)),
    }
}
