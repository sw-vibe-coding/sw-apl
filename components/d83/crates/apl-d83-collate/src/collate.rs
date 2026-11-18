//! Where each character falls in the collating sequence, and the
//! order of the items by it.

use std::cmp::Ordering;

use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

/// `l⍋r`, or `l⍒r` when `descending`: the permutation, from `io`, that
/// puts the items along the first axis of `r` in order. A character's
/// place is where it is first found in `l`, read in row-major order,
/// as an index along each of `l`'s axes; one not found is just past
/// the end on every axis. The last axis counts most and the first
/// least, so a sequence of a row of small letters over a row of
/// capitals sorts by spelling first and by case after. Equal items
/// keep their order.
///
/// # Errors
/// DOMAIN ERROR unless both are characters, RANK ERROR when either is
/// a scalar.
pub fn grade(l: &Array, r: &Array, descending: bool, io: i64) -> AplResult<Array> {
    let (seq, items) = (characters(l)?, characters(r)?);
    let n = r.shape[0];
    let size = items.len().checked_div(n).unwrap_or(0);
    let keys: Vec<Vec<usize>> = (0..n)
        .map(|i| key(&l.shape, seq, &items[i * size..(i + 1) * size]))
        .collect();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| {
        let o: Ordering = keys[a].cmp(&keys[b]);
        if descending { o.reverse() } else { o }
    });
    let at = |i: usize| Number::Int(i64::try_from(i).unwrap_or(i64::MAX) + io);
    Ok(Array {
        shape: vec![n],
        data: Data::Num(order.into_iter().map(at).collect()),
    })
}

/// The characters of an array that is not a scalar.
fn characters(a: &Array) -> AplResult<&[char]> {
    match &a.data {
        Data::Num(_) => Err(AplError::new(ErrorKind::Domain)),
        Data::Char(_) if a.shape.is_empty() => Err(AplError::new(ErrorKind::Rank)),
        Data::Char(chars) => Ok(chars),
    }
}

/// What an item sorts by: the places of its characters along the last
/// axis of the sequence, then along the axis before, and so on to the
/// first.
fn key(shape: &[usize], seq: &[char], item: &[char]) -> Vec<usize> {
    let places: Vec<Vec<usize>> = item.iter().map(|c| place(shape, seq, *c)).collect();
    (0..shape.len())
        .rev()
        .flat_map(|axis| places.iter().map(move |p| p[axis]))
        .collect()
}

/// Where `c` is first found in the sequence, as an index along each
/// axis; just past the end on every axis when it is not there.
fn place(shape: &[usize], seq: &[char], c: char) -> Vec<usize> {
    let Some(mut at) = seq.iter().position(|&s| s == c) else {
        return shape.to_vec();
    };
    let mut index = vec![0; shape.len()];
    for (axis, &len) in shape.iter().enumerate().rev() {
        index[axis] = at % len;
        at /= len;
    }
    index
}
