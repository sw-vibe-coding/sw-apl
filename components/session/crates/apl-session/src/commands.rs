//! The input lines the session answers itself rather than evaluating:
//! the system commands, whose work is in `apl-commands`, and the
//! lines that drive the del editor.

use apl_commands::{canonical, directive, system_command};
use apl_editor::Definition;
use apl_eval::error_lines;
use apl_parse::parse_header;
use apl_sysvars::latent;
use apl_value::{AplError, ErrorKind};

use crate::reply::Reply;
use crate::session::Session;

/// An opening del: start a new function, or reopen one for editing.
/// A command written on the same line takes effect at once, so
/// `∇NAME[⎕]∇` shows a function and leaves definition mode again.
///
/// `del` is the glyph that opened it. Del-tilde opens as del does and
/// locks the function when it closes, whichever del closes it; that
/// includes reopening an unlocked function, since reopening is
/// opening. A function already locked cannot be reopened by either.
pub fn open_definition(session: &mut Session, text: &str, del: char) -> Reply {
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
        Err(err) => return Reply::failed(error_lines(&err, &format!("{del}{text}"))),
        Ok(defn) => session.defining = Some(Definition::start(defn, del == '⍫')),
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
        let locked = locked || open.locking;
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

/// Where an input line goes: a system command, a line of an open
/// definition, an opening del, or a statement to evaluate.
///
/// A command comes first even in definition mode. The manual: "A
/// system command entered during function definition will not be
/// accepted as a statement in the definition. Some commands, such as
/// )COPY, will be rejected with the message NOT WITH OPEN
/// DEFINITION; most will be executed immediately." So the question
/// is never whether to take the line as a body line -- it is not one
/// -- but only which commands are refused.
///
/// `None` when the line is a statement, which the session evaluates.
pub fn dispatch(session: &mut Session, line: &str) -> Option<Reply> {
    let trimmed = line.trim();
    if let Some(command) = trimmed.strip_prefix(')') {
        return Some(run_command(session, command));
    }
    if session.defining.is_some() {
        return Some(definition_line(session, line));
    }
    // A body line of an open definition is the function's, whatever
    // it says; outside one, a settings directive takes effect.
    if directive(&mut session.ws.saved, line) {
        return Some(Reply::default());
    }
    let del = trimmed.chars().next().filter(|c| *c == '∇' || *c == '⍫')?;
    Some(open_definition(session, &trimmed[del.len_utf8()..], del))
}

/// The commands an open definition prevents, which are the ones the
/// manual's Table 2.1 gives report 6. Each would store or copy a
/// workspace that is in the middle of being changed.
const NOT_WITH_OPEN: [&str; 4] = ["COPY", "PCOPY", "SAVE", "CONTINUE"];

/// Run one system command. `)LOAD` and `)COPY` hand back lines
/// to run as though they had been typed, which is the only way
/// the `)` commands and the del definitions in a workspace file
/// take effect; what they print on the way is not shown.
///
/// A refused command answers with its report and not an error: so do
/// the other trouble reports, and only an error undoes a `)LOAD`.
///
/// A workspace put together by running APL is all or nothing. The old
/// one is put aside first and given back the moment a fed line
/// reports an error, because a half-loaded workspace is worse than a
/// refused one -- nothing says which half arrived. A definition still
/// open is dropped with it: it belonged to the workspace being
/// replaced, and its lines would otherwise swallow the file's. That
/// is why `)LOAD` needs no report 6, where `)SAVE` and `)COPY` do.
///
/// A workspace loaded in (B) with a latent expression runs it once it
/// is in, and what it shows follows the SAVED line.
pub fn run_command(session: &mut Session, command: &str) -> Reply {
    let typed = command.split_whitespace().next().unwrap_or_default();
    let name = canonical(&typed.to_ascii_uppercase()).to_string();
    if session.defining.is_some() && NOT_WITH_OPEN.contains(&name.as_str()) {
        return Reply::from(vec!["NOT WITH OPEN DEFINITION".to_string()]);
    }
    let answer = system_command(&mut session.ws, command);
    if answer.feed.is_empty() {
        return Reply::from(answer);
    }
    let was = session.ws.saved.clone();
    session.defining = None;
    for line in &answer.feed {
        let reply = session.respond(line);
        if reply.error {
            session.ws.saved = was;
            return reply;
        }
    }
    let mut reply = Reply::from(answer);
    let latent = latent(&session.ws).filter(|_| name == "LOAD");
    let shown = latent.map(|line| session.respond(line).lines);
    reply.lines.extend(shown.unwrap_or_default());
    reply
}
