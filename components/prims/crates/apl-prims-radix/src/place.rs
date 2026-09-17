//! Place values and digit extraction for a radix vector.

use apl_value::Number;

/// Place values for a radix vector: the last position weighs 1.
#[must_use]
pub fn weights(radix: &[f64]) -> Vec<f64> {
    let mut w = vec![1.0; radix.len()];
    for i in (1..radix.len()).rev() {
        w[i - 1] = w[i] * radix[i];
    }
    w
}

/// The digits of `value`, computed from the last radix position
/// back with the APL residue (a zero radix keeps whatever remains).
#[must_use]
pub fn digits_of(radix: &[Number], value: f64) -> Vec<f64> {
    let mut digits = vec![0.0; radix.len()];
    let mut rest = value;
    for i in (0..radix.len()).rev() {
        let base = radix[i].as_f64();
        if base == 0.0 {
            digits[i] = rest;
            rest = 0.0;
        } else {
            digits[i] = rest - base * (rest / base).floor();
            rest = (rest - digits[i]) / base;
        }
    }
    digits
}
