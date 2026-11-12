//! A session in (C) '80, where it differs: functions are defined and
//! changed by `)DEF` and `)EDIT`, not the del, and `)EXEC` runs the
//! latent expression (owner, 2026-11-11; docs/mode-c.md); `)DOS` takes
//! the keyboard until its RETURN. These work on the workspace and the
//! function open by command, and hand the session back what it alone
//! can do: run a line.

mod define;
mod dos;
mod lines;

pub use define::{Command, command};
pub use lines::line;
