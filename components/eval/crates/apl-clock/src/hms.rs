//! How a span of time reads.

/// Sixtieths of a second as APL\360 printed a duration: hours, then
/// minutes and seconds in two figures each.
#[must_use]
pub fn hms(sixtieths: i64) -> String {
    let seconds = sixtieths.max(0) / 60;
    let (minutes, hours) = (seconds / 60, seconds / 3600);
    format!("{hours}.{:02}.{:02}", minutes % 60, seconds % 60)
}
