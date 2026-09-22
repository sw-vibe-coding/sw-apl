//! `f/r` along an axis: fold each line of elements from the right.

use apl_prims_scalar::{Element, elements, related};
use apl_prims_select::strides;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

/// Reduce along axis `k`; a scalar reduces to itself.
///
/// # Errors
/// SYNTAX ERROR for a function with no scalar dyadic form;
/// DOMAIN ERROR from the function, for characters unless it is = or
/// ≠, and for an empty reduction of a function with no identity
/// element.
pub fn reduce(f: char, r: &Array, k: usize) -> AplResult<Array> {
    let data = elements(r, f)?;
    if r.shape.is_empty() {
        return Ok(r.clone());
    }
    let (st, n) = (strides(&r.shape), r.shape[k]);
    let mut shape = r.shape.clone();
    shape.remove(k);
    // One element along the axis is that element: the manual says a
    // reduction of one element yields it, character or number alike.
    if n == 1 {
        return Array::new(shape, r.data.clone());
    }
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

/// Fold from the right; the identity element for no elements. What
/// comes out of a relation is a number, so only a lone element could
/// be a character, and callers hand that on without folding.
///
/// # Errors
/// DOMAIN ERROR from the function, and for a lone character.
pub fn fold(f: char, items: impl DoubleEndedIterator<Item = Element>) -> AplResult<Number> {
    let mut rest = items.rev();
    let Some(start) = rest.next() else {
        return identity(f);
    };
    let last = rest.try_fold(start, |acc, x| related(f, x, acc).map(Element::Num))?;
    match last {
        Element::Num(n) => Ok(n),
        Element::Char(_) => Err(AplError::new(ErrorKind::Domain)),
    }
}

/// The value of reducing an empty vector (APL\360 identity table).
fn identity(f: char) -> AplResult<Number> {
    Ok(match f {
        '+' | '-' | '|' | '∨' | '<' | '>' | '≠' => Number::Int(0),
        '×' | '÷' | '*' | '!' | '∧' | '≤' | '=' | '≥' => Number::Int(1),
        '⌈' => Number::Float(f64::MIN),
        '⌊' => Number::Float(f64::MAX),
        '⍟' | '○' | '⍲' | '⍱' => return Err(AplError::new(ErrorKind::Domain)),
        _ => return Err(AplError::new(ErrorKind::Syntax)),
    })
}
