//! A scaled field: a mantissa, the decoration after the 7's digits in
//! place of the `E`, and an exponent.

use apl_d83_field::{Overflow, joins};

use crate::field::plain;

/// `x` in the field `f`, scaled: the mantissa in the digits up to and
/// around the 7 at `seven`, its whole part as many digits as the
/// field gives it, then the decoration that stands for `E`, then the
/// exponent in the digits after it.
pub fn scaled(f: &[char], seven: usize, x: f64, fc: &[char]) -> Result<Vec<char>, Overflow> {
    let end = (seven..f.len())
        .find(|&i| !f[i].is_ascii_digit() && !joins(f, i))
        .ok_or(Overflow)?;
    let mut e = power(&f[..end], x.abs());
    let mut out = loop {
        match plain(&f[..end], x / 10f64.powi(e), fc) {
            Err(Overflow) if x != 0.0 && e < 400 => e += 1,
            done => break done?,
        }
    };
    out.push(f[end]);
    out.extend(exponent(&f[end + 1..], e, fc)?);
    Ok(out)
}

/// The power of ten that leaves `a` with as many whole digits as the
/// mantissa `m` has places for, from the power of `a`'s first digit
/// as `{:e}` writes it: exact, where a logarithm can land a hair under
/// a power of ten.
fn power(m: &[char], a: f64) -> i32 {
    let ints = m
        .iter()
        .take_while(|&&c| c != '.')
        .filter(|c| c.is_ascii_digit());
    let ints = i32::try_from(ints.count()).unwrap_or(1);
    let first = format!("{a:e}")
        .split_once('e')
        .and_then(|(_, p)| p.parse::<i32>().ok());
    if a == 0.0 {
        0
    } else {
        first.map_or(0, |p| p - (ints - 1))
    }
}

/// The exponent, its minus sign the high minus in the first blank
/// before its digits.
fn exponent(f: &[char], e: i32, fc: &[char]) -> Result<Vec<char>, Overflow> {
    let mut out = plain(f, f64::from(e.abs()), fc)?;
    if e < 0 {
        let first = out
            .iter()
            .position(char::is_ascii_digit)
            .unwrap_or(out.len());
        let at = (0..first).rev().find(|&i| out[i] == ' ').ok_or(Overflow)?;
        out[at] = '¯';
    }
    Ok(out)
}
