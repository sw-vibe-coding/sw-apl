//! Which of PC APL's commands, and what it does.

use apl_modes::Feature;
use apl_workspace::Workspace;

use crate::listing::{drop_file, list_files};
use crate::transfer::{inward, outward};

/// Run one of PC APL's own commands, `name` in capitals and the words
/// after it as typed. `None` when it is not one of them, or the mode
/// has not got it, and the caller answers it as it would any other.
pub fn command(ws: &mut Workspace, name: &str, rest: &[&str]) -> Option<Vec<String>> {
    let profile = ws.mode.profile();
    if profile.has(Feature::Reset) && reset(ws, name, rest) {
        return Some(Vec::new());
    }
    if !profile.has(Feature::TransferForm) {
        return None;
    }
    match name {
        "IN" => Some(inward(ws, rest)),
        "OUT" => Some(outward(ws, rest)),
        "LIB" => list_files(ws, rest),
        "DROP" => drop_file(ws, rest),
        _ => None,
    }
}

/// `)RESET`, and `)SI CLEAR` with it, clear the state indicator
/// (APL Reference, May 1983, page 12-10): every suspended function and
/// every one waiting on it, at once, where `→` clears one suspension.
/// Neither says anything. False for any other command.
fn reset(ws: &mut Workspace, name: &str, rest: &[&str]) -> bool {
    match (name, rest) {
        ("RESET", []) => {}
        ("SI", [word]) if word.eq_ignore_ascii_case("CLEAR") => {}
        _ => return false,
    }
    while !ws.si().is_empty() {
        ws.leave();
    }
    true
}
