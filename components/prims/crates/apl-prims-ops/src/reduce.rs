//! `f/r` along an axis: fold each line of elements from the right.

use apl_prims_scalar::{apply_dyadic, numbers};
use apl_prims_select::strides;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

/// Reduce along axis `k`; a scalar reduces to itself.
///
/// # Errors
/// NOT IMPLEMENTED for functions without a scalar dyadic form;
/// DOMAIN ERROR from the function or for an empty reduction of a
/// function with no identity element.
pub fn reduce(f: char, r: &Array, k: usize) -> AplResult<Array> {
    let data = numbers(r)?;
    if r.shape.is_empty() {
        return Ok(r.clone());
    }
    let (st, n) = (strides(&r.shape), r.shape[k]);
    let mut shape = r.shape.clone();
    shape.remove(k);
    let out = (0..shape.iter().product::<usize>())
        .map(|o| {
            let base = base_index(o, &shape, &st, k);
            fold(f, (0..n).map(|i| data[base + i * st[k]]))
        })
        .collect::<AplResult<Vec<_>>>()?;
    Array::new(shape, Data::Num(out))
}

/// The flat index in the source of position `o` (row-major over the
/// shape without axis `k`) with the axis-`k` coordinate zero.
pub fn base_index(o: usize, rest: &[usize], st: &[usize], k: usize) -> usize {
    let mut remaining = o;
    let mut flat = 0;
    for (axis, &len) in rest.iter().enumerate().rev() {
        let coordinate = remaining % len;
        remaining /= len;
        flat += coordinate * st[if axis < k { axis } else { axis + 1 }];
    }
    flat
}

/// Fold from the right; the identity element for no elements.
pub fn fold(f: char, items: impl DoubleEndedIterator<Item = Number>) -> AplResult<Number> {
    let mut rest = items.rev();
    let Some(start) = rest.next() else {
        return identity(f);
    };
    rest.try_fold(start, |acc, x| apply_dyadic(f, x, acc))
}

/// The value of reducing an empty vector (APL\360 identity table).
fn identity(f: char) -> AplResult<Number> {
    Ok(match f {
        '+' | '-' | '|' | '∨' | '<' | '>' | '≠' => Number::Int(0),
        '×' | '÷' | '*' | '!' | '∧' | '≤' | '=' | '≥' => Number::Int(1),
        '⌈' => Number::Float(f64::MIN),
        '⌊' => Number::Float(f64::MAX),
        '⍟' | '○' | '⍲' | '⍱' => return Err(AplError::new(ErrorKind::Domain)),
        _ => return Err(AplError::new(ErrorKind::NotImplemented)),
    })
}
