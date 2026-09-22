//! Which command, and what it replies.

use apl_a70_commands::command as seventy;
use apl_eval::{Saved, Workspace, hms};
use apl_inquiry::command as inquiry;
use apl_library::valid;

use crate::load::{copy, load};
use crate::save::{drop_workspace, lib, moment, save};

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
    let typed = words.next().unwrap_or("").to_ascii_uppercase();
    let name = canonical(&typed).to_string();
    let rest: Vec<&str> = words.collect();
    // )OFF signs off, and )CONTINUE saves the workspace first.
    let off = matches!((name.as_str(), rest.as_slice()), ("OFF" | "CONTINUE", []));
    let lines = match (name.as_str(), rest.as_slice()) {
        ("OFF", []) => Vec::new(),
        ("CONTINUE", []) => save(ws, Some("CONTINUE")),
        ("SAVE", []) => save(ws, None),
        ("SAVE", [id]) => save(ws, Some(id)),
        ("LOAD", _) => return load(ws, &rest),
        ("DROP", _) => drop_workspace(ws, &rest),
        ("LIB", _) => lib(ws, &rest),
        ("COPY" | "PCOPY", _) => return copy(ws, &rest, name == "PCOPY"),
        // The '70-only commands, in (A) only, and the inquiry commands
        // answer for themselves, and None for a name they do not know.
        _ => seventy(&mut ws.saved, ws.mode, &name, &rest)
            .or_else(|| inquiry(ws, &name, &rest))
            .unwrap_or_else(|| vec![workspace_command(&mut ws.saved, &name, &rest)]),
    };
    ending(ws, lines, off)
}

/// What a command hands back once it has run: its own lines, and
/// the sign-off after them when the session is ending. Nothing here
/// feeds lines back -- `)LOAD` and `)COPY` return before this.
///
/// The sign-off is the moment, how long the session was connected
/// and how much processor time it used. APL\360 named the port and
/// the user as well and carried totals to date; sw-apl has no
/// accounts and keeps no such records.
fn ending(ws: &Workspace, mut lines: Vec<String>, off: bool) -> Answer {
    if off {
        let time = (ws.clock)();
        lines.push(moment(ws));
        lines.push(format!("CONNECTED {}", hms(time.now - ws.signed_on)));
        lines.push(format!("CPU TIME {}", hms(time.cpu)));
    }
    Answer {
        lines,
        feed: Vec::new(),
        off,
    }
}

/// The commands whose names are longer than four characters, which
/// are therefore the ones that can be cut short.
pub const ABBREVIATED: [&str; 9] = [
    "CLEAR", "CONTINUE", "DIGITS", "ERASE", "GROUP", "ORIGIN", "PCOPY", "SYMBOLS", "WIDTH",
];

/// The command a typed name means.
///
/// The manual: "Where the first word of a command form is more than
/// four characters long, only the first four are significant. The
/// others are included only for mnemonic reasons, and may be
/// dropped or replaced, as desired. For example, )CLEAR, )CLEA,
/// )CLEAVER, etc., are all equivalent."
///
/// So what follows the fourth character is ignored rather than
/// forgiven: `)CLEAVER` is `)CLEAR`, not a near miss. A name of four
/// characters or fewer has nothing to cut and must be exact, which
/// is why `)VAR` is not `)VARS`. Four characters, not four bytes: a
/// name is whatever followed the parenthesis and may be any text.
#[must_use]
pub fn canonical(name: &str) -> &str {
    if name.chars().count() < 4 {
        return name;
    }
    let first: String = name.chars().take(4).collect();
    let found = ABBREVIATED.iter().find(|long| long.starts_with(&first));
    found.map_or(name, |long| *long)
}

/// A command that changes the workspace itself: the name it answers
/// to, or clearing it altogether. `)WSID name` replies with the name
/// it replaced, as a setting does.
fn workspace_command(saved: &mut Saved, name: &str, rest: &[&str]) -> String {
    let was = match (name, rest) {
        ("CLEAR", []) => {
            *saved = Saved::default();
            return CLEAR.to_string();
        }
        ("WSID", []) => return saved.id.clone().unwrap_or_else(|| CLEAR.to_string()),
        ("WSID", [id]) if valid(id) => saved.id.replace((*id).to_string()),
        _ => return INCORRECT.to_string(),
    };
    format!("WAS {}", was.unwrap_or_else(|| CLEAR.to_string()))
}
