//! Monadic scalar functions on one number.

use apl_value::{AplError, AplResult, ErrorKind, Number};

/// `f r` for the scalar monadic glyphs.
pub fn apply_monadic(f: char, r: Number) -> AplResult<Number> {
    let a = r.as_f64();
    let x = match f {
        '+' => a,
        '-' => -a,
        '×' => signum(a),
        '÷' => reciprocal(a)?,
        '⌈' => a.ceil(),
        '⌊' => a.floor(),
        '|' => a.abs(),
        '*' => a.exp(),
        _ => return Err(AplError::new(ErrorKind::NotImplemented)),
    };
    finite(x)
}

fn signum(a: f64) -> f64 {
    if a == 0.0 { 0.0 } else { a.signum() }
}

fn reciprocal(a: f64) -> AplResult<f64> {
    if a == 0.0 {
        Err(AplError::new(ErrorKind::Domain))
    } else {
        Ok(1.0 / a)
    }
}

/// Overflow to infinity or NaN is a DOMAIN ERROR.
pub fn finite(x: f64) -> AplResult<Number> {
    if x.is_finite() {
        Ok(Number::from_f64(x))
    } else {
        Err(AplError::new(ErrorKind::Domain))
    }
}
