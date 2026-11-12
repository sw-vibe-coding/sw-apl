//! Definition by command, in a mode that has it: (C)'s `)DEF` and
//! `)EDIT` open a function, the lines of an open one go to it, and
//! `)EXEC` runs the latent expression. Such a mode has no del
//! (owner, 2026-11-11; docs/mode-c.md).

use apl_c80_define::{Keyed, Typed, header};
use apl_eval::{Feature, error_lines};

use apl_sysvars::latent;
use apl_value::{AplError, ErrorKind};

use crate::session::Session;
use apl_reply::Reply;

/// `)DEF header`, `)EDIT name` or `)EXEC`, where the mode defines by
/// command; `None` for any other command, or in another mode.
pub fn command(session: &mut Session, name: &str, text: &str) -> Option<Reply> {
    let profile = session.ws.mode.profile();
    let rest = text
        .trim_start()
        .split_once(' ')
        .map_or("", |(_, r)| r.trim());
    let glyphs = session.ws.mode.glyphs();
    let opened = match name {
        "DEF" if profile.has(Feature::DefineByCommand) => {
            let at = 1 + text.chars().count() - rest.chars().count();
            header(rest, glyphs, at).and_then(|d| {
                let taken = session.ws.is_function(&d.name) || session.ws.get(&d.name).is_some();
                (!taken)
                    .then_some(d)
                    .ok_or_else(|| AplError::new(ErrorKind::Defn))
            })
        }
        "EDIT" if profile.has(Feature::DefineByCommand) => session
            .ws
            .function(&apl_c80_keys::name(rest))
            .map(|d| (*d).clone())
            .ok_or_else(|| AplError::new(ErrorKind::Defn)),
        "EXEC" if profile.has(Feature::Auto) => {
            let line = latent(&session.ws);
            return Some(line.map_or_else(Reply::default, |l| session.respond(&l)));
        }
        _ => return None,
    };
    Some(match opened {
        Ok(defn) => {
            session.keyed = Some(Keyed::open(defn, glyphs));
            Reply::default()
        }
        Err(err) => Reply::failed(error_lines(&err, &format!("){text}"))),
    })
}

/// One line typed while a function is open by command. `None` when
/// none is, or the line is a system command definition mode does not
/// take, for the session to run.
pub fn line(session: &mut Session, line: &str) -> Option<Reply> {
    let keyed = session.keyed.as_mut()?;
    match keyed.line(line) {
        Ok(Typed::Stay(lines)) => Some(Reply::from(lines)),
        Ok(Typed::Command) => None,
        Err(err) => Some(Reply::failed(error_lines(&err, line))),
        Ok(Typed::Leave) => {
            let defn = session.keyed.take()?.close();
            let stored = session.ws.define(defn);
            Some(stored.map_or_else(
                |e| Reply::failed(error_lines(&e, line)),
                |()| Reply::default(),
            ))
        }
    }
}

/// A del typed in a mode that defines by command, which has none: a
/// CHARACTER ERROR, as for any character it has not got. `None` in any
/// other mode, or for any other line.
pub fn no_del(session: &Session, line: &str) -> Option<Reply> {
    let del = line
        .trim_start()
        .chars()
        .next()
        .filter(|c| *c == '∇' || *c == '⍫')?;
    if !session.ws.mode.profile().has(Feature::DefineByCommand) {
        return None;
    }
    let at = line.len() - line.trim_start().len();
    let err = AplError::new(ErrorKind::Character(del)).at(line[..at].chars().count());
    Some(Reply::failed(error_lines(&err, line)))
}

/// A command as the shared commands read it: in a mode that types
/// APL80's characters, the names `)ERASE`, `)COPY` and `)PCOPY` take have
/// their `.` and `-` as the glyphs hold them, so `)ERASE M-N` finds
/// `M-N`. Every other command keeps its text as typed.
#[must_use]
pub fn spelled(session: &Session, name: &str, command: &str) -> String {
    let keyed = session.ws.mode.profile().has(Feature::Apl80Keys);
    if !keyed || !matches!(name, "ERASE" | "COPY" | "PCOPY") {
        return command.to_string();
    }
    apl_c80_keys::name(command)
}
