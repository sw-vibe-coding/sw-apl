//! Which of APL80's commands, and what it does.

use apl_c80_machine::Machine;
use apl_modes::Feature;
use apl_value::{Array, Data};
use apl_workspace::Workspace;

/// Run one of APL80's commands: `text` is what followed the
/// parenthesis, as typed, since `)AUTO` takes the rest of the line as
/// it stands. `None` when it is not one of them, or the mode has not
/// got it, and the caller answers it as it would any other.
pub fn command(ws: &mut Workspace, text: &str) -> Option<Vec<String>> {
    let text = text.trim_start();
    let end = text.find(' ').unwrap_or(text.len());
    let (name, rest) = (text[..end].to_ascii_uppercase(), text[end..].trim());
    let profile = ws.mode.profile();
    if rest.is_empty()
        && profile.has(Feature::Machine)
        && let Some(lines) = machine(ws, &name)
    {
        return Some(lines);
    }
    match (name.as_str(), rest.is_empty()) {
        ("TRON" | "TROFF", true) if profile.has(Feature::Trace) => ws.trace = name == "TRON",
        ("PS" | "PD", true) if profile.has(Feature::PrintPrecision) => {
            ws.saved.print.digits = if name == "PS" { 6 } else { 15 };
        }
        ("AUTO", _) if profile.has(Feature::Auto) => {
            ws.saved.latent = (!rest.is_empty()).then(|| characters(rest));
        }
        _ => return None,
    }
    Some(Vec::new())
}

/// `)RAM`, which opens the TRS-80's memory to peek, poke and call --
/// once open it stays so, and a second `)RAM` changes nothing -- and
/// `)DOS`, which hands the keyboard to DOS until its RETURN.
fn machine(ws: &mut Workspace, name: &str) -> Option<Vec<String>> {
    match name {
        "RAM" => {
            ws.machine.get_or_insert_with(Machine::default);
            Some(Vec::new())
        }
        "DOS" => {
            ws.dos = true;
            Some(vec![READY.to_string()])
        }
        _ => None,
    }
}

/// What DOS shows when it waits for a command.
pub const READY: &str = "DOS READY";

/// `text` as a character vector.
fn characters(text: &str) -> Array {
    let chars: Vec<char> = text.chars().collect();
    Array {
        shape: vec![chars.len()],
        data: Data::Char(chars),
    }
}
