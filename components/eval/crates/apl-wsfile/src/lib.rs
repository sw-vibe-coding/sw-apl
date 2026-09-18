//! The workspace file: a saved workspace written as APL you could
//! have typed. Settings are the commands that set them, variables are
//! assignments, functions are del definitions. Reading one back is
//! therefore not parsing but running it, which the session already
//! knows how to do -- so this crate only writes.
//!
//! A workspace file is also a sample, and a sample is also a
//! workspace. That is the point of the format.
//!
//! Two things APL cannot say about itself travel as `⍝!` directives,
//! which are comments to the interpreter and instructions to `)LOAD`:
//! when the workspace was saved, and where its random link stands.

mod groups;
mod literal;
mod write;

pub use groups::{definitions, expand, plain};
pub use literal::literal;
pub use write::{DIRECTIVE, PREAMBLE, rot13, write};
