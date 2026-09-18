//! sw-apl session: one input line in, transcript lines out. Owns the
//! workspace, the del editor's open definition, and the print
//! settings; knows nothing about terminals.

mod commands;
mod render;
mod session;

pub use apl_eval::{Console, INDENT, Shown};
pub use session::{Reply, Session};
