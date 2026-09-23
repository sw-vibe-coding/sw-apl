//! Calling a defined function: check the valence, shadow the locals
//! and the labels in a fresh activation, run the body from line 1
//! following its branches, take the result, restore. How a body line
//! is evaluated is the caller's business, handed in as `Run`, so this
//! stays below the evaluator rather than inside it.
//!
//! A body that fails leaves its activation on the stack instead of
//! unwinding, which is what suspension is: the locals stay visible,
//! `)SI` reports where it stopped, and a branch takes it up again.

mod branch;
mod call;
mod stack;

pub use apl_workspace::Run;
pub use branch::{emit, interrupted, target as branch_target};
pub use call::{call, value};
pub use stack::{clear, resume, suspend};
