//! What each mode can do, asked as questions rather than by mode.
//!
//! Code that differs by mode asks the session's profile -- has it the
//! I-beams, a scan, a limit on functions -- and never which mode it is
//! in, so a mode is added here and in `apl-modes`' `Mode::profile`, and
//! not as an arm in every `match` on `Mode` (docs/architecture.md,
//! "Adding a mode").

mod profile;

pub use profile::{A70, B75, C80, Cut, Feature, Profile};
