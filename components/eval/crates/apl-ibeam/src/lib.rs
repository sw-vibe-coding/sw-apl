//! The I-beam system functions: a monadic function whose integer
//! argument selects a system value, as in APL\360.
//!
//! What varies between runs -- the time, the date, the processor time
//! used -- is read through a `Clock` the workspace holds, so a test or
//! a transcript can give it one that does not move.

mod clock;
mod ibeam;

pub use clock::{Clock, Time, stopped, system};
pub use ibeam::{argument, ibeam};
