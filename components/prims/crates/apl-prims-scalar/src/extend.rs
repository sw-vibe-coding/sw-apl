//! Scalar extension: pair up elements, check rank and length.

use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

use crate::dispatch::{DYADIC, MONADIC, apply_dyadic, apply_monadic};
use crate::element::{elements, pairs, related};

/// Apply monadic scalar function `f` to every element of `r`, within
/// comparison tolerance `ct`.
///
/// # Errors
/// DOMAIN ERROR from the function; SYNTAX ERROR for a glyph with
/// no monadic scalar form.
pub fn monadic(f: char, r: &Array, ct: f64) -> AplResult<Array> {
    if !MONADIC.contains(f) {
        return Err(AplError::new(ErrorKind::Syntax));
    }
    let out = numbers(r)?
        .iter()
        .map(|&x| apply_monadic(f, x, ct))
        .collect::<AplResult<Vec<_>>>()?;
    Ok(Array {
        shape: r.shape.clone(),
        data: Data::Num(out),
    })
}

/// Apply dyadic scalar function `f` with scalar extension, within
/// comparison tolerance `ct`.
///
/// # Errors
/// DOMAIN ERROR for characters, except with = and ≠; RANK ERROR when
/// ranks differ and LENGTH ERROR when shapes differ,
/// unless either side is a scalar or a one-element array; DOMAIN ERROR from the function, and
/// SYNTAX ERROR for a glyph with no dyadic scalar form.
pub fn dyadic(f: char, l: &Array, r: &Array, ct: f64) -> AplResult<Array> {
    if !DYADIC.contains(f) {
        return Err(AplError::new(ErrorKind::Syntax));
    }
    // Numbers, the usual case, are paired where they lie. Characters,
    // which only = and ≠ take, are paired as elements; asking for them
    // before the shapes keeps DOMAIN ERROR ahead of LENGTH ERROR.
    let chars = [l, r].iter().any(|a| matches!(a.data, Data::Char(_)));
    let held = chars.then(|| Ok((elements(l, f)?, elements(r, f)?)));
    let held = held.transpose()?;
    let shape = agree(l, r)?;
    let n = shape.iter().product::<usize>();
    let out = match &held {
        None => pairs(numbers(l)?, numbers(r)?, n, |a, b| {
            apply_dyadic(f, a, b, ct)
        })?,
        Some((lc, rc)) => pairs(lc, rc, n, |a, b| related(f, a, b, ct))?,
    };
    Ok(Array {
        shape,
        data: Data::Num(out),
    })
}

/// The result shape of a dyadic scalar function.
///
/// The APL\360 User's Manual, page 3.33: arguments of different sizes
/// are a length or rank error unless one is a scalar or a one-element
/// array, whose single element then goes with each element of the
/// other. When both are one-element arrays of different shapes the
/// manual does not say which shape the result takes; sw-apl gives it
/// the higher rank's, which keeps a scalar with a one-element vector
/// what it always was.
fn agree(l: &Array, r: &Array) -> AplResult<Vec<usize>> {
    if l.shape == r.shape {
        return Ok(l.shape.clone());
    }
    let lone = |a: &Array| a.shape.iter().product::<usize>() == 1;
    match (lone(l), lone(r)) {
        (true, true) if l.shape.len() > r.shape.len() => Ok(l.shape.clone()),
        (true, _) => Ok(r.shape.clone()),
        (false, true) => Ok(l.shape.clone()),
        _ if l.shape.len() != r.shape.len() => Err(AplError::new(ErrorKind::Rank)),
        _ => Err(AplError::new(ErrorKind::Length)),
    }
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
