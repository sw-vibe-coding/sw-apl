//! A line typed in (C) before the session evaluates it: one to DOS,
//! one for a function open by command, or a del, which (C) has not got;
//! and the line on the modeled screen.

use apl_c80_define::{Keyed, Typed};
use apl_eval::{Feature, INDENT, Workspace, error_lines};
use apl_reply::Reply;
use apl_value::{AplError, ErrorKind};

use crate::dos::dos;

/// `line`, when (C) answers it before the session would: a line typed
/// to DOS, or while a function is open by command, or a del. `None` for
/// anything else, and for a system command definition mode does not
/// take. Once `)RAM` is open the line is on the screen a peek reads.
pub fn line(ws: &mut Workspace, keyed: &mut Option<Keyed>, line: &str) -> Option<Reply> {
    if ws.dos {
        return Some(dos(ws, line));
    }
    echo(ws, keyed.as_ref(), line);
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

/// `line` on the modeled screen after its prompt, as the TRS-80 shows a
/// line typed, once `)RAM` has opened the memory.
fn echo(ws: &mut Workspace, keyed: Option<&Keyed>, line: &str) {
    if let Some(machine) = ws.machine.as_mut() {
        let prompt = keyed.map_or_else(|| INDENT.to_string(), Keyed::prompt);
        machine.screen.print(&format!("{prompt}{line}\n"));
    }
}
