//! A number as the decimal digits it is written with, rounded.
//!
//! Rounding starts from the number as written -- its shortest decimal
//! form -- and goes half to even. The manual's own example rounds
//! `¯123.45` to one place as `¯123.4`; the binary double nearest 123.45
//! is a little above it, so rounding that would give `¯123.5`. The
//! 5110's arithmetic was not binary; rounding the written form, half
//! to even, gives what its manual shows. A decision, not a source.

/// The digits of `|x|`, with no leading zeros, and the power of ten
/// the first one stands for plus one: `(123, 2)` for 12.3, since
/// 12.3 is 0.123 × 10². Zero is no digits.
pub(crate) fn digits(x: f64) -> (Vec<u8>, i32) {
    let text = format!("{}", x.abs());
    let (int, frac) = text.split_once('.').unwrap_or((&text, ""));
    let all: Vec<u8> = int.bytes().chain(frac.bytes()).map(|b| b - b'0').collect();
    let lead = all.iter().take_while(|&&d| d == 0).count();
    let exponent = i32::try_from(int.len()).unwrap_or(i32::MAX) - i32::try_from(lead).unwrap_or(0);
    let mut kept = all[lead..].to_vec();
    while kept.last() == Some(&0) {
        kept.pop();
    }
    (kept, exponent)
}

/// `|x|` rounded to `keep` digits from its first, half to even: the
/// digits, padded with zeros to `keep`, and the exponent. A rounding
/// that carries into a new first digit gives one digit more and an
/// exponent one higher: 9.96 to two digits is 1, 0, 0 and 2.
#[must_use]
pub fn rounded(x: f64, keep: i32) -> (Vec<u8>, i32) {
    let (mut ds, mut e) = digits(x);
    if ds.is_empty() {
        return (vec![0; usize::try_from(keep.max(0)).unwrap_or(0)], e);
    }
    let Ok(k) = usize::try_from(keep) else {
        return (Vec::new(), e);
    };
    let round = ds.len() > k && up(&ds, k);
    ds.truncate(k);
    if round && carry(&mut ds) {
        ds.insert(0, 1);
        e += 1;
    }
    ds.resize(ds.len().max(k), 0);
    (ds, e)
}

/// Whether dropping the digits from `k` on rounds up, half to even.
fn up(ds: &[u8], k: usize) -> bool {
    let rest = &ds[k..];
    let even = k == 0 || ds[k - 1].is_multiple_of(2);
    match rest[0] {
        0..=4 => false,
        5 if rest[1..].iter().all(|&d| d == 0) => !even,
        _ => true,
    }
}

/// Add one at the last digit; true when it carried out of the first.
fn carry(ds: &mut [u8]) -> bool {
    for d in ds.iter_mut().rev() {
        if *d < 9 {
            *d += 1;
            return false;
        }
        *d = 0;
    }
    true
}
