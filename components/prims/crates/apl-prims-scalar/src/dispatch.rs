//! Glyph to family: arithmetic, circular and gamma, logic.

use apl_scalar_arith::{dyadic_arith, monadic_arith};
use apl_scalar_circ::{binomial, circular, factorial};
use apl_scalar_logic::{boolean, compare, not};
use apl_value::{AplError, AplResult, ErrorKind, Number};

/// Glyphs with a monadic scalar meaning. The mixed functions are
/// dispatched above this crate, so the scalar family is what
/// remains of the generated monadic set.
pub const MONADIC: &str = "+-×÷⌈⌊|*⍟○!~";
/// Glyphs with a dyadic scalar meaning.
pub const DYADIC: &str = "+-×÷⌈⌊|*⍟○!∧∨⍲⍱<≤=≥>≠";

/// `f r` for one number.
///
/// # Errors
/// DOMAIN ERROR from the function; SYNTAX ERROR for a glyph with no such form.
pub fn apply_monadic(f: char, r: Number) -> AplResult<Number> {
    attended()?;
    let a = r.as_f64();
    if let Some(x) = monadic_arith(f, a) {
        return finite(x?);
    }
    match f {
        '○' => finite(std::f64::consts::PI * a),
        '!' => finite(factorial(a)?),
        '~' => Ok(Number::Int(not(a)?.into())),
        _ => Err(AplError::new(ErrorKind::Syntax)),
    }
}

/// `l f r` for two numbers: exact integers first, then the families.
///
/// # Errors
/// DOMAIN ERROR from the function; SYNTAX ERROR for a glyph with no such form.
pub fn apply_dyadic(f: char, left: Number, right: Number) -> AplResult<Number> {
    attended()?;
    if let Some(exact) = Number::exact_int(f, left, right) {
        return Ok(exact);
    }
    if let Some(truth) = compare(f, left, right) {
        return Ok(Number::Int(truth.into()));
    }
    let (lhs, rhs) = (left.as_f64(), right.as_f64());
    if let Some(value) = dyadic_arith(f, lhs, rhs) {
        return finite(value?);
    }
    if let Some(truth) = boolean(f, lhs, rhs) {
        return Ok(Number::Int(truth?.into()));
    }
    match f {
        '○' => finite(circular(lhs, rhs)?),
        '!' => finite(binomial(lhs, rhs)?),
        _ => Err(AplError::new(ErrorKind::Syntax)),
    }
}

/// Overflow to infinity or NaN is a DOMAIN ERROR.
fn finite(x: f64) -> AplResult<Number> {
    if x.is_finite() {
        Ok(Number::from_f64(x))
    } else {
        Err(AplError::new(ErrorKind::Domain))
    }
}

/// INTERRUPT if this session has been asked to stop.
///
/// Every primitive's inner loop comes through the two scalar
/// dispatchers -- reduce, scan, the products, and plain `A+B` over an
/// array alike -- so this is the one place a long statement notices
/// ATTN, and the whole cost of the owner's rule that a tight loop be
/// loose. `polled` really reads the flag once in `apl_attn::STRIDE`
/// calls; measured on four million products, about seven percent.
fn attended() -> AplResult<()> {
    if apl_attn::polled() {
        return Err(AplError::new(ErrorKind::Interrupt));
    }
    Ok(())
}
