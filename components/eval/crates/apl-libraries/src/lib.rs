//! Libraries beyond 0 and 1: numbered by configuration, read-only,
//! and kept somewhere else -- a directory at the terminal and the
//! service, text fetched from a URL in a browser. The owner's
//! workspaces repository is library 2, EXTENDED, by convention.
//!
//! `Added` wraps the store a session already has, so libraries 0 and
//! 1 are exactly what they were, and answers for the rest. A library
//! keeps library 1's file naming and modes lines, so `)LIB N` and
//! `)LOAD N NAME` see only what runs in the mode.

mod added;
mod build;
mod source;

pub use added::Added;
pub use source::Source;
