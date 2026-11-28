//! (D) '83's AP210 files. The manual (May 1983, page 3-22) names a
//! file `[d:]filename[.ext]`; every drive is library 0 here, and the
//! file is kept there under its name and `.DOS`, as a transfer file
//! is under `.AIO`. sw-apl keeps the records, not bytes: UTF-8 text,
//! a first line naming it, then a record to a line -- its code, its
//! shape and its elements -- and an empty line for a record never
//! written. See docs/ibm-pc-apl-plan.md.

mod elements;
mod file;
mod line;

pub use file::{key, load, save};
