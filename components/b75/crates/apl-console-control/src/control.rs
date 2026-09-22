//! What `⎕CC` answers: 1 when the 5110 would have done what was
//! asked, 0 when it would not.

use std::ops::RangeInclusive;

use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

/// The characters that choose a national character set, the first
/// being EBCDIC's and the rest the countries' (the manual's list).
const SETS: &str = ".0123456789÷×-+";

/// `⎕CC r`: choose the national character set `r` names. 1 for one
/// of the manual's characters, 0 for anything else.
///
/// # Errors
/// DOMAIN ERROR unless `r` is characters.
pub fn national(r: &Array) -> AplResult<Array> {
    let Data::Char(chars) = &r.data else {
        return Err(AplError::new(ErrorKind::Domain));
    };
    let one = matches!(chars.as_slice(), [c] if SETS.contains(*c));
    Ok(flag(r.shape.len() <= 1 && one))
}

/// `l ⎕CC r`: operation `l` done with each of `r` in turn -- 1 the
/// screen off and on, 2 the alarm, 3 the keyboard's case, 4 scroll
/// by lines, 5 the printer's tab. 1 when every value is one the
/// operation takes; 0 otherwise, or for an operation there is not.
///
/// # Errors
/// DOMAIN ERROR unless both are whole numbers, RANK ERROR for more
/// than a vector.
pub fn control(l: &Array, r: &Array) -> AplResult<Array> {
    let (op, values) = (whole(l)?, whole(r)?);
    let takes: RangeInclusive<i64> = match op.as_slice() {
        [1 | 3] => 0..=1,
        [2] => 0..=2,
        [4] => -16..=16,
        [5] => 1..=131,
        _ => return Ok(flag(false)),
    };
    Ok(flag(values.iter().all(|v| takes.contains(v))))
}

/// The whole numbers a scalar or vector holds.
fn whole(a: &Array) -> AplResult<Vec<i64>> {
    if a.shape.len() > 1 {
        return Err(AplError::new(ErrorKind::Rank));
    }
    let Data::Num(numbers) = &a.data else {
        return Err(AplError::new(ErrorKind::Domain));
    };
    numbers
        .iter()
        .map(|n| match Number::from_f64(n.as_f64()) {
            Number::Int(i) => Ok(i),
            Number::Float(_) => Err(AplError::new(ErrorKind::Domain)),
        })
        .collect()
}

/// 1 or 0.
fn flag(done: bool) -> Array {
    Array::scalar(Number::Int(i64::from(done)))
}
