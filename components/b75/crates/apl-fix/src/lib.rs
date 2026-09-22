//! A function as characters and back: canonical representation,
//! `⎕CR`, and fix, `⎕FX` -- two of the system functions (B) '75 has
//! and (A) '70 does not (IBM 5110 APL Reference Manual, Chapter 5;
//! APL/CMS User's Manual for APLSV's rules). They meet the del
//! editor: a locked function's characters are not to be had, and a
//! function fixed from characters is one the editor could have made.
//! See docs/mode-b.md.

mod chars;
mod fix;

pub use chars::{matrix, rows};
pub use fix::{canonical, fix};
