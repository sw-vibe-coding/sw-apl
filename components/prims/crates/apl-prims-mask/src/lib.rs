//! Compress (`l/r`) and expand (`l\r`) along an axis with a boolean
//! left argument.

mod mask;

pub use mask::{compress, expand};
