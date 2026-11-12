//! Modes: which of sw-apl's languages a workspace runs in.
//!
//! sw-apl has three modes, (A) '70, (B) '75 and (C) '80. A workspace names the
//! modes it runs in on a `⍝!MODES` line near its top; `apl-shelves`
//! lists and loads it only in those. See `docs/workspaces.md`.

mod extra;
mod line;
mod mode;

pub use apl_profile::{Cut, Feature, Profile};
pub use line::{modes, render, retag};
pub use mode::{Host, LETTERS, Mode, Modes};
