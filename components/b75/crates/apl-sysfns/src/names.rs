//! Names written as characters: whether one can be a name, what it
//! names now, and expunging it.

use apl_lex::{TokenKind, tokenize};
use apl_scan::labels;
use apl_workspace::Workspace;

/// What `name` names now, as `⎕NC` gives it: 0 nothing, 1 a label,
/// 2 a variable, 3 a function, 4 not to be used as a name -- not a
/// name at all, a system name, or a group's. A label and a local are
/// the innermost call's: the active referent.
#[must_use]
pub fn class(ws: &Workspace, name: &str) -> i64 {
    if !valid(name) || ws.saved.groups.contains_key(name) {
        return 4;
    }
    let owner = ws
        .si()
        .iter()
        .rev()
        .find(|a| a.locals.iter().any(|n| n == name));
    let owner = owner.and_then(|a| ws.function(&a.name));
    if owner.is_some_and(|f| labels(&f.body).iter().any(|(n, _)| n == name)) {
        return 1;
    }
    match (ws.get(name), ws.is_function(name)) {
        (Some(_), _) => 2,
        (None, true) => 3,
        (None, false) => 0,
    }
}

/// `⎕EX` of one name: erase its active referent and say whether the
/// name is now free. A label, a group, and a function that is running
/// or waiting are not erased, and neither is what is not a name.
pub fn expunge(ws: &mut Workspace, name: &str) -> bool {
    let running = ws.si().iter().any(|a| a.name == name);
    match class(ws, name) {
        0 => true,
        2 => ws.saved.vars.remove(name).is_some(),
        3 if !running => ws.saved.funcs.remove(name).is_some(),
        _ => false,
    }
}

/// True when `name` is one name as a header or an assignment would
/// take it. A quad name is not: those are the system's.
fn valid(name: &str) -> bool {
    let tokens = tokenize(name, "");
    matches!(tokens.as_deref(), Ok([t]) if t.kind == TokenKind::Name(name.to_string()))
}
