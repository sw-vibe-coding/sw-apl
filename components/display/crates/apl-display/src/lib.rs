//! APL\360 output formatting: numbers to `)DIGITS` significant
//! digits with the high minus, vectors space-separated, matrices
//! with right-aligned columns, planes separated by blank lines,
//! and `)WIDTH` wrapping with six-space continuation lines.

mod array;
mod number;
mod wrap;

pub use array::format_array;
pub use number::format_number;
