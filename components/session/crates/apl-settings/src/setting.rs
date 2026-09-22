//! Setting one setting, if the value is one it may take.

use apl_workspace::Saved;

/// Where the random link must lie: a Lehmer generator's state is
/// never zero and always below its modulus, 2^31 - 1.
const LINKS: std::ops::RangeInclusive<u64> = 1..=2_147_483_646;

/// Set the setting `name` -- `ORIGIN`, `DIGITS`, `WIDTH` or `LINK` --
/// to `value`, and give back what it was. `None`, changing nothing,
/// when `name` is no setting or `value` is not one it may take: an
/// origin of 0 or 1, from 1 to 16 digits, a width from 30 to 254, a
/// link from 1 to 2^31 - 2.
pub fn setting(saved: &mut Saved, name: &str, value: &str) -> Option<String> {
    if name == "CT" {
        let ct = value
            .parse::<f64>()
            .ok()
            .filter(|x| (0.0..1.0).contains(x))?;
        return Some(std::mem::replace(&mut saved.env.ct, ct).to_string());
    }
    if name == "LINK" {
        let link = value.parse().ok().filter(|n| LINKS.contains(n))?;
        return Some(std::mem::replace(&mut saved.env.link, link).to_string());
    }
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
