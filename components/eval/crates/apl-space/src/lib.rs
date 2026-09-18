//! Workspace accounting: what a value costs, what the symbol table
//! costs altogether, and whether something more will fit.
//!
//! The figures here are a model, not a measurement. Nothing in this
//! crate asks Rust how much memory an `Array` occupies: a workspace
//! has to be the same size on every machine, at every optimisation
//! level, and in a test that states a number. So a value costs what
//! APL\360 would have charged for it -- a descriptor, an entry per
//! axis, and eight bytes a number or one a character -- and a
//! workspace holds what those add up to.

mod quota;
mod size;

pub use quota::{DEFAULT, free, room};
pub use size::{Funcs, Groups, Vars, of_function, of_name, of_value, used};
