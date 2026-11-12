//! The input lines the session answers itself rather than evaluating:
//! the system commands, whose work is in `apl-commands`, and the
//! lines that drive the del editor (`apl-del`) or, in (C), a function
//! open by command (`apl-c80-session`).

use apl_c80_session::Command;
use apl_commands::{Answer, canonical, directive, system_command};
use apl_sysvars::latent;

use crate::session::Session;
use apl_reply::Reply;

/// Where an input line goes: a system command, a line of an open
/// definition, an opening del, or a statement to evaluate.
///
/// A command comes first even in definition mode. The manual: "A
/// system command entered during function definition will not be
/// accepted as a statement in the definition. Some commands, such as
/// )COPY, will be rejected with the message NOT WITH OPEN
/// DEFINITION; most will be executed immediately." So the question
/// is never whether to take the line as a body line -- it is not one
/// -- but only which commands are refused. A line edit's marks and fill
/// are positions on the line, and the manual's own fill begins with
/// `)`: no line of one is a command. A body line of an open definition
/// is the function's, whatever it says; outside one, a settings
/// directive takes effect.
///
/// `None` when the line is a statement, which the session evaluates.
pub fn dispatch(session: &mut Session, line: &str) -> Option<Reply> {
    let trimmed = line.trim();
    if session.defining.as_ref().is_some_and(|d| d.patch.is_some()) {
        return Some(apl_del::line(&mut session.ws, &mut session.defining, line));
    }
    if let Some(reply) = apl_c80_session::line(&mut session.ws, &mut session.keyed, line) {
        return Some(reply);
    }
    if let Some(command) = trimmed.strip_prefix(')') {
        return Some(run_command(session, command));
    }
    if session.defining.is_some() {
        return Some(apl_del::line(&mut session.ws, &mut session.defining, line));
    }
    if directive(&mut session.ws.saved, line) {
        return Some(Reply::default());
    }
    let del = trimmed.chars().next().filter(|c| *c == '∇' || *c == '⍫')?;
    Some(apl_del::open(
        &mut session.ws,
        &mut session.defining,
        &trimmed[del.len_utf8()..],
        del,
    ))
}

/// The commands an open definition prevents, which are the ones the
/// manual's Table 2.1 gives report 6. Each would store or copy a
/// workspace that is in the middle of being changed.
const NOT_WITH_OPEN: [&str; 4] = ["COPY", "PCOPY", "SAVE", "CONTINUE"];

/// Run one system command. `)LOAD` and `)COPY` hand back lines
/// to run as though they had been typed, which is the only way
/// the `)` commands and the definitions in a workspace file
/// take effect; what they print on the way is not shown (`fed`).
///
/// A refused command answers with its report and not an error: so do
/// the other trouble reports, and only an error undoes a `)LOAD`. In
/// (C) a command may be one of its own, answered or run there.
pub fn run_command(session: &mut Session, command: &str) -> Reply {
    let typed = command.split_whitespace().next().unwrap_or_default();
    let name = canonical(&typed.to_ascii_uppercase()).to_string();
    let open = session.defining.is_some() || session.keyed.is_some();
    if open && NOT_WITH_OPEN.contains(&name.as_str()) {
        return Reply::from(vec!["NOT WITH OPEN DEFINITION".to_string()]);
    }
    let shared = match apl_c80_session::command(&mut session.ws, &mut session.keyed, &name, command)
    {
        Command::Done(reply) => return reply,
        Command::Run(line) => return session.respond(&line),
        Command::Shared(text) => text,
    };
    let answer = system_command(&mut session.ws, &shared);
    if answer.feed.is_empty() {
        return Reply::from(answer);
    }
    fed(session, answer, name == "LOAD")
}

/// A command's lines run as though typed, its own lines after.
///
/// A workspace put together by running APL is all or nothing. The old
/// one is put aside first and given back the moment a fed line
/// reports an error, because a half-loaded workspace is worse than a
/// refused one -- nothing says which half arrived. A definition still
/// open is dropped with it: it belonged to the workspace being
/// replaced, and its lines would otherwise swallow the file's. That
/// is why `)LOAD` needs no report 6, where `)SAVE` and `)COPY` do.
///
/// A workspace loaded in a mode that runs a latent expression runs it
/// once it is in, and what it shows follows the SAVED line.
fn fed(session: &mut Session, answer: Answer, loaded: bool) -> Reply {
    let was = session.ws.saved.clone();
    (session.defining, session.keyed) = (None, None);
    let failed = answer
        .feed
        .iter()
        .map(|l| session.respond(l))
        .find(|r| r.error);
    if let Some(reply) = failed {
        session.ws.saved = was;
        return reply;
    }
    let mut reply = Reply::from(answer);
    let latent = latent(&session.ws).filter(|_| loaded);
    reply.lines.extend(
        latent
            .map(|line| session.respond(&line).lines)
            .unwrap_or_default(),
    );
    reply
}
