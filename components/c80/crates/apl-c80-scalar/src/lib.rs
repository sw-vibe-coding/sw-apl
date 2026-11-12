//! The scalar functions (C) '80 leaves out: APL80's manual says its
//! hyperbolic functions are not implemented and the arguments of `!`
//! must be integers (docs/mode-c.md). Every scalar function's inner
//! loop -- plain, reduced, scanned or in a product -- comes through the
//! scalar dispatcher, which asks here, so one check covers them all.

mod cut;

pub use cut::{circular, dyadic, factorial, leave_out};
