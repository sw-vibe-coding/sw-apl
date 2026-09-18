//! The input lines the session answers itself instead of evaluating:
//! system commands, and the lines that drive the del editor.

use apl_editor::Definition;
use apl_eval::{Saved, error_lines};
use apl_parse::parse_header;
use apl_value::{AplError, ErrorKind};

use crate::reply::{Reply, si_lines, sign_off};

/// What an unnamed workspace is called, as APL\360 named it.
pub const CLEAR: &str = "CLEAR WS";
use crate::session::Session;

/// Run one system command (the text after the parenthesis).
pub fn system_command(session: &mut Session, command: &str) -> Reply {
    let mut words = command.split_whitespace();
    let name = words.next().unwrap_or("").to_ascii_uppercase();
    let rest: Vec<&str> = words.collect();
    match (name.as_str(), rest.as_slice()) {
        ("OFF", []) => return Reply::off(sign_off(&session.ws)),
        ("SI" | "SIV", []) => {
            return Reply::from(si_lines(session.ws.si(), name == "SIV"));
        }
        _ => {}
    }
    let reply = workspace_command(&mut session.ws.saved, &name, &rest);
    Reply::from(vec![
        reply.unwrap_or_else(|| "INCORRECT COMMAND".to_string()),
    ])
}

/// A command that changes the workspace: its settings, the name it
/// answers to, or clearing it altogether. A setting replies with the
/// value it replaced, as APL\360 did. `None` when the name or the
/// argument is not one of them.
fn workspace_command(saved: &mut Saved, name: &str, rest: &[&str]) -> Option<String> {
    let number = rest.first().and_then(|v| v.parse::<usize>().ok());
    let was = match (name, rest, number) {
        ("CLEAR", [], _) => {
            *saved = Saved::default();
            return Some(CLEAR.to_string());
        }
        ("WSID", [], _) => return Some(saved.id.clone().unwrap_or_else(|| CLEAR.to_string())),
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
        _ => return None,
    };
    Some(format!("WAS {}", was.unwrap_or_else(|| CLEAR.to_string())))
}

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
        Err(err) => return Reply::from(error_lines(&err, &format!("∇{text}"))),
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
        Err(err) => return Reply::from(error_lines(&err, line)),
    };
    if let Some(locked) = step.closed {
        let open = session.defining.take().expect("a definition is open");
        let (defn, renamed) = open.close(locked);
        if let Some(was) = renamed {
            session.ws.erase(&was);
        }
        session.ws.define(defn);
    }
    Reply::from(step.lines)
}
