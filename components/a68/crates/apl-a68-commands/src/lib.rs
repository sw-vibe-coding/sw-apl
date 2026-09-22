//! The system commands only (A) '68 has: `)ORIGIN`, `)DIGITS` and
//! `)WIDTH`, and `)GROUP`, `)GRP` and `)GRPS`. The IBM 5100 family
//! dropped them (docs/mode-b.md); the shared core reaches them only in
//! (A).

mod group;
mod settings;

pub use group::{group, grouping, groups, members};
pub use settings::settings_command;
