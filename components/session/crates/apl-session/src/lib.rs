//! sw-apl session: one input line in, transcript lines out. Owns the
//! workspace, the del editor's open definition, and the print
//! settings; knows nothing about terminals.

mod commands;
mod reply;
mod session;

pub use apl_eval::{
    Console, Files, Host, INDENT, Memory, Mode, QUOTA, Shelf, Shown, Store, system,
};
pub use reply::Reply;
pub use session::Session;
