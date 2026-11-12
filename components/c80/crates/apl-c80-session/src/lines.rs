//! A line typed in (C) before the session evaluates it: one for a
//! function open by command, or a del, which (C) has not got.

use apl_c80_define::{Keyed, Typed};
use apl_eval::{Feature, Workspace, error_lines};
use apl_reply::Reply;
use apl_value::{AplError, ErrorKind};

/// `line`, when (C) answers it before the session would: a line typed
/// while a function is open by command, or a del. `None` for anything
/// else, and for a system command definition mode does not take.
pub fn line(ws: &mut Workspace, keyed: &mut Option<Keyed>, line: &str) -> Option<Reply> {
    let Some(open) = keyed.as_mut() else {
        return refused(ws, line);
    };
    let typed = open.line(line);
    let fail = |err: AplError| Reply::failed(error_lines(&err, line));
    match typed {
        Ok(Typed::Stay(lines)) => Some(Reply::from(lines)),
        Ok(Typed::Command) => None,
        Err(err) => Some(fail(err)),
        Ok(Typed::Leave) => {
            let defn = keyed.take()?.close();
            Some(ws.define(defn).map_or_else(fail, |()| Reply::default()))
        }
    }
}

/// A del typed in a mode that defines by command, which has none: a
/// CHARACTER ERROR, as for any character it has not got. `None` in any
/// other mode, or for any other line.
fn refused(ws: &Workspace, line: &str) -> Option<Reply> {
    let typed = line.trim_start();
    let del = typed.chars().next().filter(|c| *c == '∇' || *c == '⍫')?;
    if !ws.mode.profile().has(Feature::DefineByCommand) {
        return None;
    }
    let at = line[..line.len() - typed.len()].chars().count();
    let err = AplError::new(ErrorKind::Character(del)).at(at);
    Some(Reply::failed(error_lines(&err, line)))
}
