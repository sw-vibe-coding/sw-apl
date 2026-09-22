//! `f\r` along an axis: each element is the reduction of the prefix
//! ending there.

use apl_prims_scalar::{DYADIC, Element, numbers};
use apl_prims_select::strides;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

use crate::reduce::{base_index, fold};

/// Scan along axis `k`; a scalar scans to itself.
///
/// # Errors
/// SYNTAX ERROR for a function with no scalar dyadic form;
/// DOMAIN ERROR from the function.
pub fn scan(f: char, r: &Array, k: usize) -> AplResult<Array> {
    if !DYADIC.contains(f) {
        return Err(AplError::new(ErrorKind::Syntax));
    }
    let data = numbers(r)?;
    if r.shape.is_empty() {
        return Ok(r.clone());
    }
    let (st, n) = (strides(&r.shape), r.shape[k]);
    let mut rest = r.shape.clone();
    rest.remove(k);
    let mut out = vec![Number::Int(0); data.len()];
    for o in 0..rest.iter().product::<usize>() {
        let base = base_index(o, &rest, &st, k);
        for i in 0..n {
            let prefix = (0..=i).map(|j| Element::Num(data[base + j * st[k]]));
            out[base + i * st[k]] = fold(f, prefix)?;
        }
    }
    Array::new(r.shape.clone(), Data::Num(out))
}
