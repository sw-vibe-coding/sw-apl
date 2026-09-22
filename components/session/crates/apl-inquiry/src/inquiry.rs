//! Which inquiry command, and what it replies.

use apl_eval::{Activation, Workspace, free, used};
use apl_value::{columns, pad};

use crate::erase::erase;
use crate::names::{INCORRECT, functions, globals, listing};

/// Answer one inquiry command, or `None` when it is not one of
/// these: the caller then tries the commands it knows itself.
pub fn command(ws: &mut Workspace, name: &str, rest: &[&str]) -> Option<Vec<String>> {
    let width = ws.saved.print.width;
    Some(match (name, rest) {
        ("FNS", _) => listing(sorted(functions(ws)), rest, width),
        ("VARS", _) => listing(sorted(globals(ws)), rest, width),
        ("ERASE", _) => erase(ws, rest),
        ("SYMBOLS", []) => symbols(ws),
        ("SI" | "SIV", []) => si_lines(ws.si(), name == "SIV"),
        ("SYMBOLS" | "SI" | "SIV", _) => vec![INCORRECT.to_string()],
        _ => return None,
    })
}

/// Alphabetically, which is how the manual says `)FNS` and `)VARS`
/// print. A name holds one thing, so there is nothing to
/// order two entries of the same name against.
fn sorted(mut names: Vec<String>) -> Vec<String> {
    names.sort_unstable();
    names
}

/// `)SYMBOLS`: how many names the workspace holds, and how many it
/// could hold. sw-apl has no symbol table set aside at sign-on --
/// names are charged against the workspace like everything else --
/// so the size is what the space still free would hold if every
/// further name were the shortest one, and it moves as the workspace
/// fills. `)SYMBOLS n` has nothing to set and says so.
fn symbols(ws: &Workspace) -> Vec<String> {
    let saved = &ws.saved;
    let held = saved.vars.len() + saved.funcs.len() + saved.groups.len();
    let table = used(&saved.vars, &saved.funcs, &saved.groups);
    let room = free(ws.quota, table) / apl_eval::of_name("A");
    vec![format!("IS {}, USED {held}", held + room)]
}

/// The state indicator, innermost first: each function with the line
/// it stopped on, starred when it is the one the user can take up
/// again rather than a caller waiting on it. `verbose` adds the names
/// it made local, which is what `)SIV` shows.
fn si_lines(stack: &[Activation], verbose: bool) -> Vec<String> {
    let entries: Vec<(String, &[String])> = stack
        .iter()
        .rev()
        .map(|a| {
            let star = if a.suspended { "*" } else { "" };
            (format!("{}[{}]{star}", a.name, a.line), a.locals.as_slice())
        })
        .collect();
    let column = entries.iter().map(|(e, _)| columns(e)).max();
    entries
        .iter()
        .map(|(entry, locals)| match column {
            Some(width) if verbose && !locals.is_empty() => {
                format!("{}  {}", pad(entry, width), locals.join(" "))
            }
            _ => entry.clone(),
        })
        .collect()
}
