//! sw-apl value model: one semantic number type with an integer fast
//! path, flat arrays of numbers or characters, and APL\360 errors.
//!
//! Facade only; behaviour lives in the named modules.

mod array;
mod error;
mod number;

pub use apl_glyphs::{
    AXIS, DYADIC, LATER, LOOKALIKE, MODE_B, MONADIC, OVERSTRIKE, OVERSTRIKE_B, PRIMITIVE_NAMES,
    PRIMITIVES, SYNTAX, UNDERBAR, UNDERSCORE, UNDERSCORED, columns, pad,
};
pub use array::{Array, Data};
pub use error::{AplError, Context, ErrorKind};
pub use number::{FUZZ, Number};

/// Result type used throughout the interpreter.
pub type AplResult<T> = Result<T, AplError>;
