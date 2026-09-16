//! APL\360 output formatting: numbers to quad-PP significant digits
//! with the high minus, vectors space-separated, matrices with
//! right-aligned columns.

mod array;
mod number;

pub use array::format_array;
pub use number::format_number;
