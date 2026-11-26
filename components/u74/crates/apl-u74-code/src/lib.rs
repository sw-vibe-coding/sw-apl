//! The IBM 029's card code: which of a column's twelve rows each
//! character punches. The table is the 029's, as ibm-1130-rs gives it
//! (`punch-card-core`, `hollerith.rs`; docs/citations.md).

mod code;

pub use code::{Holes, ROWS, holes};
