//! (C)'s commands: opening a function, running the latent expression,
//! and the names the shared commands take.

use apl_ast::Defn;
use apl_c80_define::{Keyed, header};
use apl_eval::{Feature, Workspace, error_lines};
use apl_reply::Reply;
use apl_sysvars::latent;
use apl_value::{AplError, AplResult, ErrorKind};

/// What a command comes to.
#[derive(Debug)]
pub enum Command {
    /// It is answered: show this.
    Done(Reply),
    /// Run this line, as `)EXEC` asks, and show what it shows.
    Run(String),
    /// A command the shared commands run, as this text.
    Shared(String),
}

/// One system command, `name` and the `text` it was typed as. A
/// command ends (C)'s format statement, as APL80's does.
pub fn command(ws: &mut Workspace, keyed: &mut Option<Keyed>, name: &str, text: &str) -> Command {
    ws.format = None;
    let profile = ws.mode.profile();
    if name == "EXEC" && profile.has(Feature::Auto) {
        return latent(ws).map_or_else(|| Command::Done(Reply::default()), Command::Run);
    }
    let Some(opened) = opened(ws, name, text) else {
        return Command::Shared(spelled(ws, name, text));
    };
    Command::Done(match opened {
        Ok(defn) => {
            *keyed = Some(Keyed::open(defn, ws.mode.glyphs()));
            Reply::default()
        }
        Err(err) => Reply::failed(error_lines(&err, &format!("){text}"))),
    })
}

/// `)DEF header`, a new function, or `)EDIT name`, one that exists, in
/// a mode that defines by command; `None` for any other command.
fn opened(ws: &Workspace, name: &str, text: &str) -> Option<AplResult<Defn>> {
    if !ws.mode.profile().has(Feature::DefineByCommand) {
        return None;
    }
    let rest = text
        .trim_start()
        .split_once(' ')
        .map_or("", |(_, r)| r.trim());
    let found = match name {
        "DEF" => header(
            rest,
            ws.mode.glyphs(),
            1 + text.chars().count() - rest.chars().count(),
        ),
        "EDIT" => ws
            .function(&apl_c80_keys::name(rest))
            .map(|d| (*d).clone())
            .ok_or_else(defn),
        _ => return None,
    };
    let taken = |d: &Defn| name == "DEF" && (ws.is_function(&d.name) || ws.get(&d.name).is_some());
    Some(found.and_then(|d| if taken(&d) { Err(defn()) } else { Ok(d) }))
}

/// A command as the shared commands read it: in a mode that types
/// APL80's characters, the names `)ERASE`, `)COPY` and `)PCOPY` take
/// have their `.` and `-` as the glyphs hold them, so `)ERASE M-N`
/// finds `M-N`. Every other command keeps its text as typed.
fn spelled(ws: &Workspace, name: &str, text: &str) -> String {
    let keyed = ws.mode.profile().has(Feature::Apl80Keys);
    if !keyed || !matches!(name, "ERASE" | "COPY" | "PCOPY") {
        return text.to_string();
    }
    apl_c80_keys::name(text)
}

/// The error a name in use, or one with no function, gives.
fn defn() -> AplError {
    AplError::new(ErrorKind::Defn)
}
