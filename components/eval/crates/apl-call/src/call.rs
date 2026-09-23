//! Applying a defined function, and running its body.

use apl_ast::Defn;
use apl_scan::without_label;
use apl_value::{AplError, AplResult, Array, ErrorKind};
use apl_workspace::{Output, Workspace, render};

use crate::branch::{emit, halt, interrupted};
use crate::stack::bind;

/// How to evaluate one body line. The evaluator passes its own
/// `eval_line`; nothing here needs to know what that does.
pub type Run = fn(&mut Workspace, &str) -> AplResult<Output>;

/// Apply the function `name` holds. `None` comes back when the header
/// declares no result, which only a whole statement may ignore.
///
/// A body that fails does not unwind: the activation stays on the
/// stack, so its locals stay visible and `)SI` can report it.
///
/// # Errors
/// SYNTAX ERROR when the valence written is not the one declared,
/// DEPTH ERROR when calls nest too deeply, and anything the body
/// raises, carrying the line it came from.
pub fn call(
    ws: &mut Workspace,
    name: &str,
    left: Option<Array>,
    right: Option<Array>,
    run: Run,
) -> AplResult<Option<Array>> {
    let defn = ws
        .function(name)
        .ok_or_else(|| AplError::new(ErrorKind::Value))?;
    if left.is_some() != defn.left.is_some() || right.is_some() != defn.right.is_some() {
        return Err(AplError::new(ErrorKind::Syntax));
    }
    let at = bind(ws, &defn, (left, right))?;
    run_body(ws, &defn, run, at, 1)?;
    let result = defn.result.as_ref().and_then(|r| ws.get(r).cloned());
    ws.leave();
    Ok(result)
}

/// A call used for its value. Such a call is part of a larger
/// expression, which sw-apl cannot take up again, so a failure
/// unwinds it rather than leaving it suspended.
///
/// # Errors
/// VALUE ERROR when the function declares no result; otherwise
/// whatever the call raises, with `pos` as the caret.
pub fn value(
    ws: &mut Workspace,
    name: &str,
    pos: usize,
    args: (Option<Array>, Option<Array>),
    run: Run,
) -> AplResult<Array> {
    let depth = ws.si().len();
    call(ws, name, args.0, args.1, run)
        .inspect_err(|_| {
            while ws.si().len() > depth {
                ws.leave();
            }
        })
        .map_err(|e| e.at(pos))?
        .ok_or_else(|| AplError::new(ErrorKind::Value).at(pos))
}

/// Run the body of `defn` from line `from`, following its branches,
/// for the activation at `at` on the stack. The line is recorded as
/// it runs, not only when it fails: it is what `)SI` reports and what
/// `⌶26` and `⌶27` read. What a line displays joins
/// the pending output, so it reaches the terminal ahead of the result.
///
/// # Errors
/// Whatever a line raises, after stopping the activation on it, and
/// INTERRUPT when a stop was asked for between two of them.
pub fn run_body(
    ws: &mut Workspace,
    defn: &Defn,
    run: Run,
    at: usize,
    from: usize,
) -> AplResult<()> {
    let mut line = from;
    while let Some(text) = defn.body.get(line - 1) {
        ws.stop(at, line, false);
        let stopped = interrupted().then(|| AplError::new(ErrorKind::Interrupt));
        let shown = stopped.map_or_else(|| run(ws, &without_label(text)), Err);
        let shown = shown.map_err(|e| halt(ws, e, defn, at, line))?;
        let Some(next) = advance(ws, shown, line, defn.body.len()) else {
            return Ok(());
        };
        line = next;
    }
    Ok(())
}

/// Where the line counter goes after a line has run: a branch moves
/// it, and anything else is shown and followed by the next line.
/// `None` returns from the body, which is what a branch to a line the
/// function does not have means.
fn advance(ws: &mut Workspace, shown: Output, line: usize, last: usize) -> Option<usize> {
    match shown {
        Output::Branch(Some(to)) => match usize::try_from(to) {
            Ok(n) if (1..=last).contains(&n) => Some(n),
            _ => None,
        },
        Output::Nothing | Output::Branch(None) => Some(line + 1),
        shown => {
            ws.output
                .push(Output::Lines(render(&shown, ws.saved.print)));
            emit(ws);
            Some(line + 1)
        }
    }
}
