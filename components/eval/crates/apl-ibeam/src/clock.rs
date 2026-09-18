//! Where the varying values come from.

use chrono::{Datelike, Local, Timelike};

/// Sixtieths of a second, the unit APL\360 reported time in.
const TICKS: i64 = 60;

/// What the clock says, in the units the I-beams report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Time {
    /// Sixtieths of a second since midnight.
    pub now: i64,
    /// Sixtieths of a second of processor time used by this process.
    pub cpu: i64,
    /// Today's date as the integer MMDDYY.
    pub date: i64,
}

/// Where a workspace reads the time. It is a plain function so a test
/// can install one that does not move and a transcript reproduces.
pub type Clock = fn() -> Time;

/// A clock that does not move: midnight on the zeroth of nothing,
/// with no processor time used. A clear workspace starts with it, so
/// nothing reads the real world unless a host asks for it.
#[must_use]
pub fn stopped() -> Time {
    Time::default()
}

/// The real time of day, processor time, and date.
#[must_use]
pub fn system() -> Time {
    let local = Local::now();
    let midnight = i64::from(local.num_seconds_from_midnight()) * TICKS;
    let sixtieths = i64::from(local.nanosecond()) * TICKS / 1_000_000_000;
    Time {
        now: midnight + sixtieths.min(TICKS - 1),
        cpu: cpu(),
        date: i64::from(local.month()) * 10_000
            + i64::from(local.day()) * 100
            + i64::from(local.year().rem_euclid(100)),
    }
}

/// Processor time this process has used, in sixtieths of a second.
/// Zero where the platform does not report it.
#[cfg(unix)]
fn cpu() -> i64 {
    let mut usage = std::mem::MaybeUninit::<libc::rusage>::zeroed();
    // SAFETY: getrusage fills the struct it is given, and RUSAGE_SELF
    // is the one value that asks about this process.
    if unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) } != 0 {
        return 0;
    }
    // SAFETY: getrusage returned 0, so the struct is initialised.
    let usage = unsafe { usage.assume_init() };
    let spent = |t: libc::timeval| t.tv_sec * TICKS + i64::from(t.tv_usec) * TICKS / 1_000_000;
    spent(usage.ru_utime) + spent(usage.ru_stime)
}

/// Zero: this platform does not report processor time.
#[cfg(not(unix))]
fn cpu() -> i64 {
    0
}
