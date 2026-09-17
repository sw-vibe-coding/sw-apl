//! Bracket indexing `A[I;J]` and indexed assignment `A[I;J]←V`.

mod index;
mod select;

pub use index::{index, indexed_assign};
