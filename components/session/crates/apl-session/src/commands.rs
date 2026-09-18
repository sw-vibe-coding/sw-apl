//! The input lines the session answers itself instead of evaluating:
//! system commands, and the lines that drive the del editor.

use apl_editor::Definition;
use apl_parse::parse_header;
use apl_value::{AplError, ErrorKind};

use crate::render::{error_lines, si_lines};
use crate::session::{Reply, Session};

/// Run one system command (the text after the parenthesis).
pub fn system_command(session: &mut Session, command: &str) -> Reply {
    let mut words = command.split_whitespace();
    let name = words.next().unwrap_or("").to_ascii_uppercase();
    let rest: Vec<&str> = words.collect();
    let number = match rest.as_slice() {
        [] => None,
        [one] => one.parse::<usize>().ok(),
        _ => Some(usize::MAX),
    };
    let reply = match (name.as_str(), number) {
        ("OFF", None) if rest.is_empty() => return Reply::Off,
        ("SI" | "SIV", None) if rest.is_empty() => {
            return Reply::Output(si_lines(session.ws.si(), name == "SIV"));
        }
        ("ORIGIN", Some(n @ (0 | 1))) => {
            let new = i64::try_from(n).unwrap_or(1);
            was_line(std::mem::replace(&mut session.ws.env.io, new))
        }
        ("DIGITS", Some(n @ 1..=16)) => was_line(std::mem::replace(&mut session.digits, n)),
        ("WIDTH", Some(n @ 30..=254)) => was_line(std::mem::replace(&mut session.width, n)),
        _ => "INCORRECT COMMAND".to_string(),
    };
    Reply::Output(vec![reply])
}

/// The APL\360 reply to a settings command: the previous value.
fn was_line<T: std::fmt::Display>(was: T) -> String {
    format!("WAS {was}")
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
        Err(err) => return Reply::Output(error_lines(&err, &format!("∇{text}"))),
        Ok(defn) => session.defining = Some(Definition::start(defn)),
    }
    if rest.is_empty() {
        return Reply::Output(Vec::new());
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
        Err(err) => return Reply::Output(error_lines(&err, line)),
    };
    if let Some(locked) = step.closed {
        let open = session.defining.take().expect("a definition is open");
        let (defn, renamed) = open.close(locked);
        if let Some(was) = renamed {
            session.ws.erase(&was);
        }
        session.ws.define(defn);
    }
    Reply::Output(step.lines)
}
