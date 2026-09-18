//! The transcript lines only the session knows how to make: the error
//! display and the state indicator. What a statement produced is
//! rendered by `apl-console`, below the evaluator, because a read part
//! way through a statement has to render it too.

use apl_eval::{Activation, INDENT};
use apl_value::AplError;

/// APL\360 error display: the error's name, the statement it came
/// from, and a caret under the point of detection. A statement inside
/// a defined function is headed by the function and the line instead
/// of the six-space indent.
pub fn error_lines(err: &AplError, line: &str) -> Vec<String> {
    let caret = err.caret.unwrap_or(0);
    let (head, statement) = match &err.context {
        Some(context) => (
            format!("{}[{}]  ", context.function, context.line),
            context.statement.as_str(),
        ),
        None => (INDENT.to_string(), line),
    };
    let indent = " ".repeat(head.chars().count() + caret);
    vec![
        err.kind.to_string(),
        format!("{head}{statement}"),
        format!("{indent}^"),
    ]
}

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
