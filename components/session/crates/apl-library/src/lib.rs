//! Workspace libraries: which workspace a command means, and what
//! to say when there is not one.
//!
//! Split out of `apl-commands` when the trouble reports arrived:
//! that crate was at its module budget, and every command that names
//! a workspace needs the same resolution and the same answers when
//! it fails.

mod name;
mod report;
mod which;

pub use name::valid;
pub use report::{IMPROPER_LIBRARY, INCORRECT, OBJECT_NOT_FOUND, WS_NOT_FOUND, not_saved};
pub use which::{Named, Stored, holds, library, named, text};
