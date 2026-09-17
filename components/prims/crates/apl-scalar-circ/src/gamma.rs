//! Gamma, factorial, binomial.

use apl_value::{AplError, AplResult, ErrorKind, Number};

/// Lanczos coefficients (g = 7, n = 9).
const LANCZOS: [f64; 9] = [
    0.999_999_999_999_809_9,
    676.520_368_121_885_1,
    -1_259.139_216_722_402_8,
    771.323_428_777_653_1,
    -176.615_029_162_140_6,
    12.507_343_278_686_905,
    -0.138_571_095_265_720_12,
    9.984_369_578_019_572e-6,
    1.505_632_735_149_311_6e-7,
];

/// The gamma function by the Lanczos approximation with reflection.
#[must_use]
pub fn gamma(x: f64) -> f64 {
    use std::f64::consts::PI;
    if x < 0.5 {
        return PI / ((PI * x).sin() * gamma(1.0 - x));
    }
    let x = x - 1.0;
    let t = x + 7.5;
    let a = LANCZOS[1..]
        .iter()
        .zip((1u32..).map(f64::from))
        .fold(LANCZOS[0], |acc, (c, i)| acc + c / (x + i));
    (2.0 * PI).sqrt() * t.powf(x + 0.5) * (-t).exp() * a
}

/// `!x`: exact for integers up to 20, gamma(x + 1) otherwise.
///
/// # Errors
/// DOMAIN ERROR at negative integers.
pub fn factorial(x: f64) -> AplResult<f64> {
    if let Number::Int(n) = Number::from_f64(x) {
        if n < 0 {
            return Err(AplError::new(ErrorKind::Domain));
        }
        if let Ok(n @ 0..=20) = u32::try_from(n) {
            return Ok((1..=n).map(f64::from).product());
        }
    }
    Ok(gamma(x + 1.0))
}

/// `a!b`: combinations of `b` things taken `a` at a time, extended
/// by gamma for non-integers; zero when `a` exceeds `b` for
/// non-negative integers.
///
/// # Errors
/// DOMAIN ERROR where the gamma extension is undefined.
pub fn binomial(a: f64, b: f64) -> AplResult<f64> {
    let integral = a.fract() == 0.0 && b.fract() == 0.0;
    if integral && a >= 0.0 && b >= 0.0 && a > b {
        return Ok(0.0);
    }
    let r = factorial(b)? / (factorial(a)? * factorial(b - a)?);
    if r.is_finite() {
        Ok(r)
    } else {
        Err(AplError::new(ErrorKind::Domain))
    }
}
