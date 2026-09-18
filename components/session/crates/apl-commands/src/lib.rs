//! The system commands: the input lines that begin with a right
//! parenthesis. They act on the workspace and on the library
//! directories, and know nothing of the session's other state, which
//! is why they live below it.

mod command;
mod load;
mod save;

pub use command::{Answer, system_command};
pub use save::library;
