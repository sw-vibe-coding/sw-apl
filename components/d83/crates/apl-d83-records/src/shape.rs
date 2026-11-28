//! A record's contents shaped to its length: characters padded with
//! blanks, bits with zeros.

use apl_value::{Array, Data, Number};

/// `value`'s characters as a record of `n`, ASCII only when `ascii`.
/// None for numbers, for more than `n`, or for a character ASCII has
/// not got.
pub(crate) fn chars(value: &Array, n: usize, ascii: bool) -> Option<Array> {
    let Data::Char(c) = &value.data else {
        return None;
    };
    if c.len() > n || (ascii && !c.iter().all(char::is_ascii)) {
        return None;
    }
    Some(fit(value, n))
}

/// `value`'s bits as a record of `n` bits. None for anything but
/// zeros and ones, or for more than `n`.
pub(crate) fn bits(value: &Array, n: usize) -> Option<Array> {
    let Data::Num(v) = &value.data else {
        return None;
    };
    let bit = |x: &Number| match *x {
        Number::Int(b @ (0 | 1)) => Some(Number::Int(b)),
        _ => None,
    };
    let ints: Vec<Number> = v.iter().map(bit).collect::<Option<_>>()?;
    if ints.len() > n {
        return None;
    }
    Some(fit(&Array::vector(ints), n))
}

/// `value` raveled to `n` elements: cut, or filled with blanks or
/// zeros.
pub(crate) fn fit(value: &Array, n: usize) -> Array {
    let data = match &value.data {
        Data::Char(c) => Data::Char(
            c.iter()
                .copied()
                .chain(std::iter::repeat(' '))
                .take(n)
                .collect(),
        ),
        Data::Num(v) => {
            let zero = std::iter::repeat(Number::Int(0));
            Data::Num(v.iter().copied().chain(zero).take(n).collect())
        }
    };
    Array {
        shape: vec![n],
        data,
    }
}
