//! What names a workspace holds, and how a list of them prints.

use apl_eval::{Referent, Workspace};
use apl_value::columns;

/// The reply to a command given an argument it does not take, in
/// the one place it is spelled.
pub use apl_library::INCORRECT;

/// The names that hold a *global* function, which is what `)FNS`
/// lists.
#[must_use]
pub fn functions(ws: &Workspace) -> Vec<String> {
    let function = |r: &Referent| matches!(r, Referent::Function(_));
    outermost(ws, function, ws.saved.funcs.keys())
}

/// The names that hold a *global* variable, which is what `)VARS`
/// lists.
#[must_use]
pub fn globals(ws: &Workspace) -> Vec<String> {
    let value = |r: &Referent| matches!(r, Referent::Value(_));
    outermost(ws, value, ws.saved.vars.keys())
}

/// The global names of one kind. Under a suspension a call's locals
/// sit in the symbol table in place of the globals they displaced, so
/// the globals are gathered from both: from what the outermost call
/// to localize a name displaced, and from `here` for the rest.
///
/// Only the outermost activation to localize a name decides: an inner
/// call that localizes the same name displaced the *outer call's
/// local*, not a global, and counting that would invent a global that
/// is not there.
fn outermost<'a>(
    ws: &Workspace,
    kind: impl Fn(&Referent) -> bool,
    here: impl Iterator<Item = &'a String>,
) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let mut localized: Vec<&str> = Vec::new();
    for (name, was) in ws.si().iter().flat_map(|a| &a.displaced) {
        if localized.contains(&name.as_str()) {
            continue;
        }
        localized.push(name);
        if was.as_ref().is_some_and(&kind) {
            names.push(name.clone());
        }
    }
    names.extend(here.filter(|n| !localized.contains(&n.as_str())).cloned());
    names
}

/// A listing as APL\360 printed one: from the letter given if one
/// was, wrapped at the print width. An argument that is not one
/// starting letter is INCORRECT COMMAND.
///
/// The order is the caller's. `)FNS`, `)VARS` and `)GRPS` sort,
/// because the manual says those print alphabetically; `)GRP` does
/// not, because the manual says only that the names in the group are
/// printed, and the order they were gathered in is worth keeping.
#[must_use]
pub fn listing(mut names: Vec<String>, rest: &[&str], width: usize) -> Vec<String> {
    let from = match rest {
        [] => String::new(),
        [letter] => (*letter).to_string(),
        _ => return vec![INCORRECT.to_string()],
    };
    names.retain(|n| n.as_str() >= from.as_str());
    let mut lines: Vec<String> = Vec::new();
    for name in names {
        match lines.last_mut() {
            Some(line) if columns(line) + 1 + columns(&name) <= width => {
                line.push(' ');
                line.push_str(&name);
            }
            _ => lines.push(name),
        }
    }
    lines
}
