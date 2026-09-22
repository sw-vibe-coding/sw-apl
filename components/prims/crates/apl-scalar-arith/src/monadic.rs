//! Monadic arithmetic.

use apl_value::{AplError, AplResult, ErrorKind};

/// `f a` for the arithmetic glyphs; `None` when `f` is not one.
/// Floor and ceiling are tolerant within `ct`.
#[must_use]
pub fn monadic_arith(f: char, a: f64, ct: f64) -> Option<AplResult<f64>> {
    Some(Ok(match f {
        '+' => a,
        '-' => -a,
        '×' => {
            if a == 0.0 {
                0.0
            } else {
                a.signum()
            }
        }
        '÷' => return Some(reciprocal(a)),
        '⌈' => tolerant_round(a, ct).unwrap_or_else(|| a.ceil()),
        '⌊' => tolerant_round(a, ct).unwrap_or_else(|| a.floor()),
        '|' => a.abs(),
        '*' => a.exp(),
        '⍟' => return Some(ln(a)),
        _ => return None,
    }))
}

/// The nearest integer when `a` is within the tolerance `ct` of it,
/// relative to its magnitude and never less than `ct` itself.
#[must_use]
pub fn tolerant_round(a: f64, ct: f64) -> Option<f64> {
    let r = a.round();
    ((a - r).abs() <= ct * a.abs().max(1.0)).then_some(r)
}

fn reciprocal(a: f64) -> AplResult<f64> {
    if a == 0.0 {
        Err(AplError::new(ErrorKind::Domain))
    } else {
        Ok(1.0 / a)
    }
}

/// Natural logarithm; DOMAIN ERROR at or below zero.
pub fn ln(a: f64) -> AplResult<f64> {
    if a <= 0.0 {
        Err(AplError::new(ErrorKind::Domain))
    } else {
        Ok(a.ln())
    }
}
