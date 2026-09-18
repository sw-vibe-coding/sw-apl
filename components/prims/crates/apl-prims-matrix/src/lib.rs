//! Domino: matrix inverse and matrix divide.
//!
//! Not in the August 1968 APL\360 manual -- domino was added to
//! APL\360 in 1970, which is why it is the last primitive here. The
//! rules followed are the ones the APLX Language Manual states for
//! the same lineage: both arguments rank 2 or less, vectors treated
//! as one-column matrices, scalars as one by one, a singular right
//! argument a DOMAIN ERROR.

mod divide;
mod house;

pub use divide::{matrix_divide, matrix_inverse};
