//! Which of PC APL's commands, and what it does.

use apl_modes::Feature;
use apl_workspace::Workspace;

/// Run one of PC APL's own commands, `name` in capitals and the words
/// after it as typed. `None` when it is not one of them, or the mode
/// has not got it, and the caller answers it as it would any other.
///
/// `)RESET`, and `)SI CLEAR` with it, clear the state indicator
/// (APL Reference, May 1983, page 12-10): every suspended function and
/// every one waiting on it, at once, where `→` clears one suspension.
/// Neither says anything.
pub fn command(ws: &mut Workspace, name: &str, rest: &[&str]) -> Option<Vec<String>> {
    if !ws.mode.profile().has(Feature::Reset) {
        return None;
    }
    match (name, rest) {
        ("RESET", []) => {}
        ("SI", [word]) if word.eq_ignore_ascii_case("CLEAR") => {}
        _ => return None,
    }
    while !ws.si().is_empty() {
        ws.leave();
    }
    Some(Vec::new())
}
