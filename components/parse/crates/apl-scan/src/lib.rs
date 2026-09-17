//! What the parser reads straight off the token stream, before it
//! starts recursing: matching brackets, top-level semicolon segments,
//! which tokens end an operand, and the del header.

mod header;
mod scan;

pub use header::parse_header;
pub use scan::{Funcs, ends_operand, matching, segments};
