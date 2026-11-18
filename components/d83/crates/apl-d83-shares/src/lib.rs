//! (D) '83's shared-variable processor, as IBM PC APL's manual (May
//! 1983, chapter 9) gives it: a name offered to another processor,
//! the degree of coupling it reaches, access control, retraction, and
//! the processors themselves -- the auxiliary processors that a
//! shared variable couples to, as AP205 couples to the screen.
//!
//! The processors answer at once. What the APL side sets reaches its
//! partner as it is set, and what the partner sets in reply is there
//! the next time the variable is used, so an access control setting
//! is kept and reported but never has a wait to impose, and the
//! processors set none of their own. See
//! docs/ibm-pc-apl-plan.md.

mod access;
mod offer;
mod shares;

pub use shares::{Processor, Shares};
