//! Where the shared core asks for a '70-only command.

use apl_eval::{Feature, Mode, Saved};

use crate::group::grouping;
use crate::settings::settings_command;

/// Answer a '70-only command, or `None` when this is not one or the
/// session's mode has no settings commands: there the name is one the system does
/// not have, and the caller answers it as such.
pub fn command(saved: &mut Saved, mode: Mode, name: &str, rest: &[&str]) -> Option<Vec<String>> {
    if !mode.profile().has(Feature::Settings) {
        return None;
    }
    if let Some(lines) = grouping(saved, name, rest) {
        return Some(lines);
    }
    settings_command(saved, name, rest).map(|line| vec![line])
}
