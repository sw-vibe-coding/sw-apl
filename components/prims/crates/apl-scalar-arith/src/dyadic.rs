//! Dyadic arithmetic.

use apl_value::{AplError, AplResult, ErrorKind};

use crate::expo::{log, power};
use crate::monadic::tolerant_round;

/// `a f b` for the arithmetic glyphs; `None` when `f` is not one.
/// Residue is tolerant within `ct`.
#[must_use]
pub fn dyadic_arith(f: char, a: f64, b: f64, ct: f64) -> Option<AplResult<f64>> {
    Some(Ok(match f {
        '+' => a + b,
        '-' => a - b,
        '×' => a * b,
        '÷' => return Some(divide(a, b)),
        '⌈' => a.max(b),
        '⌊' => a.min(b),
        '|' => residue(a, b, ct),
        '*' => return Some(power(a, b)),
        '⍟' => return Some(log(a, b)),
        _ => return None,
    }))
}

fn divide(a: f64, b: f64) -> AplResult<f64> {
    match (a == 0.0, b == 0.0) {
        (true, true) => Ok(1.0),
        (false, true) => Err(AplError::new(ErrorKind::Domain)),
        _ => Ok(a / b),
    }
}

/// APL residue: `a|b` is `b - a × ⌊ b ÷ a`, `0|b` is `b`, and a
/// quotient within the tolerance of an integer leaves no remainder.
fn residue(a: f64, b: f64, ct: f64) -> f64 {
    if a == 0.0 {
        return b;
    }
    let q = b / a;
    match tolerant_round(q, ct) {
        Some(_) => 0.0,
        None => b - a * q.floor(),
    }
}
