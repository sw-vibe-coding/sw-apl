//! A system command typed in reply to quad input. The manuals,
//! APL\360's and the 5110's alike: it is executed, and the request for
//! input is made again -- unless it replaces the active workspace.

use apl_commands::canonical;
use apl_eval::Workspace;

use crate::commands::run_command;
use crate::session::Session;
use apl_reply::Reply;

/// The commands that give up the read: two replace the workspace the
/// read belongs to, and two end the session it is part of.
const ABANDONING: [&str; 4] = ["LOAD", "CLEAR", "OFF", "CONTINUE"];

impl Default for Session {
    fn default() -> Self {
        let ws = Workspace {
            command: Some(at_quad),
            ..Workspace::default()
        };
        Session { ws, defining: None }
    }
}

/// Run a command typed at the quad prompt, in the middle of the
/// statement that asked, and give back what it showed. A command
/// that abandons the read is left for the session instead, since
/// the statement it would replace is still running.
fn at_quad(ws: &mut Workspace, command: &str) -> Option<Vec<String>> {
    let typed = command.split_whitespace().next().unwrap_or_default();
    if ABANDONING.contains(&canonical(&typed.to_ascii_uppercase())) {
        ws.abandoned = Some(command.to_string());
        return None;
    }
    let mut session = Session {
        ws: std::mem::take(ws),
        defining: None,
    };
    let reply = run_command(&mut session, command);
    *ws = session.ws;
    Some(reply.lines)
}

/// Run the command that abandoned a read, once the statement has
/// unwound: nothing it was part of waits on the state indicator, and
/// what it showed before the command comes first.
pub fn abandon(session: &mut Session, command: &str) -> Reply {
    while !session.ws.si().is_empty() {
        session.ws.leave();
    }
    let mut lines = session.ws.console.take();
    lines.extend(session.ws.flush().lines);
    let mut reply = run_command(session, command);
    lines.append(&mut reply.lines);
    reply.lines = lines;
    reply
}
