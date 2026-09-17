//! sw-apl session: one input line in, transcript lines out. Owns the
//! workspace and the print settings; knows nothing about terminals.

mod commands;
mod render;
mod session;

pub use session::{INDENT, Reply, Session};
