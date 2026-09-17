//! Reverse, rotate, transpose.

use apl_prims_mixed::{int_vector, ravel};
use apl_value::{AplError, AplResult, Array, ErrorKind};

use crate::gather::{gather, strides, transpose_axes};

/// `⌽r` along axis `k`.
#[must_use]
pub fn reverse(r: &Array, k: usize) -> Array {
    turn(r, k, |_, o, n| n - 1 - o)
}

/// `l⌽r` along axis `k`: `l` is a scalar shift or an array of shifts
/// shaped like `r` without axis `k`.
///
/// # Errors
/// RANK ERROR or LENGTH ERROR when the shifts do not fit; DOMAIN
/// ERROR for non-integer shifts.
pub fn rotate(l: &Array, r: &Array, k: usize) -> AplResult<Array> {
    let shifts = int_vector(&ravel(l))?;
    let mut rest = r.shape.clone();
    if !r.shape.is_empty() {
        rest.remove(k);
    }
    if !l.shape.is_empty() && l.shape != rest {
        let kind = [ErrorKind::Rank, ErrorKind::Length][usize::from(l.shape.len() == rest.len())];
        return Err(AplError::new(kind));
    }
    Ok(turn(r, k, |row, o, n| {
        let shift = shifts[if l.shape.is_empty() { 0 } else { row }];
        let turned =
            (i64::try_from(o).unwrap_or(0) + shift).rem_euclid(i64::try_from(n).unwrap_or(1));
        usize::try_from(turned).unwrap_or(0)
    }))
}

/// Rearrange along axis `k`: `pick(row, o, n)` gives the source
/// position along `k` (of length `n`) for result position `o`, where
/// `row` numbers the positions of the other axes in row-major order.
/// A scalar is unchanged.
fn turn(r: &Array, k: usize, pick: impl Fn(usize, usize, usize) -> usize) -> Array {
    if r.shape.is_empty() {
        return r.clone();
    }
    let (st, n) = (strides(&r.shape), r.shape[k]);
    let mut rest = r.shape.clone();
    rest.remove(k);
    let rest_st = strides(&rest);
    let data = gather(&r.data, &r.shape, |out| {
        let mut without = out.to_vec();
        without.remove(k);
        let row = without.iter().zip(&rest_st).map(|(o, s)| o * s).sum();
        let flat: usize = out.iter().zip(&st).map(|(o, s)| o * s).sum();
        Some(flat - out[k] * st[k] + pick(row, out[k], n) * st[k])
    });
    Array {
        shape: r.shape.clone(),
        data,
    }
}

/// `l⍉r`: axis `i` of `r` becomes axis `l[i]` of the result (in the
/// index origin); repeated axes select diagonals. Without `l` the
/// axes are reversed (`⍉r`).
///
/// # Errors
/// LENGTH ERROR unless one entry per axis; DOMAIN ERROR unless the
/// entries are the result axes with none missing.
pub fn transpose(l: Option<&Array>, r: &Array, io: i64) -> AplResult<Array> {
    let axes = transpose_axes(l, r.shape.len(), io)?;
    if axes.len() != r.shape.len() {
        return Err(AplError::new(ErrorKind::Length));
    }
    let out_rank = axes.iter().max().map_or(0, |m| m + 1);
    let mut shape = vec![usize::MAX; out_rank];
    for (&target, &n) in axes.iter().zip(&r.shape) {
        shape[target] = shape[target].min(n);
    }
    if shape.contains(&usize::MAX) {
        return Err(AplError::new(ErrorKind::Domain));
    }
    let st = strides(&r.shape);
    let data = gather(&r.data, &shape, |out| {
        Some(axes.iter().zip(&st).map(|(&a, &s)| out[a] * s).sum())
    });
    Array::new(shape, data)
}
