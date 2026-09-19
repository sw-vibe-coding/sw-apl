//! Taking names out of a stored workspace.
//!
//! Copying is not loading: running the file would apply its
//! settings, and the index origin would change under code already
//! written. Copying therefore takes the definitions and leaves the
//! commands. See `docs/index-origin-considerations.md`.

mod copy;

pub use copy::{Taken, take};
