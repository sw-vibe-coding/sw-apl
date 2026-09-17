//! Position mapping shared by the selection functions.

use apl_prims_join::{Axis, resolve_axis};
use apl_prims_mixed::int_vector;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

/// Row-major strides of `shape`.
#[must_use]
pub fn strides(shape: &[usize]) -> Vec<usize> {
    let mut strides = vec![1; shape.len()];
    for i in (1..shape.len()).rev() {
        strides[i - 1] = strides[i] * shape[i];
    }
    strides
}

/// Build data of `shape` by mapping each result position to a
/// source element of `data`; `None` selects the fill (0 or blank).
pub fn gather(data: &Data, shape: &[usize], map: impl Fn(&[usize]) -> Option<usize>) -> Data {
    let count: usize = shape.iter().product();
    let mut multi = vec![0usize; shape.len()];
    let mut picks = Vec::with_capacity(count);
    for _ in 0..count {
        picks.push(map(&multi));
        for ax in (0..shape.len()).rev() {
            multi[ax] += 1;
            if multi[ax] < shape[ax] {
                break;
            }
            multi[ax] = 0;
        }
    }
    match data {
        Data::Num(v) => Data::Num(
            picks
                .iter()
                .map(|p| p.map_or(Number::Int(0), |i| v[i]))
                .collect(),
        ),
        Data::Char(v) => Data::Char(picks.iter().map(|p| p.map_or(' ', |i| v[i])).collect()),
    }
}

/// The result axis for each source axis of a transpose: reversed
/// axes without `l`, else `l` in the index origin.
///
/// # Errors
/// DOMAIN ERROR for an axis below the origin.
pub fn transpose_axes(l: Option<&Array>, rank: usize, io: i64) -> AplResult<Vec<usize>> {
    let Some(l) = l else {
        return Ok((0..rank).rev().collect());
    };
    int_vector(l)?
        .iter()
        .map(|&a| usize::try_from(a - io).map_err(|_| AplError::new(ErrorKind::Domain)))
        .collect()
}

/// The 0-based axis for a reverse or rotate: the bracket if given,
/// else the last axis (or the first for the bar forms).
///
/// # Errors
/// INDEX ERROR for a fractional or out-of-range bracket.
pub fn axis_index(axis: Option<&Array>, first: bool, rank: usize, io: i64) -> AplResult<usize> {
    match resolve_axis(axis, io, rank)? {
        Axis::Last if first => Ok(0),
        Axis::Last => Ok(rank.saturating_sub(1)),
        Axis::At(k) => Ok(k),
        Axis::Laminate(_) => Err(AplError::new(ErrorKind::Index)),
    }
}
