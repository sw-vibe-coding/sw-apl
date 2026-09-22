//! Where the shared core asks for a '68-only command.

use apl_eval::{Mode, Saved};

use crate::group::grouping;
use crate::settings::settings_command;

/// Answer a '68-only command, or `None` when this is not one or the
/// session is not in (A): outside (A) the name is one the system does
/// not have, and the caller answers it as such.
pub fn command(saved: &mut Saved, mode: Mode, name: &str, rest: &[&str]) -> Option<Vec<String>> {
    if mode != Mode::A {
        return None;
    }
    if let Some(lines) = grouping(saved, name, rest) {
        return Some(lines);
    }
    settings_command(saved, name, rest).map(|line| vec![line])
}
