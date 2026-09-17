//! Calling a defined function: check the valence, shadow the locals
//! in a fresh frame, run the body, take the result, restore. How a
//! body line is evaluated is the caller's business, handed in as
//! `Run`, so this stays below the evaluator rather than inside it.

mod call;

pub use call::{Run, call, value};
