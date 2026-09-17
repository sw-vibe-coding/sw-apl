//! `k○x`: the circular, hyperbolic, and pythagorean functions.

use apl_value::{AplError, AplResult, ErrorKind, Number};

/// `k○x` for integer `k` from ¯7 to 7.
///
/// # Errors
/// DOMAIN ERROR for any other `k`, or when the function is undefined
/// at `x`.
pub fn circular(k: f64, x: f64) -> AplResult<f64> {
    let domain = || AplError::new(ErrorKind::Domain);
    let Number::Int(k) = Number::from_f64(k) else {
        return Err(domain());
    };
    let r = match k {
        0 => (1.0 - x * x).sqrt(),
        1 => x.sin(),
        2 => x.cos(),
        3 => x.tan(),
        4 => (1.0 + x * x).sqrt(),
        5 => x.sinh(),
        6 => x.cosh(),
        7 => x.tanh(),
        -1 => x.asin(),
        -2 => x.acos(),
        -3 => x.atan(),
        -4 => (x * x - 1.0).sqrt(),
        -5 => x.asinh(),
        -6 => x.acosh(),
        -7 => x.atanh(),
        _ => return Err(domain()),
    };
    if r.is_finite() { Ok(r) } else { Err(domain()) }
}
