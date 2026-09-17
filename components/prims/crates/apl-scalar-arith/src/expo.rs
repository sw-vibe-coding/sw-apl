//! Power and logarithm.

use apl_value::{AplError, AplResult, ErrorKind};

use crate::monadic::ln;

/// `a*b`.
///
/// # Errors
/// DOMAIN ERROR for a negative base with a fractional exponent.
pub fn power(a: f64, b: f64) -> AplResult<f64> {
    if a < 0.0 && b.fract() != 0.0 {
        return Err(AplError::new(ErrorKind::Domain));
    }
    Ok(a.powf(b))
}

/// `a⍟b`: the base-`a` logarithm of `b`.
///
/// # Errors
/// DOMAIN ERROR when either argument is not positive, or the base is
/// 1 (except `1⍟1`, which is 1).
pub fn log(a: f64, b: f64) -> AplResult<f64> {
    if (a - 1.0) == 0.0 {
        return if (b - 1.0) == 0.0 {
            Ok(1.0)
        } else {
            Err(AplError::new(ErrorKind::Domain))
        };
    }
    Ok(ln(b)? / ln(a)?)
}
