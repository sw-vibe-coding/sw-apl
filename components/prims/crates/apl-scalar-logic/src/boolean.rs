//! And, or, nand, nor, not: arguments must be 0 or 1.

use apl_value::{AplError, AplResult, ErrorKind};

/// `a f b` for `∧ ∨ ⍲ ⍱`; `None` when `f` is not one of them.
#[must_use]
pub fn boolean(f: char, a: f64, b: f64) -> Option<AplResult<bool>> {
    let op: fn(bool, bool) -> bool = match f {
        '∧' => |p, q| p && q,
        '∨' => |p, q| p || q,
        '⍲' => |p, q| !(p && q),
        '⍱' => |p, q| !(p || q),
        _ => return None,
    };
    Some(as_bool(a).and_then(|p| as_bool(b).map(|q| op(p, q))))
}

/// `~a`.
///
/// # Errors
/// DOMAIN ERROR unless `a` is 0 or 1.
pub fn not(a: f64) -> AplResult<bool> {
    as_bool(a).map(|p| !p)
}

fn as_bool(x: f64) -> AplResult<bool> {
    match x {
        0.0 => Ok(false),
        1.0 => Ok(true),
        _ => Err(AplError::new(ErrorKind::Domain)),
    }
}
