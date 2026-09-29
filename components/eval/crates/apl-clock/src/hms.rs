//! How time reads: a span as hours, minutes and seconds, and a moment
//! as a time stamp; and whether a session has a clock to stamp with.

use crate::clock::Time;

/// Sixtieths of a second as APL\360 printed a duration: hours, then
/// minutes and seconds in two figures each.
#[must_use]
pub fn hms(sixtieths: i64) -> String {
    let seconds = sixtieths.max(0) / 60;
    let (minutes, hours) = (seconds / 60, seconds / 3600);
    format!("{hours}.{:02}.{:02}", minutes % 60, seconds % 60)
}

/// Whether a session has a clock at all, as `--clock` names it. The
/// 5110 had none, so (B)'s time stamp is 1900 unless a session is
/// started with one; (A)'s I-beams read the host's clock either way,
/// as APL\360 read its machine's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimeSource {
    /// No clock: the historical answer.
    #[default]
    None,
    /// The computer's own clock.
    Hardware,
}

impl TimeSource {
    /// The source `--clock` or a configuration file names.
    #[must_use]
    pub fn parse(word: &str) -> Option<TimeSource> {
        match word {
            "none" => Some(TimeSource::None),
            "hardware" => Some(TimeSource::Hardware),
            _ => None,
        }
    }
}

/// The time stamp `⎕TS` gives: year, month, day, hour, minute, second
/// and millisecond, as APLSV's has it.
#[must_use]
pub fn stamp(at: &Time) -> [i64; 7] {
    let (month, day) = (at.date / 10_000, at.date / 100 % 100);
    let seconds = at.now / 60;
    let (hour, minute, second) = (seconds / 3600, seconds / 60 % 60, seconds % 60);
    [
        at.year,
        month,
        day,
        hour,
        minute,
        second,
        at.now % 60 * 1000 / 60,
    ]
}
