//! The clock: the time, the date and the processor time, read through
//! a `Clock` the workspace holds so a test or a transcript can give it
//! one that does not move; and how a span of time reads.
//!
//! Every mode has these. The I-beams that report them in '70 are in
//! `apl-ibeam`, which only (A) reaches.

mod clock;
mod hms;

pub use clock::{Clock, Time, stopped, system};
pub use hms::hms;
