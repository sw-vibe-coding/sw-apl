//! `l↑r` and `l↓r`.

use apl_value::{AplError, AplResult, Array, ErrorKind};

use apl_prims_mixed::int_vector;

use crate::gather::{gather, strides};

/// `l↑r`: the first (or, for negative counts, last) elements along
/// each axis, padding with the fill beyond the array.
///
/// # Errors
/// LENGTH ERROR unless one count per axis (a scalar `r` counts as a
/// one-element vector per count); RANK and DOMAIN from the counts.
pub fn take(l: &Array, r: &Array) -> AplResult<Array> {
    select(l, r, true)
}

/// `l↓r`: all but the first (or last) elements along each axis.
///
/// # Errors
/// As for [`take`].
pub fn drop(l: &Array, r: &Array) -> AplResult<Array> {
    select(l, r, false)
}

/// Take or drop by per-axis signed counts.
fn select(l: &Array, r: &Array, take: bool) -> AplResult<Array> {
    let counts = int_vector(l)?;
    let src: Vec<usize> = if r.shape.is_empty() {
        vec![1; counts.len()]
    } else {
        r.shape.clone()
    };
    if counts.len() != src.len() {
        return Err(AplError::new(ErrorKind::Length));
    }
    let (shape, offsets) = bounds(&counts, &src, take);
    let st = strides(&src);
    let data = gather(&r.data, &shape, |out| {
        let mut axes = out.iter().zip(&offsets).zip(&src).zip(&st);
        axes.try_fold(0usize, |acc, (((&o, &off), &n), &stride)| {
            let i = i64::try_from(o).unwrap_or(i64::MAX) + off;
            usize::try_from(i)
                .ok()
                .filter(|&i| i < n)
                .map(|i| acc + i * stride)
        })
    });
    Array::new(shape, data)
}

/// Result length and source offset per axis for a take (or drop) of
/// `count` elements from an axis of length `n`.
fn bounds(counts: &[i64], src: &[usize], take: bool) -> (Vec<usize>, Vec<i64>) {
    counts
        .iter()
        .zip(src)
        .map(|(&c, &n)| {
            let n = i64::try_from(n).unwrap_or(i64::MAX);
            let (len, off) = match (take, c >= 0) {
                (true, true) => (c, 0),
                (true, false) => (-c, n + c),
                (false, true) => ((n - c).max(0), c),
                (false, false) => ((n + c).max(0), 0),
            };
            (usize::try_from(len).unwrap_or(0), off)
        })
        .unzip()
}
