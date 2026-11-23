//! One number in one numerical field.

use apl_d83_field::{Layout, Overflow, decorate, digits, fixed, joins, padded, reach};

use crate::scaled::scaled;

/// `x` laid out in the field `f`, under the format control `fc`. The
/// number shown is `x`'s magnitude: its sign is what the field's
/// decorations say. A field with a 7 is scaled.
pub fn render(f: &[char], x: f64, fc: &[char]) -> Result<Vec<char>, Overflow> {
    match f.iter().position(|&c| c == '7') {
        Some(seven) => scaled(f, seven, x, fc),
        None => plain(f, x, fc),
    }
}

/// `x` in a field without scaling: its digits and point, the commas
/// between digits shown, the decorations, and the fill of an 8.
pub fn plain(f: &[char], x: f64, fc: &[char]) -> Result<Vec<char>, Overflow> {
    let at = Layout::of(f, fc[4]);
    let mut out: Vec<char> = (0..f.len()).map(|i| fixed(f, &at, i, fc)).collect();
    place(f, &at, &mut out, x, fc)?;
    for i in (0..f.len()).filter(|&i| f[i] == ',' && joins(f, i)) {
        if out[i - 1].is_ascii_digit() && out[i + 1].is_ascii_digit() {
            out[i] = fc[1];
        }
    }
    decorate(f, &at, &mut out, x)?;
    if f.contains(&'8') {
        out.iter_mut()
            .filter(|c| **c == ' ')
            .for_each(|c| *c = fc[2]);
    }
    Ok(out)
}

/// The digits of `x` in the field's digit places, and the point when
/// a fraction shows.
fn place(f: &[char], at: &Layout, out: &mut [char], x: f64, fc: &[char]) -> Result<(), Overflow> {
    let point = at.point.unwrap_or(f.len());
    let (ints, fracs): (Vec<usize>, Vec<usize>) = at.digits.iter().partition(|&&i| i < point);
    let (int, frac) = digits(x.abs(), fracs.len());
    let int = padded(f, &ints, &int, x != 0.0)?;
    let shown = frac
        .trim_end_matches('0')
        .len()
        .max(reach(f, &fracs, x != 0.0));
    for (k, d) in int.chars().enumerate() {
        out[ints[ints.len() - int.len() + k]] = d;
    }
    for (k, d) in frac.chars().take(shown).enumerate() {
        out[fracs[k]] = d;
    }
    if shown > 0 {
        out[point] = fc[0];
    }
    Ok(())
}
