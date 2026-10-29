//! Line editing: `[N⎕M]` in the del editor, as APL\360 had it.
//!
//! The statement is displayed and the carriage stops under position
//! M. Under it the reader types marks: `/` deletes the character
//! above, a digit K puts K blanks before it, a letter 5×R blanks, R
//! its place in the alphabet; anything else is ignored. The statement
//! is displayed again with the marks applied and the carriage at the
//! first blank put in, and what the reader types then lands on the
//! line by column, as on paper. The line that results is taken as if
//! it had been typed whole -- its number included, so changing the
//! number moves the statement.
//!
//! The source is the APL\360-OS and APL\360-DOS User's Manual
//! (SH20-0906-0), "Line editing", p. 3.18, and its example on p. A.5.

mod fill;
mod marks;
mod patch;

pub use fill::fill;
pub use marks::mark;
pub use patch::{Patch, Taken};
