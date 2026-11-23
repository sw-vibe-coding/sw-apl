//! IBM PC APL's picture format, `L⍕R` with a character pattern `L`:
//! numbers laid out in the pattern's numerical fields, with the
//! decorations the pattern gives them, as the APL Reference (May 1983)
//! gives it on pages 7-64 to 7-69, under the format control `⎕FC`
//! (pages 8-18 and 8-19). Reading the pattern is `apl-d83-field`'s.

mod field;
mod picture;
mod scaled;

pub use picture::{CLEAR, picture};
