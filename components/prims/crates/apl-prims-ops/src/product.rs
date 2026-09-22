//! Inner product `l f.g r` and outer product `l ∘.g r`.

use apl_prims_scalar::{DYADIC, Element, elements, related};
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

use crate::reduce::fold;

/// `l ∘.g r`: `g` applied to every pair; the result shape is the
/// shape of `l` followed by the shape of `r`.
///
/// # Errors
/// DOMAIN ERROR from `g`, or for character data unless `g` is = or
/// ≠; SYNTAX ERROR when `g` has no scalar dyadic form.
pub fn outer(g: char, left: &Array, right: &Array, ct: f64) -> AplResult<Array> {
    if !DYADIC.contains(g) {
        return Err(AplError::new(ErrorKind::Syntax));
    }
    let (lv, rv) = (elements(left, g)?, elements(right, g)?);
    let data = lv
        .iter()
        .flat_map(|&a| rv.iter().map(move |&b| related(g, a, b, ct)))
        .collect::<AplResult<Vec<_>>>()?;
    let shape = [left.shape.as_slice(), right.shape.as_slice()].concat();
    Array::new(shape, Data::Num(data))
}

/// `l f.g r`: `g` between the last axis of `l` and the first axis of
/// `r`, reduced by `f`. A scalar argument extends along that axis.
///
/// # Errors
/// LENGTH ERROR when the shared axes differ; DOMAIN ERROR from the
/// functions; SYNTAX ERROR when either lacks a scalar dyadic form.
pub fn inner(f: char, g: char, left: &Array, right: &Array, ct: f64) -> AplResult<Array> {
    if !DYADIC.contains(f) || !DYADIC.contains(g) {
        return Err(AplError::new(ErrorKind::Syntax));
    }
    let shared = if right.shape.is_empty() {
        *left.shape.last().unwrap_or(&1)
    } else {
        right.shape[0]
    };
    let (lshape, lv) = extend(left, shared, g)?;
    let (rshape, rv) = extend(right, shared, g)?;
    if lshape[lshape.len() - 1] != rshape[0] {
        return Err(AplError::new(ErrorKind::Length));
    }
    let rows: usize = lshape[..lshape.len() - 1].iter().product();
    let cols: usize = rshape[1..].iter().product();
    let out = (0..rows * cols)
        .map(|at| cell((f, g, ct), (&lv, &rv), shared, cols, at))
        .collect::<AplResult<Vec<_>>>()?;
    let shape = [&lshape[..lshape.len() - 1], &rshape[1..]].concat();
    Array::new(shape, Data::Num(out))
}

/// One result element: the `f`-reduction of `g` over the shared axis
/// for the row and column that `at` names (row-major).
fn cell(
    (f, g, ct): (char, char, f64),
    (lv, rv): (&[Element], &[Element]),
    shared: usize,
    cols: usize,
    at: usize,
) -> AplResult<Number> {
    let (row, col) = (at / cols, at % cols);
    let terms = (0..shared)
        .map(|i| related(g, lv[row * shared + i], rv[i * cols + col], ct))
        .collect::<AplResult<Vec<_>>>()?;
    fold(f, terms.into_iter().map(Element::Num), ct)
}

/// The shape and elements of an inner-product argument for `g`; a
/// scalar becomes a vector of `n` copies (the shared axis).
fn extend(x: &Array, n: usize, g: char) -> AplResult<(Vec<usize>, Vec<Element>)> {
    let values = elements(x, g)?;
    if x.shape.is_empty() {
        return Ok((vec![n], vec![values[0]; n]));
    }
    Ok((x.shape.clone(), values))
}
