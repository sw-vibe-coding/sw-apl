//! Which command, and what it replies.

use apl_eval::{Activation, Saved, Workspace, hms};

use crate::load::{copy, lib, load};
use crate::save::{drop_workspace, moment, save};

/// What an unnamed workspace is called, as APL\360 named it.
pub const CLEAR: &str = "CLEAR WS";

/// The reply APL\360 gives to a command it does not know, or to one
/// given an argument it does not take.
pub const INCORRECT: &str = "INCORRECT COMMAND";

/// What a system command produced.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Answer {
    /// The transcript lines the command produced.
    pub lines: Vec<String>,
    /// Lines for the session to run as though they had been typed,
    /// which is how `)LOAD` and `)COPY` work: a workspace file is
    /// APL, so reading one is typing it. What they print on the way
    /// is not shown, because `)LOAD` shows only its SAVED line.
    pub feed: Vec<String>,
    /// Set by `)OFF` and `)CONTINUE`: the session has ended.
    pub off: bool,
}

/// Run one system command: the text after the parenthesis.
pub fn system_command(ws: &mut Workspace, command: &str) -> Answer {
    let mut words = command.split_whitespace();
    let name = words.next().unwrap_or("").to_ascii_uppercase();
    let rest: Vec<&str> = words.collect();
    // )OFF signs off, and )CONTINUE saves the workspace first.
    let off = matches!((name.as_str(), rest.as_slice()), ("OFF" | "CONTINUE", []));
    let mut lines = match (name.as_str(), rest.as_slice()) {
        ("OFF", []) => Vec::new(),
        ("CONTINUE", []) => save(ws, Some("CONTINUE")),
        ("SI" | "SIV", []) => si_lines(ws.si(), name == "SIV"),
        ("SAVE", []) => save(ws, None),
        ("SAVE", [id]) => save(ws, Some(id)),
        ("LOAD", _) => return load(ws, &rest),
        ("DROP", _) => drop_workspace(ws, &rest),
        ("LIB", _) => lib(ws, &rest),
        ("COPY" | "PCOPY", _) => return copy(ws, &rest, name == "PCOPY"),
        _ => vec![workspace_command(&mut ws.saved, &name, &rest)],
    };
    if off {
        lines.extend(sign_off(ws));
    }
    let feed = Vec::new();
    Answer { lines, feed, off }
}

/// A command that changes the workspace itself: its settings, the
/// name it answers to, or clearing it altogether. A setting replies
/// with the value it replaced, as APL\360 did.
fn workspace_command(saved: &mut Saved, name: &str, rest: &[&str]) -> String {
    let number = rest.first().and_then(|v| v.parse::<usize>().ok());
    let was = match (name, rest, number) {
        ("CLEAR", [], _) => {
            *saved = Saved::default();
            return CLEAR.to_string();
        }
        ("WSID", [], _) => return saved.id.clone().unwrap_or_else(|| CLEAR.to_string()),
        ("WSID", [id], _) => saved.id.replace((*id).to_string()),
        ("ORIGIN", [_], Some(n @ (0 | 1))) => {
            let io = i64::try_from(n).unwrap_or(1);
            Some(std::mem::replace(&mut saved.env.io, io).to_string())
        }
        ("DIGITS", [_], Some(n @ 1..=16)) => {
            Some(std::mem::replace(&mut saved.print.digits, n).to_string())
        }
        ("WIDTH", [_], Some(n @ 30..=254)) => {
            Some(std::mem::replace(&mut saved.print.width, n).to_string())
        }
        _ => return INCORRECT.to_string(),
    };
    format!("WAS {}", was.unwrap_or_else(|| CLEAR.to_string()))
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
    let column = entries.iter().map(|(e, _)| e.chars().count()).max();
    entries
        .iter()
        .map(|(entry, locals)| match column {
            Some(width) if verbose && !locals.is_empty() => {
                format!("{entry:width$}  {}", locals.join(" "))
            }
            _ => entry.clone(),
        })
        .collect()
}

/// The APL\360 sign-off: the time and date the session ended, then
/// how long it was connected and how much processor time it used.
/// APL\360 also named the port and the user, and carried totals to
/// date; sw-apl has no accounts and keeps no such records.
fn sign_off(ws: &Workspace) -> Vec<String> {
    let time = (ws.clock)();
    vec![
        moment(ws),
        format!("CONNECTED {}", hms(time.now - ws.signed_on)),
        format!("CPU TIME {}", hms(time.cpu)),
    ]
}
