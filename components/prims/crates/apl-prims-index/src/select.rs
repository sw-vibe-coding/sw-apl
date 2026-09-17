//! Resolving an index list to source positions.

use apl_prims_select::strides;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

/// The positions an index list selects, in result order, and the
/// result shape (the index shapes catenated; an elided axis counts
/// as a vector of the whole axis).
pub struct Selection {
    pub shape: Vec<usize>,
    pub positions: Vec<usize>,
}

/// Resolve `indexes` against `array` in the index origin `io`.
///
/// # Errors
/// RANK ERROR unless there is one index per axis; DOMAIN ERROR for
/// non-integer indexes; INDEX ERROR for an index outside the axis.
pub fn select(array: &Array, indexes: &[Option<Array>], io: i64) -> AplResult<Selection> {
    if indexes.len() != array.shape.len() {
        return Err(AplError::new(ErrorKind::Rank));
    }
    let per_axis: Vec<(Vec<usize>, Vec<usize>)> = indexes
        .iter()
        .zip(&array.shape)
        .map(|(idx, &n)| axis_values(idx.as_ref(), n, io))
        .collect::<AplResult<_>>()?;
    let shape: Vec<usize> = per_axis
        .iter()
        .flat_map(|(s, _)| s.iter().copied())
        .collect();
    let counts: Vec<usize> = per_axis.iter().map(|(_, v)| v.len()).collect();
    let st = strides(&array.shape);
    let positions = (0..counts.iter().product::<usize>())
        .map(|flat| position(flat, &per_axis, &counts, &st))
        .collect();
    Ok(Selection { shape, positions })
}

/// The source position of result element `flat`: decompose it over
/// the per-axis index counts (last axis fastest) and look each
/// coordinate up in its index list.
fn position(
    flat: usize,
    per_axis: &[(Vec<usize>, Vec<usize>)],
    counts: &[usize],
    st: &[usize],
) -> usize {
    let mut rest = flat;
    let mut src = 0;
    for axis in (0..counts.len()).rev() {
        src += per_axis[axis].1[rest % counts[axis]] * st[axis];
        rest /= counts[axis];
    }
    src
}

/// One axis of the index list: its shape and 0-based positions.
fn axis_values(idx: Option<&Array>, n: usize, io: i64) -> AplResult<(Vec<usize>, Vec<usize>)> {
    let Some(idx) = idx else {
        return Ok((vec![n], (0..n).collect()));
    };
    let Data::Num(values) = &idx.data else {
        return Err(AplError::new(ErrorKind::Domain));
    };
    let positions = values
        .iter()
        .map(|&v| match v {
            Number::Int(i) => usize::try_from(i - io)
                .ok()
                .filter(|&p| p < n)
                .ok_or(AplError::new(ErrorKind::Index)),
            Number::Float(_) => Err(AplError::new(ErrorKind::Domain)),
        })
        .collect::<AplResult<_>>()?;
    Ok((idx.shape.clone(), positions))
}
