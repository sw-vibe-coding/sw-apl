//! Boolean selection along one axis.

use apl_prims_mixed::{bool_vector, reshape};
use apl_prims_select::{gather, strides};
use apl_value::{AplError, AplResult, Array, ErrorKind, Number};

/// `l/r` along axis `k`: keep the positions where `l` is 1.
///
/// # Errors
/// DOMAIN ERROR unless `l` is boolean; RANK ERROR above rank 1;
/// LENGTH ERROR when its length differs from the axis (a scalar
/// extends either side).
pub fn compress(l: &Array, r: &Array, k: usize) -> AplResult<Array> {
    let mask = bool_vector(l)?;
    let r = extend_scalar(r, mask.len());
    let mask = if mask.len() == 1 {
        vec![mask[0]; r.shape[k]]
    } else {
        mask
    };
    if mask.len() != r.shape[k] {
        return Err(AplError::new(ErrorKind::Length));
    }
    let picks: Vec<Option<usize>> = mask
        .iter()
        .enumerate()
        .filter(|(_, m)| **m)
        .map(|(i, _)| Some(i))
        .collect();
    along_axis(&r, k, &picks)
}

/// `l\r` along axis `k`: source elements at the 1s, fill at the 0s.
///
/// # Errors
/// As for [`compress`], where the count of 1s must equal the axis.
pub fn expand(l: &Array, r: &Array, k: usize) -> AplResult<Array> {
    let mask = bool_vector(l)?;
    let ones = mask.iter().filter(|&&m| m).count();
    let r = extend_scalar(r, ones);
    if ones != r.shape[k] {
        return Err(AplError::new(ErrorKind::Length));
    }
    let mut next = 0;
    let picks: Vec<Option<usize>> = mask
        .iter()
        .map(|&m| {
            next += usize::from(m);
            m.then(|| next - 1)
        })
        .collect();
    along_axis(&r, k, &picks)
}

/// A scalar right argument becomes a vector of `len` copies.
fn extend_scalar(r: &Array, len: usize) -> Array {
    if r.shape.is_empty() {
        reshape(
            &Array::scalar(Number::Int(i64::try_from(len).unwrap_or(0))),
            r,
        )
        .unwrap_or_else(|_| r.clone())
    } else {
        r.clone()
    }
}

/// Rebuild `r` with axis `k` replaced by the given source positions
/// (`None` is the fill).
fn along_axis(r: &Array, k: usize, picks: &[Option<usize>]) -> AplResult<Array> {
    let st = strides(&r.shape);
    let mut shape = r.shape.clone();
    shape[k] = picks.len();
    let data = gather(&r.data, &shape, |out| {
        let flat: usize = out.iter().zip(&st).map(|(o, s)| o * s).sum();
        picks[out[k]].map(|p| flat - out[k] * st[k] + p * st[k])
    });
    Array::new(shape, data)
}
