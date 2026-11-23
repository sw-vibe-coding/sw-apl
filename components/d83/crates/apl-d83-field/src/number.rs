//! A number's digits, and how many of them a field shows.

/// A number too wide for its field.
#[derive(Debug)]
pub struct Overflow;

/// The digits of `a`, rounded to `places` after the point: the whole
/// part without leading zeros (none at all for less than one), and
/// the fraction, `places` long.
#[must_use]
pub fn digits(a: f64, places: usize) -> (String, String) {
    let scaled = (a * 10f64.powi(i32::try_from(places).unwrap_or(i32::MAX))).round();
    let all = format!("{scaled:.0}");
    let all = format!("{all:0>width$}", width = places + 1);
    let (int, frac) = all.split_at(all.len() - places);
    (int.trim_start_matches('0').to_string(), frac.to_string())
}

/// Whether the digit at `i` pads zeros out to itself: a 0 always, a
/// 9 when the number is not zero.
fn pads(f: &[char], i: usize, nonzero: bool) -> bool {
    f[i] == '0' || (f[i] == '9' && nonzero)
}

/// The whole part as the field shows it: zeros padded out from the
/// point to the furthest 0 (or 9) among the whole-number positions.
///
/// # Errors
/// Overflow when the digits will not fit the whole-number positions.
pub fn padded(f: &[char], ints: &[usize], int: &str, nonzero: bool) -> Result<String, Overflow> {
    let reach = ints
        .iter()
        .position(|&i| pads(f, i, nonzero))
        .map_or(0, |j| ints.len() - j);
    let need = int.len().max(reach);
    if need > ints.len() {
        return Err(Overflow);
    }
    Ok(format!("{int:0>need$}"))
}

/// How many fraction digits are shown at least: out from the point to
/// the furthest 0 (or 9) among the fraction positions.
#[must_use]
pub fn reach(f: &[char], fracs: &[usize], nonzero: bool) -> usize {
    fracs
        .iter()
        .rposition(|&i| pads(f, i, nonzero))
        .map_or(0, |j| j + 1)
}
