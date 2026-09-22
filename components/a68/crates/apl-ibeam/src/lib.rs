//! The I-beam system functions: a monadic function whose integer
//! argument selects a system value, as in APL\360.
//!
//! '68-only: the IBM 5100 family replaced them with system variables
//! and functions (docs/mode-b.md). What varies between runs is read
//! through the `Clock` in `apl-clock`, which every mode shares.

mod ibeam;

pub use ibeam::{argument, ibeam};
