//! Reading and writing through an index list.

use apl_value::{AplError, AplResult, Array, Data, ErrorKind};

use crate::select::select;

/// `array[indexes]`.
///
/// # Errors
/// As for selection: RANK, DOMAIN, or INDEX ERROR.
pub fn index(array: &Array, indexes: &[Option<Array>], io: i64) -> AplResult<Array> {
    let sel = select(array, indexes, io)?;
    let data = match &array.data {
        Data::Num(v) => Data::Num(sel.positions.iter().map(|&p| v[p]).collect()),
        Data::Char(v) => Data::Char(sel.positions.iter().map(|&p| v[p]).collect()),
    };
    Array::new(sel.shape, data)
}

/// `array[indexes]←value`: the array with the selected positions
/// replaced; a scalar value extends, otherwise its shape must match
/// the selection.
///
/// # Errors
/// RANK or LENGTH ERROR when the value does not fit; DOMAIN ERROR
/// when the value's type differs from the array's; selection errors.
pub fn indexed_assign(
    array: &Array,
    indexes: &[Option<Array>],
    value: &Array,
    io: i64,
) -> AplResult<Array> {
    let sel = select(array, indexes, io)?;
    if !value.shape.is_empty() && value.shape != sel.shape {
        let kind = if value.shape.len() == sel.shape.len() {
            ErrorKind::Length
        } else {
            ErrorKind::Rank
        };
        return Err(AplError::new(kind));
    }
    let pick = |i: usize| if value.shape.is_empty() { 0 } else { i };
    let data = match (&array.data, &value.data) {
        (Data::Num(a), Data::Num(v)) => Data::Num(written(a, &sel.positions, |i| v[pick(i)])),
        (Data::Char(a), Data::Char(v)) => Data::Char(written(a, &sel.positions, |i| v[pick(i)])),
        _ => return Err(AplError::new(ErrorKind::Domain)),
    };
    Array::new(array.shape.clone(), data)
}

/// A copy of `data` with `positions[i]` set to `value(i)`.
fn written<T: Clone>(data: &[T], positions: &[usize], value: impl Fn(usize) -> T) -> Vec<T> {
    let mut out = data.to_vec();
    for (i, &p) in positions.iter().enumerate() {
        out[p] = value(i);
    }
    out
}
