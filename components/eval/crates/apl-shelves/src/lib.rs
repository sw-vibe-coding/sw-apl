//! The libraries as each mode sees them.
//!
//! A workspace is listed and loaded only in the modes its `⍝!MODES`
//! line names. One that runs in both is kept once and shown in both;
//! that is an optimisation nobody sees. Saving or dropping in one mode
//! never touches what another mode keeps as its own, which is how two
//! workspaces come to share a name. See `docs/workspaces.md`.
//!
//! The stores underneath know nothing of this. They keep text under
//! keys; this crate decides which key a mode means by a name.

mod find;
mod keep;

pub use find::{list, read};
pub use keep::{forget, keep};
