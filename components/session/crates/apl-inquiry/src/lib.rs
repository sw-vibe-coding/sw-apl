//! The inquiry commands: what a workspace holds, and the group
//! facility for handling several names as one.
//!
//! These are `apl-commands`' neighbours rather than its contents:
//! that crate is at its module budget, and listing names has little
//! to do with reading and writing workspace files.

mod group;
mod inquiry;
mod names;

pub use group::{erase, group, members};
pub use inquiry::command;
pub use names::{functions, globals, groups, listing};
