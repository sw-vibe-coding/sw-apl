//! Element search with the fuzz for numbers.

use apl_value::{AplError, AplResult, Array, Data, ErrorKind, FUZZ, Number};

/// `l∊r`: 1 where an element of `l` occurs anywhere in `r`.
#[must_use]
pub fn membership(l: &Array, r: &Array) -> Array {
    let count = l.data.count();
    let hits = (0..count)
        .map(|i| (0..r.data.count()).any(|j| same(&l.data, i, &r.data, j)))
        .map(|hit| Number::Int(hit.into()))
        .collect();
    Array {
        shape: l.shape.clone(),
        data: Data::Num(hits),
    }
}

/// `l⍳r`: for each element of `r`, the index (in the origin) of its
/// first occurrence in the vector `l`, or one past the end.
///
/// # Errors
/// RANK ERROR unless `l` is a vector.
pub fn index_of(l: &Array, r: &Array, io: i64) -> AplResult<Array> {
    if l.shape.len() != 1 {
        return Err(AplError::new(ErrorKind::Rank));
    }
    let n = l.data.count();
    let found = (0..r.data.count())
        .map(|j| (0..n).find(|&i| same(&l.data, i, &r.data, j)).unwrap_or(n))
        .map(|i| Number::Int(io + i64::try_from(i).unwrap_or(0)))
        .collect();
    Array::new(r.shape.clone(), Data::Num(found))
}

/// Element `i` of `a` equals element `j` of `b` (tolerantly for
/// numbers; never across types).
fn same(left: &Data, i: usize, right: &Data, j: usize) -> bool {
    match (left, right) {
        (Data::Num(nums), Data::Num(others)) => nums[i].tolerant_eq(others[j], FUZZ),
        (Data::Char(text), Data::Char(other)) => text[i] == other[j],
        _ => false,
    }
}
