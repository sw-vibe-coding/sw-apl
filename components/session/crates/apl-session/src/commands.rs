//! The input lines the session answers itself rather than evaluating:
//! the system commands, whose work is in `apl-commands`, and the
//! lines that drive the del editor.

use apl_commands::system_command;
use apl_editor::Definition;
use apl_eval::error_lines;
use apl_parse::parse_header;
use apl_value::{AplError, ErrorKind};

use crate::reply::Reply;
use crate::session::Session;

/// An opening del: start a new function, or reopen one for editing.
/// A command written on the same line takes effect at once, so
/// `∇NAME[⎕]∇` shows a function and leaves definition mode again.
pub fn open_definition(session: &mut Session, text: &str) -> Reply {
    let at = text.find('[').unwrap_or(text.len());
    let (head, rest) = (text[..at].trim(), text[at..].to_string());
    let opening = match session.ws.function(head) {
        Some(defn) if defn.locked => Err(AplError::new(ErrorKind::Defn)),
        Some(defn) => Ok((*defn).clone()),
        None => parse_header(head).and_then(|defn| {
            let exists = session.ws.is_function(&defn.name) || session.ws.get(&defn.name).is_some();
            if rest.is_empty() && !exists {
                Ok(defn)
            } else {
                Err(AplError::new(ErrorKind::Defn))
            }
        }),
    };
    match opening {
        Err(err) => return Reply::failed(error_lines(&err, &format!("∇{text}"))),
        Ok(defn) => session.defining = Some(Definition::start(defn)),
    }
    if rest.is_empty() {
        return Reply::default();
    }
    definition_line(session, &rest)
}

/// One line typed in definition mode. Closing the definition stores
/// the function, under its new name when `[0]` renamed it.
pub fn definition_line(session: &mut Session, line: &str) -> Reply {
    let Some(definition) = &mut session.defining else {
        unreachable!("definition_line only runs while a definition is open")
    };
    let step = match definition.line(line) {
        Ok(step) => step,
        Err(err) => return Reply::failed(error_lines(&err, line)),
    };
    if let Some(locked) = step.closed {
        let open = session.defining.take().expect("a definition is open");
        let (defn, renamed) = open.close(locked);
        if let Some(was) = renamed {
            session.ws.erase(&was);
        }
        if let Err(err) = session.ws.define(defn) {
            return Reply::failed(error_lines(&err, line));
        }
    }
    Reply::from(step.lines)
}

/// Run one system command. `)LOAD` and `)COPY` hand back lines
/// to run as though they had been typed, which is the only way
/// the `)` commands and the del definitions in a workspace file
/// take effect; what they print on the way is not shown.
pub fn run_command(session: &mut Session, command: &str) -> Reply {
    let answer = system_command(&mut session.ws, command);
    if answer.feed.is_empty() {
        return Reply::from(answer);
    }
    // A workspace put together by running APL is all or nothing: the
    // old one is put aside first, and given back the moment a fed
    // line reports an error. A half-loaded workspace is worse than a
    // refused one, because nothing says which half arrived.
    let was = session.ws.saved.clone();
    for line in &answer.feed {
        let reply = session.respond(line);
        if reply.error {
            session.ws.saved = was;
            return reply;
        }
    }
    Reply::from(answer)
}
