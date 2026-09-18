//! What names a workspace holds, and how a list of them prints.

use apl_eval::{Saved, Workspace};

/// The reply to a command given an argument it does not take, in
/// the one place it is spelled.
pub use apl_library::INCORRECT;

/// The names that hold a defined function.
#[must_use]
pub fn functions(saved: &Saved) -> Vec<String> {
    saved.funcs.keys().cloned().collect()
}

/// The names that hold a group.
#[must_use]
pub fn groups(saved: &Saved) -> Vec<String> {
    saved.groups.keys().cloned().collect()
}

/// The names that hold a *global* variable, which is what `)VARS`
/// lists. Under a suspension a call's locals sit in the symbol table
/// in place of the globals they displaced, so the globals have to be
/// gathered from both.
///
/// The stack is walked outermost first, and only the outermost
/// activation to localize a name decides: an inner call that
/// localizes the same name displaced the *outer call's local*, not a
/// global, and counting that would invent a global that is not there.
#[must_use]
pub fn globals(ws: &Workspace) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let mut localized: Vec<&str> = Vec::new();
    for activation in ws.si() {
        for (name, was) in &activation.displaced {
            if localized.contains(&name.as_str()) {
                continue;
            }
            localized.push(name);
            if was.is_some() {
                names.push(name.clone());
            }
        }
    }
    let here = ws.saved.vars.keys();
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
            Some(line) if line.chars().count() + 1 + name.chars().count() <= width => {
                line.push(' ');
                line.push_str(&name);
            }
            _ => lines.push(name),
        }
    }
    lines
}
