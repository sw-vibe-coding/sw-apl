//! Somewhere to keep a saved workspace.
//!
//! A workspace is small UTF-8 text -- `apl-wsfile` turns one into a
//! `String` and back -- so nothing about keeping one needs a file.
//! The filesystem is simply the place the CLI and the service keep
//! them; a browser has no filesystem and keeps them elsewhere. This
//! is the seam between the two, and everything above it works on
//! text.

mod files;
mod memory;
mod store;

pub use files::Files;
pub use memory::{Memory, READ_ONLY, Shelf};
pub use store::{Library, SUFFIX, Store};
