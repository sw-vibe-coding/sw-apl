//! Modes: which of sw-apl's languages a workspace runs in.
//!
//! sw-apl has two modes, (A) '68 and (B) '75. A workspace names the
//! modes it runs in on a `⍝!MODES` line near its top; `apl-shelves`
//! lists and loads it only in those. See `docs/workspaces.md`.

mod host;
mod line;
mod mode;

pub use host::Host;
pub use line::{modes, render, retag};
pub use mode::{LETTERS, Mode, Modes};
