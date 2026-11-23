//! A field's decorations: those fixed in place, and those that float
//! against the number.

use std::ops::Range;

use crate::layout::{Layout, joins};
use crate::number::Overflow;

/// What position `i` of the field shows before the number goes in:
/// its own character if it is a fixed decoration, otherwise a blank.
#[must_use]
pub fn fixed(f: &[char], at: &Layout, i: usize, fc: &[char]) -> char {
    let c = f[i];
    let blank = c.is_ascii_digit()
        || Some(i) == at.point
        || (c == ',' && joins(f, i))
        || c == fc[4]
        || at.left.contains(&i)
        || at.right.contains(&i);
    if blank { ' ' } else { c }
}

/// Put the decorations against the first and last digits in: fixed
/// where the field has no 1, 2 or 3, or a 4 is the control nearest
/// that side; otherwise floated against the number shown, when its
/// sign is the one the 1 (negative), 2 (not negative) or 3 (any) asks.
///
/// # Errors
/// Overflow when a floating decoration has no room in the field.
pub fn decorate(f: &[char], at: &Layout, out: &mut [char], x: f64) -> Result<(), Overflow> {
    let kind = f.iter().copied().find(|c| matches!(c, '1' | '2' | '3'));
    let controls: Vec<char> = at
        .digits
        .iter()
        .map(|&i| f[i])
        .filter(|&c| c != '5')
        .collect();
    let floats = |c: Option<&char>| kind.is_some() && c != Some(&'4');
    let show = match kind {
        Some('1') => x < 0.0,
        Some('2') => x >= 0.0,
        _ => true,
    };
    let (lo, hi) = shown(at, out);
    let left = (floats(controls.first()), lo.checked_sub(at.left.len()));
    side(f, out, at.left.clone(), (left.0, show), left.1)?;
    side(
        f,
        out,
        at.right.clone(),
        (floats(controls.last()), show),
        Some(hi),
    )
}

/// Where the number shown starts and where it ends, its digits and
/// point; where it would be, at the point, when nothing shows.
fn shown(at: &Layout, out: &[char]) -> (usize, usize) {
    let number: Vec<usize> = (0..out.len())
        .filter(|&i| (at.digits.contains(&i) || Some(i) == at.point) && out[i] != ' ')
        .collect();
    let anchor = at.point.unwrap_or(at.digits[at.digits.len() - 1] + 1);
    let lo = number.first().copied().unwrap_or(anchor);
    (lo, number.last().map_or(anchor, |&h| h + 1))
}

/// One side's run of decoration: fixed, or floated to `to` when shown.
fn side(
    f: &[char],
    out: &mut [char],
    run: Range<usize>,
    (floats, show): (bool, bool),
    to: Option<usize>,
) -> Result<(), Overflow> {
    if !floats {
        run.for_each(|i| out[i] = f[i]);
        return Ok(());
    }
    if !show || run.is_empty() {
        return Ok(());
    }
    let to = to.filter(|&t| t + run.len() <= out.len()).ok_or(Overflow)?;
    for (k, i) in run.enumerate() {
        out[to + k] = f[i];
    }
    Ok(())
}
