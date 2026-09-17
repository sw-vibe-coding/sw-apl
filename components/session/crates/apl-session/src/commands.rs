//! System commands: the lines that start with a right parenthesis.

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
