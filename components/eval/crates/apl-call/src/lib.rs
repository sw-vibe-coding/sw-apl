//! Calling a defined function: check the valence, shadow the locals
//! and the labels in a fresh frame, run the body from line 1 following
//! its branches, take the result, restore. How a body line is
//! evaluated is the caller's business, handed in as `Run`, so this
//! stays below the evaluator rather than inside it.

mod branch;
mod call;

pub use branch::target as branch_target;
pub use call::{Run, call, value};
