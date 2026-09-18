//! Reading a line part way through a statement: quad evaluates what
//! is typed, quote-quad takes it as characters. Both go through the
//! workspace's console, which is handed everything the statement has
//! shown so far so the prompt lands after it and not before it.

mod read;

pub use read::{characters, evaluated};
