//! One number in decimal or scaled form, as dyadic format lays it out.

use apl_value::Number;

use crate::digits::{digits, rounded};

/// `|x|` rounded to `places` decimal places: [`rounded`] to as many
/// digits as its whole part has, plus `places`.
#[must_use]
fn to_places(x: f64, places: i32) -> (Vec<u8>, i32) {
    let (_, e) = digits(x);
    rounded(x, e.saturating_add(places))
}

/// One number as its field holds it, before padding: decimal form for
/// a precision of 0 or more, scaled form for a negative one. The sign
/// is kept even when no digit is left to carry it.
#[must_use]
pub fn layout(n: Number, precision: i64) -> String {
    let x = n.as_f64();
    let sign = if x < 0.0 { "¯" } else { "" };
    let body = if precision >= 0 {
        decimal(x, precision)
    } else {
        scaled(x, precision.unsigned_abs())
    };
    format!("{sign}{body}")
}

/// `|x|` to `places` decimal places. A whole part of 0 is left out
/// when there are places, as the manual shows: `.26`, `.00`.
fn decimal(x: f64, places: i64) -> String {
    let p = usize::try_from(places).unwrap_or(0);
    let (ds, e) = to_places(x, i32::try_from(places).unwrap_or(i32::MAX));
    let ints = usize::try_from(e.max(0)).unwrap_or(0);
    let digit = |d: &u8| char::from(b'0' + d);
    let whole: String = ds.iter().take(ints).map(digit).collect();
    let lead = usize::try_from((-e).max(0)).unwrap_or(0);
    let frac: String = std::iter::repeat_n('0', lead)
        .chain(ds.iter().skip(ints).map(digit))
        .chain(std::iter::repeat('0'))
        .take(p)
        .collect();
    match (whole.trim_start_matches('0'), p) {
        ("", 0) => "0".to_string(),
        (w, 0) => w.to_string(),
        (w, _) => format!("{w}.{frac}"),
    }
}

/// `|x|` in scaled form to `count` digits: one before the point, the
/// rest after it, and a power of ten of at least two digits. Zero is
/// `0.0E¯01`, as the manual shows it.
fn scaled(x: f64, count: u64) -> String {
    let d = usize::try_from(count).unwrap_or(1);
    let (ds, e) = rounded(x, i32::try_from(d).unwrap_or(i32::MAX));
    let zero = ds.iter().all(|&v| v == 0);
    let power = if zero { -1 } else { e - 1 };
    let text: String = ds.iter().take(d).map(|v| char::from(b'0' + v)).collect();
    let (first, rest) = text.split_at(1.min(text.len()));
    let mantissa = if rest.is_empty() {
        first.to_string()
    } else {
        format!("{first}.{rest}")
    };
    let sign = if power < 0 { "¯" } else { "" };
    format!("{mantissa}E{sign}{:02}", power.unsigned_abs())
}
