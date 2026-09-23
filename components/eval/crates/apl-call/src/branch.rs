//! Where a run goes next: the line a branch names, the flag that
//! stops a run from outside it, and what it has printed so far.

use apl_ast::Defn;
use apl_value::{AplError, AplResult, Array, Context, Data, ErrorKind, Number};
use apl_workspace::{Output, Workspace};

/// The line a branch selects: the first element of its value, or
/// `None` when the value is empty, which falls through to the next
/// line. Whether the line exists is the body's business; 0 is the
/// traditional way to write a number that cannot be one.
///
/// # Errors
/// RANK ERROR above rank 1, DOMAIN ERROR for character data and for
/// a number that is not a whole line number.
pub fn target(value: &Array) -> AplResult<Option<i64>> {
    if value.shape.len() > 1 {
        return Err(AplError::new(ErrorKind::Rank));
    }
    let Data::Num(v) = &value.data else {
        return Err(AplError::new(ErrorKind::Domain));
    };
    match v.first() {
        None => Ok(None),
        Some(Number::Int(n)) => Ok(Some(*n)),
        Some(Number::Float(_)) => Err(AplError::new(ErrorKind::Domain)),
    }
}

/// Whether this session was asked to stop since this was last
/// called, which answers it.
///
/// Read between the lines of a body, where a real read every time
/// costs nothing. The primitives read it too, strided, inside their
/// own loops -- so a single long statement stops as well as a loop of
/// lines does. The flag is this thread's session's: see `apl-attn`.
#[must_use]
pub fn interrupted() -> bool {
    apl_attn::asked()
}

/// Stop activation `at` on the line that failed, and, for the
/// function the error came from, record that line as the error's own.
/// Which activation is the suspended one is settled when the error
/// reaches the terminal, since a call inside an expression unwinds on
/// the way out and may take the innermost one with it.
pub fn halt(
    ws: &mut Workspace,
    mut err: AplError,
    defn: &Defn,
    at: usize,
    line: usize,
) -> AplError {
    if err.context.is_none() {
        err.context = Some(Context {
            function: defn.name.clone(),
            line,
            statement: defn.body[line - 1].clone(),
        });
    }
    ws.stop(at, line, false);
    err
}

/// Offer what the run has printed to a console that shows as it goes,
/// while the statement is still running: a 2741 printed as the
/// carriage moved. A console that keeps a transcript declines, and the
/// lines wait, finished, for the next read or the statement's end.
///
/// A line `⍞←` left open waits for what finishes it, so a prompt and
/// its answer still share a line.
pub fn emit(ws: &mut Workspace) {
    if matches!(ws.output.last(), Some(Output::Bare(_)) | None) {
        return;
    }
    let shown = ws.flush();
    if !shown.lines.is_empty() && !ws.console.show(&shown.lines) {
        ws.output.push(Output::Lines(shown.lines));
    }
}
