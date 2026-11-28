//! How many bytes an APL object takes, as IBM PC APL keeps it (pages
//! 4-9 to 4-11): an eight-byte header, two bytes for each dimension,
//! then the elements -- a bit each for a boolean, two bytes for an
//! integer that fits a word, eight for a real, one for a character --
//! the whole rounded up to an even number.

use apl_value::{Array, Data, Number};

/// What `value` takes with its header: what a code A record must hold.
#[must_use]
pub fn bytes(value: &Array) -> usize {
    let n = 8 + 2 * value.shape.len() + elements(&value.data);
    n + n % 2
}

/// What the elements take, in the narrowest type that holds them.
fn elements(data: &Data) -> usize {
    match data {
        Data::Char(c) => c.len(),
        Data::Num(v) => {
            let as_int = |x: &Number| matches!(x, Number::Int(i) if i.abs() <= 32_767);
            if v.iter().all(|x| matches!(x, Number::Int(0 | 1))) {
                v.len().div_ceil(8)
            } else if v.iter().all(as_int) {
                2 * v.len()
            } else {
                8 * v.len()
            }
        }
    }
}
