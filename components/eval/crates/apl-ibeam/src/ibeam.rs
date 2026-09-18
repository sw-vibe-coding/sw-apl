//! The eight system values, and how a span of time among them reads.

use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

use crate::clock::Time;

/// The system value `n` selects: 20 the time of day, 21 the processor
/// time used, 22 the space available, 23 the terminals connected, 24
/// the time of sign-on, 25 today's date, 26 the line now executing,
/// 27 the lines in the state indicator. Times are in sixtieths of a
/// second since midnight, as APL\360 gave them.
///
/// `lines` is the state indicator, innermost first, and `free` is
/// the space the workspace has left: this crate does no counting of
/// its own, because what a workspace holds is the workspace's
/// business and the quota it is held against is the session's.
///
/// # Errors
/// DOMAIN ERROR for any other argument.
pub fn ibeam(n: i64, time: Time, signed_on: i64, lines: &[i64], free: i64) -> AplResult<Array> {
    let one = |v: i64| Ok(Array::scalar(Number::Int(v)));
    match n {
        20 => one(time.now),
        21 => one(time.cpu),
        22 => one(free),
        23 => one(1),
        24 => one(signed_on),
        25 => one(time.date),
        26 => one(lines.first().copied().unwrap_or(0)),
        27 => Ok(Array::vector(
            lines.iter().map(|l| Number::Int(*l)).collect(),
        )),
        _ => Err(AplError::new(ErrorKind::Domain)),
    }
}

/// The whole-number argument an I-beam was given.
///
/// # Errors
/// DOMAIN ERROR for characters, for anything but a single number, and
/// for a number that is not whole.
pub fn argument(r: &Array) -> AplResult<i64> {
    let domain = || AplError::new(ErrorKind::Domain);
    let Data::Num(v) = &r.data else {
        return Err(domain());
    };
    match v.as_slice() {
        [Number::Int(n)] => Ok(*n),
        _ => Err(domain()),
    }
}

/// Sixtieths of a second as APL\360 printed a duration: hours, then
/// minutes and seconds in two figures each.
#[must_use]
pub fn hms(sixtieths: i64) -> String {
    let seconds = sixtieths.max(0) / 60;
    let (minutes, hours) = (seconds / 60, seconds / 3600);
    format!("{hours}.{:02}.{:02}", minutes % 60, seconds % 60)
}
