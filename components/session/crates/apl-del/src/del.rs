//! Opening a definition, and a line typed into one.

use apl_editor::Definition;
use apl_eval::{Workspace, error_lines};
use apl_parse::parse_header;
use apl_reply::Reply;
use apl_value::{AplError, ErrorKind};

/// An opening del: start a new function, or reopen one for editing.
/// A command written on the same line takes effect at once, so
/// `∇NAME[⎕]∇` shows a function and leaves definition mode again.
///
/// `del` is the glyph that opened it. Del-tilde opens as del does and
/// locks the function when it closes, whichever del closes it; that
/// includes reopening an unlocked function, since reopening is
/// opening. A function already locked cannot be reopened by either.
pub fn open(ws: &mut Workspace, defining: &mut Option<Definition>, text: &str, del: char) -> Reply {
    let at = text.find('[').unwrap_or(text.len());
    let (head, rest) = (text[..at].trim(), text[at..].to_string());
    let glyphs = ws.mode.glyphs();
    let opening = match ws.function(head) {
        Some(defn) if defn.locked => Err(AplError::new(ErrorKind::Defn)),
        Some(defn) => Ok((*defn).clone()),
        None => parse_header(head, glyphs).and_then(|defn| {
            let exists = ws.is_function(&defn.name) || ws.get(&defn.name).is_some();
            let fresh = rest.is_empty() && !exists;
            fresh
                .then_some(defn)
                .ok_or_else(|| AplError::new(ErrorKind::Defn))
        }),
    };
    match opening {
        Err(err) => return Reply::failed(error_lines(&err, &format!("{del}{text}"))),
        Ok(defn) => *defining = Some(Definition::start(defn, del == '⍫', glyphs)),
    }
    if rest.is_empty() {
        return Reply::default();
    }
    line(ws, defining, &rest)
}

/// One line typed in definition mode. Closing the definition stores
/// the function, under its new name when `[0]` renamed it. With no
/// definition open there is nothing to type into, and nothing shown.
pub fn line(ws: &mut Workspace, defining: &mut Option<Definition>, line: &str) -> Reply {
    let Some(definition) = defining.as_mut() else {
        return Reply::default();
    };
    let step = match definition.line(line) {
        Ok(step) => step,
        Err(err) => return Reply::failed(error_lines(&err, line)),
    };
    if let Some(locked) = step.closed
        && let Some(open) = defining.take()
    {
        let locked = locked || open.locking;
        let (defn, renamed) = open.close(locked);
        if let Some(was) = renamed {
            ws.erase(&was);
        }
        if let Err(err) = ws.define(defn) {
            return Reply::failed(error_lines(&err, line));
        }
    }
    Reply::from(step.lines)
}
