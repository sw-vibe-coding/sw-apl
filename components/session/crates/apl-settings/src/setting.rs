//! Setting one setting, if the value is one it may take.

use apl_eval::Saved;

/// Set the setting `name` -- `ORIGIN`, `DIGITS` or `WIDTH` -- to
/// `value`, and give back what it was. `None`, changing nothing, when
/// `name` is no setting or `value` is not one it may take: an origin
/// of 0 or 1, from 1 to 16 digits, a width from 30 to 254.
pub fn setting(saved: &mut Saved, name: &str, value: &str) -> Option<String> {
    let number = value.parse::<usize>().ok()?;
    let was = match (name, number) {
        ("ORIGIN", n @ (0 | 1)) => {
            let io = i64::try_from(n).unwrap_or(1);
            std::mem::replace(&mut saved.env.io, io).to_string()
        }
        ("DIGITS", n @ 1..=16) => std::mem::replace(&mut saved.print.digits, n).to_string(),
        ("WIDTH", n @ 30..=254) => std::mem::replace(&mut saved.print.width, n).to_string(),
        _ => return None,
    };
    Some(was)
}
