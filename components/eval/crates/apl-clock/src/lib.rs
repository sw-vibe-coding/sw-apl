//! The clock: the time, the date and the processor time, read through
//! a `Clock` the workspace holds so a test or a transcript can give it
//! one that does not move; how a span of time and a moment read; and
//! whether a session has a clock at all.
//!
//! Every mode has these. The I-beams that report them in '70 are in
//! `apl-ibeam`, which only (A) reaches.

mod clock;
mod hms;
#[cfg(unix)]
mod ticks;

pub use clock::{Clock, Time, stopped, system};
pub use hms::{TimeSource, hms, stamp};
