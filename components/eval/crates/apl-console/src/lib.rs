//! The terminal as the interpreter sees it: what a statement produces,
//! how that reads as transcript lines, and where a statement gets a
//! line when it reads one.
//!
//! A read has to happen part way through a statement, after whatever
//! the statement has already shown. Putting the console here, below
//! the evaluator, is what lets the prompt land in the right place
//! without the evaluator knowing anything about terminals.

mod console;
mod output;
mod render;

pub use console::{Console, INDENT};
pub use output::{Output, Print, Shown};
pub use render::{error_lines, render, render_all};
