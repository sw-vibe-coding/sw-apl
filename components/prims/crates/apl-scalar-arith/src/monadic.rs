//! Monadic arithmetic.

use apl_value::{AplError, AplResult, ErrorKind, FUZZ};

/// `f a` for the arithmetic glyphs; `None` when `f` is not one.
#[must_use]
pub fn monadic_arith(f: char, a: f64) -> Option<AplResult<f64>> {
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
        '⌈' => tolerant_round(a).unwrap_or_else(|| a.ceil()),
        '⌊' => tolerant_round(a).unwrap_or_else(|| a.floor()),
        '|' => a.abs(),
        '*' => a.exp(),
        '⍟' => return Some(ln(a)),
        _ => return None,
    }))
}

/// The nearest integer when `a` is within the fuzz of it.
pub fn tolerant_round(a: f64) -> Option<f64> {
    let r = a.round();
    ((a - r).abs() <= FUZZ * a.abs().max(1.0)).then_some(r)
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
