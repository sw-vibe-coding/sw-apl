//! Dyadic scalar functions on two numbers.

use apl_value::{AplError, AplResult, ErrorKind, Number};

use crate::monadic::finite;

/// `l f r` for the scalar dyadic glyphs.
///
/// # Errors
/// DOMAIN ERROR for division by zero, a fractional power of a
/// negative, or overflow; NOT IMPLEMENTED for other glyphs.
pub fn apply_dyadic(glyph: char, left: Number, right: Number) -> AplResult<Number> {
    let (a, b) = (left.as_f64(), right.as_f64());
    let result = match glyph {
        '+' => a + b,
        '-' => a - b,
        '×' => a * b,
        '÷' => divide(a, b)?,
        '⌈' => a.max(b),
        '⌊' => a.min(b),
        '|' => residue(a, b),
        '*' => power(a, b)?,
        _ => return Err(AplError::new(ErrorKind::NotImplemented)),
    };
    finite(result)
}

fn divide(a: f64, b: f64) -> AplResult<f64> {
    match (a == 0.0, b == 0.0) {
        (true, true) => Ok(1.0),
        (false, true) => Err(AplError::new(ErrorKind::Domain)),
        _ => Ok(a / b),
    }
}

/// APL residue: `a|b` is `b - a × ⌊ b ÷ a`, and `0|b` is `b`.
fn residue(a: f64, b: f64) -> f64 {
    if a == 0.0 { b } else { b - a * (b / a).floor() }
}

fn power(a: f64, b: f64) -> AplResult<f64> {
    if a < 0.0 && b.fract() != 0.0 {
        return Err(AplError::new(ErrorKind::Domain));
    }
    Ok(a.powf(b))
}
