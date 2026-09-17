//! The input lines the session answers itself instead of evaluating:
//! system commands, and the del lines that open and close a function
//! definition.

use apl_parse::parse_header;

use crate::render::error_lines;
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

/// An opening del: read the header and start collecting body lines.
pub fn open_definition(session: &mut Session, header: &str) -> Reply {
    match parse_header(header) {
        Ok(defn) => {
            session.defining = Some(defn);
            Reply::Output(Vec::new())
        }
        Err(err) => Reply::Output(error_lines(&err, &format!("∇{header}"))),
    }
}

/// One line typed in definition mode. A line that is nothing but a
/// del closes the definition and stores the function.
pub fn definition_line(session: &mut Session, line: &str) -> Reply {
    let Some(defn) = &mut session.defining else {
        unreachable!("definition_line only runs while a definition is open")
    };
    if line.trim() == "∇" {
        let defn = session.defining.take().expect("a definition is open");
        session.ws.define(defn);
        return Reply::Output(Vec::new());
    }
    defn.body.push(line.trim_end().to_string());
    Reply::Output(Vec::new())
}
