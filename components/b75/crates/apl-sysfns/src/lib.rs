//! The system functions: quad-named functions on the workspace's own
//! names and definitions, as the IBM 5110 APL Reference Manual
//! (Chapter 5) gives them, with APLSV's rules where it is silent
//! (the APL/CMS User's Manual). (B) '75 has them and (A) '70 does
//! not; the lexer only makes `⎕FX` one name in (B). See
//! docs/mode-b.md.
//!
//! `⎕CR` and `⎕FX` are `apl-fix`'s, and `⎕CC` is
//! `apl-console-control`'s; this crate says which quad names are
//! functions and applies them.

mod call;
mod list;
mod names;

pub use call::{dyadic, is_function, monadic};
