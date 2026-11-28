//! (D) '83's AP210 records, as IBM PC APL's manual (May 1983, pages
//! 3-23 and 4-9 to 4-11) gives them. A file is read and written a
//! record at a time, under one of four codes: A, an APL object with
//! its header, which must fit the record length; B, bits; C,
//! characters; D, ASCII characters -- the last three without a
//! header, filling the record. sw-apl keeps a record as the value and
//! its code, not as bytes, so what a record of one code reads as under
//! another is decided here: C and D read each other's (both are
//! text), and anything else is a data type error. See
//! docs/ibm-pc-apl-plan.md.

mod code;
mod shape;
mod size;

pub use code::{Code, Record, read, write};
pub use size::bytes;
