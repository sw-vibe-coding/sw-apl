//! What the parser reads straight off the token stream, before it
//! starts recursing: matching brackets, top-level semicolon segments,
//! which tokens end an operand, the del header, and the labels in a
//! function body.

mod header;
mod label;
mod scan;

pub use header::parse_header;
pub use label::{labels, without_label};
pub use scan::{Funcs, ends_operand, matching, segments};
