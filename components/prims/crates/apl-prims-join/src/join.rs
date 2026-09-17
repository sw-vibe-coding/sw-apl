//! `l,r` and `l,[k]r`.

use apl_value::{AplError, AplResult, Array, Data, ErrorKind};

use crate::axis::{Axis, conform};

/// Catenate `l` and `r` along `axis`, or laminate them.
///
/// # Errors
/// INDEX ERROR for an axis beyond the rank; RANK ERROR when the
/// ranks cannot be reconciled; LENGTH ERROR when the other axes
/// disagree; DOMAIN ERROR when mixing numbers and characters.
pub fn catenate(l: &Array, r: &Array, axis: Axis) -> AplResult<Array> {
    let (l, r, k) = align(l, r, axis)?;
    let agree = l.shape.iter().zip(&r.shape).enumerate();
    if !agree.clone().all(|(i, (a, b))| i == k || a == b) {
        return Err(AplError::new(ErrorKind::Length));
    }
    join_along(&l, &r, k)
}

/// Bring both arguments to a common rank and pick the join axis.
/// Two scalars catenate as one-element vectors; laminating inserts
/// the new length-1 axis on both sides first.
fn align(l: &Array, r: &Array, axis: Axis) -> AplResult<(Array, Array, usize)> {
    let laminate = matches!(axis, Axis::Laminate(_));
    let scalars = l.shape.is_empty() && r.shape.is_empty() && !laminate;
    let as_vector = |a: &Array| Array {
        shape: vec![1],
        data: a.data.clone(),
    };
    let (l, r) = if scalars {
        (as_vector(l), as_vector(r))
    } else {
        (l.clone(), r.clone())
    };
    let rank = l.shape.len().max(r.shape.len());
    let k = match axis {
        Axis::Last => rank - 1,
        Axis::At(k) | Axis::Laminate(k) if k < rank + usize::from(laminate) => k,
        _ => return Err(AplError::new(ErrorKind::Index)),
    };
    let (mut l, mut r) = (conform(&l, &r, k, laminate)?, conform(&r, &l, k, laminate)?);
    if laminate {
        l.shape.insert(k, 1);
        r.shape.insert(k, 1);
    }
    Ok((l, r, k))
}

/// Join two arrays of equal rank whose shapes agree off axis `k`.
fn join_along(left: &Array, right: &Array, k: usize) -> AplResult<Array> {
    let mut shape = left.shape.clone();
    shape[k] += right.shape[k];
    let inner: usize = left.shape[k + 1..].iter().product();
    let outer: usize = left.shape[..k].iter().product();
    let (lstep, rstep) = (left.shape[k] * inner, right.shape[k] * inner);
    let data = match (&left.data, &right.data) {
        (Data::Num(lv), Data::Num(rv)) => Data::Num(interleave(lv, lstep, rv, rstep, outer)),
        (Data::Char(lv), Data::Char(rv)) => Data::Char(interleave(lv, lstep, rv, rstep, outer)),
        (_, other) if left.data.count() == 0 => other.clone(),
        (mine, _) if right.data.count() == 0 => mine.clone(),
        _ => return Err(AplError::new(ErrorKind::Domain)),
    };
    Array::new(shape, data)
}

/// For each of `outer` blocks, `lstep` elements of `left` then
/// `rstep` elements of `right`.
fn interleave<T: Clone>(
    left: &[T],
    lstep: usize,
    right: &[T],
    rstep: usize,
    outer: usize,
) -> Vec<T> {
    (0..outer)
        .flat_map(|block| {
            let lpart = &left[block * lstep..(block + 1) * lstep];
            let rpart = &right[block * rstep..(block + 1) * rstep];
            lpart.iter().chain(rpart).cloned()
        })
        .collect()
}
