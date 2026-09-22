//! The half of a workspace that a file holds: the symbol table, the
//! activation stack and the settings, with no terminal attached.
//! Kept apart from `apl-workspace` so that what a call does to the
//! names -- a local hiding a global of either kind, and giving it
//! back -- is plain data, which the writer can unwind as well.

mod activation;
mod saved;

pub use activation::{Activation, Referent};
pub use saved::Saved;
