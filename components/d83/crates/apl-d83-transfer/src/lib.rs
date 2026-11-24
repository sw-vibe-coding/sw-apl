//! (D) '83's transfer form, as IBM PC APL's manual (May 1983, page
//! 8-11) gives it: an object as one character vector -- a type, its
//! name, its rank and shape, and its elements -- and back. `⎕TF` is
//! the system function; `)IN` and `)OUT` carry the same forms in and
//! out of `.AIO` files. See docs/ibm-pc-apl-plan.md.

mod establish;
mod form;
mod parse;

pub use establish::{establish, transfer};
pub use form::{SYSTEM, form};
pub use parse::{Object, parse};
