//! What the session hands back for one input line, and the two kinds
//! of transcript line only it knows how to make: the state indicator
//! and the sign-off. What a statement produced, and how an error
//! reads, are rendered by `apl-console` below the evaluator, because
//! a read part way through a statement has to render them too.

use apl_eval::{Activation, Workspace, hms};

/// The state indicator, innermost first: each function with the line
/// it stopped on, starred when it is the one the user can take up
/// again rather than a caller waiting on it. `verbose` adds the names
/// it made local, which is what `)SIV` shows.
pub fn si_lines(stack: &[Activation], verbose: bool) -> Vec<String> {
    let entries: Vec<(String, &[String])> = stack
        .iter()
        .rev()
        .map(|a| {
            let star = if a.suspended { "*" } else { "" };
            (format!("{}[{}]{star}", a.name, a.line), a.locals.as_slice())
        })
        .collect();
    let column = entries.iter().map(|(e, _)| e.chars().count()).max();
    entries
        .iter()
        .map(|(entry, locals)| match column {
            Some(width) if verbose && !locals.is_empty() => {
                format!("{entry:width$}  {}", locals.join(" "))
            }
            _ => entry.clone(),
        })
        .collect()
}

/// The APL\360 sign-off: the time and date the session ended, then
/// how long it was connected and how much processor time it used.
/// APL\360 also named the port and the user, and carried totals to
/// date; sw-apl has no accounts and keeps no such records.
pub fn sign_off(ws: &Workspace) -> Vec<String> {
    let time = (ws.clock)();
    let date = time.date;
    vec![
        format!(
            "{} {:02}/{:02}/{:02}",
            hms(time.now),
            date / 10_000,
            (date / 100) % 100,
            date % 100
        ),
        format!("CONNECTED {}", hms(time.now - ws.signed_on)),
        format!("CPU TIME {}", hms(time.cpu)),
    ]
}

/// What the shell should do after handing a line to the session:
/// print these lines, which may be none, and then either prompt again
/// or stop.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Reply {
    /// The transcript lines this input produced.
    pub lines: Vec<String>,
    /// Set by `)OFF`: the session has ended.
    pub off: bool,
}

impl Reply {
    /// A reply that ends the session.
    #[must_use]
    pub(crate) fn off(lines: Vec<String>) -> Reply {
        Reply { lines, off: true }
    }
}

impl From<Vec<String>> for Reply {
    fn from(lines: Vec<String>) -> Reply {
        Reply { lines, off: false }
    }
}
