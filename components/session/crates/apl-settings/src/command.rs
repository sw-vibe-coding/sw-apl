//! The settings commands: `)ORIGIN`, `)DIGITS` and `)WIDTH`.
//!
//! These are APL\360's. The IBM 5100 family dropped them in favour of
//! its system variables (docs/mode-b.md), so they are '68-only.

use apl_eval::Saved;
use apl_library::INCORRECT;

use crate::setting::setting;

/// Answer one settings command, or `None` when it is not one. A
/// setting replies with the value it replaced, as APL\360 did; one
/// given no value, several, or one it may not take is INCORRECT
/// COMMAND.
pub fn settings_command(saved: &mut Saved, name: &str, rest: &[&str]) -> Option<String> {
    if !matches!(name, "ORIGIN" | "DIGITS" | "WIDTH") {
        return None;
    }
    let was = match rest {
        [value] => setting(saved, name, value),
        _ => None,
    };
    Some(was.map_or_else(|| INCORRECT.to_string(), |was| format!("WAS {was}")))
}
