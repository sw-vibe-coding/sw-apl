//! Format, `⍕`, which (B) '75 has and (A) '70 does not: the IBM 5110
//! APL Reference Manual, "The ⍕ Function: Format".
//!
//! Monadic format is the display as characters, so it is the display
//! crate's own formatting, not a copy of it. Dyadic format lays each
//! number out in a field of a given width, in decimal form to a given
//! number of places or in scaled form to a given number of digits.

mod dyadic;
mod monadic;

pub use dyadic::dyadic;
pub use monadic::monadic;
