//! Libraries beyond 0 and 1, configured outside the language when a
//! session starts: `--lib N=DIR[,NAME]` on the command line, and a
//! TOML file -- `--config FILE`, else `./sw-apl.toml`, else
//! `sw-apl/config.toml` in the user's configuration directory. A
//! flag wins over the file for the same number. The CLI and the
//! service take the same.
//!
//! ```toml
//! [[library]]
//! number = 2
//! name = "EXTENDED"
//! path = "../sw-apl-workspaces/ws"
//! ```

mod file;
mod spec;

pub use file::{configured, from_file};
pub use spec::{LibrarySpec, attach, parse_lib};
