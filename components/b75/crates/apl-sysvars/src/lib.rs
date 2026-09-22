//! The system variables: quad-named values that control the system or
//! report on it, as the IBM 5110 APL Reference Manual (Chapter 5)
//! gives them. (B) '75 has them and (A) '70 does not; the lexer only
//! makes `⎕IO` one name in (B). See docs/mode-b.md.
//!
//! The settings among them -- `⎕IO`, `⎕PP`, `⎕PW`, `⎕RL` -- are not
//! a second copy of anything: they read and set the workspace's own
//! index origin, precision, width and random link, through the same
//! `apl-settings` bounds the '70 commands and the directives use.

mod assign;
mod av;
mod read;

pub use assign::assign;
pub use av::atomic_vector;
pub use read::{form, latent, read};
