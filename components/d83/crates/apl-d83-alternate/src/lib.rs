//! Execute alternate, `L ⎕EA R`, IBM PC APL's trap (APL Reference,
//! May 1983, page 8-6): the line `R` is run, and if it fails or is
//! interrupted it is ended without a message and the line `L` is run
//! instead.

mod alternate;
mod statement;

pub use alternate::alternate;
pub use statement::{value, whole};
