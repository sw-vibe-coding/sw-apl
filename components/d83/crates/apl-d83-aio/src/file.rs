//! A transfer file's text, written and read.

use apl_d83_transfer::{Object, SYSTEM, form, parse};
use apl_workspace::Workspace;

/// What a transfer file says first: when it was written, which `)IN`
/// reports. A comment to APL, as a workspace's directives are.
const SAVED: &str = "⍝!SAVED ";

/// The text of a transfer file written `when`, holding the objects
/// `names` names -- or, given none, every variable, every function
/// and the system variables a workspace keeps. A locked function has
/// no transfer form and is left out of the whole; named, it is not
/// found.
///
/// # Errors
/// The names asked for that have no transfer form, when any have none.
pub fn compose(ws: &Workspace, names: &[&str], when: &str) -> Result<String, Vec<String>> {
    let missing: Vec<String> = names
        .iter()
        .filter(|n| form(ws, n).is_none())
        .map(|n| (*n).to_string())
        .collect();
    if !missing.is_empty() {
        return Err(missing);
    }
    let chosen = if names.is_empty() {
        every(ws)
    } else {
        names.iter().map(|n| (*n).to_string()).collect()
    };
    let forms = chosen.iter().filter_map(|n| form(ws, n));
    Ok(forms.fold(format!("{SAVED}{when}\n"), |text, f| text + &f + "\n"))
}

/// Every name a whole workspace is written with: the variables, the
/// functions, each sorted, then the system variables it keeps.
fn every(ws: &Workspace) -> Vec<String> {
    let mut variables: Vec<String> = ws.saved.vars.keys().cloned().collect();
    let mut functions: Vec<String> = ws.saved.funcs.keys().cloned().collect();
    variables.sort();
    functions.sort();
    variables.extend(functions);
    variables.extend(SYSTEM.iter().map(|n| (*n).to_string()));
    variables
}

/// What a transfer file holds.
pub struct Contents {
    /// When it was written, as its first line says; empty when it does
    /// not say.
    pub when: String,
    /// Its objects, in the order written.
    pub objects: Vec<Object>,
}

/// The contents of a transfer file. Reading stops at the first line
/// that is not a transfer form.
#[must_use]
pub fn contents(text: &str) -> Contents {
    let (when, body) = match text.strip_prefix(SAVED) {
        Some(rest) => rest.split_once('\n').unwrap_or((rest, "")),
        None => ("", text),
    };
    let chars: Vec<char> = body.chars().collect();
    let mut at = 0;
    let mut objects = Vec::new();
    while let Some((object, used)) = chars.get(at..).and_then(parse) {
        objects.push(object);
        at += used;
        if chars.get(at) == Some(&'\n') {
            at += 1;
        }
    }
    Contents {
        when: when.to_string(),
        objects,
    }
}
