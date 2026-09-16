//! Shape and reshape.

use apl_prims_scalar::numbers;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

use crate::iota::non_negative_int;

/// `⍴r`: the shape as a vector.
#[must_use]
pub fn shape(r: &Array) -> Array {
    Array::vector(
        r.shape
            .iter()
            .map(|&n| Number::Int(i64::try_from(n).unwrap_or(i64::MAX)))
            .collect(),
    )
}

/// `l⍴r`: the elements of `r` cycled into shape `l`. An empty `r`
/// fills with zero (or blank for characters).
///
/// # Errors
/// RANK ERROR when `l` is not a scalar or vector; DOMAIN ERROR when
/// it holds anything but non-negative integers.
pub fn reshape(l: &Array, r: &Array) -> AplResult<Array> {
    if l.shape.len() > 1 {
        return Err(AplError::new(ErrorKind::Rank));
    }
    let shape = numbers(l)?
        .iter()
        .map(|&n| {
            non_negative_int(n)
                .and_then(|i| usize::try_from(i).map_err(|_| AplError::new(ErrorKind::Domain)))
        })
        .collect::<AplResult<Vec<_>>>()?;
    let count = shape.iter().product::<usize>();
    let data = match &r.data {
        Data::Num(v) => Data::Num(cycle(v, count, Number::Int(0))),
        Data::Char(v) => Data::Char(cycle(v, count, ' ')),
    };
    Array::new(shape, data)
}

/// The first `count` elements of `v` repeated, or `fill` when empty.
fn cycle<T: Clone>(v: &[T], count: usize, fill: T) -> Vec<T> {
    if v.is_empty() {
        return vec![fill; count];
    }
    v.iter().cycle().take(count).cloned().collect()
}
