//! Which of APL80's commands, and what it does.

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

/// `text` as a character vector.
fn characters(text: &str) -> Array {
    let chars: Vec<char> = text.chars().collect();
    Array {
        shape: vec![chars.len()],
        data: Data::Char(chars),
    }
}
