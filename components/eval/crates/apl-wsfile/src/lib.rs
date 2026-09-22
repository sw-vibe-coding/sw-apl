//! The workspace file: a saved workspace written as APL you could
//! have typed. Settings are the commands that set them, variables are
//! assignments, functions are del definitions. Reading one back is
//! therefore not parsing but running it, which the session already
//! knows how to do -- so this crate only writes.
//!
//! A workspace file is also a sample, and a sample is also a
//! workspace. That is the point of the format.
//!
//! What APL cannot say about itself, or cannot say the same way in
//! every mode, travels as `⍝!` directives, which are comments to APL
//! and instructions to sw-apl: the modes the workspace runs in, when
//! it was saved, where its random link stands, and its index origin,
//! digits and width. The settings were once written as the commands
//! that set them, which only '70 has.

mod groups;
mod literal;
mod write;

pub use groups::{definitions, expand, plain};
pub use literal::literal;
pub use write::{DIRECTIVE, PREAMBLE, rot13, write};
