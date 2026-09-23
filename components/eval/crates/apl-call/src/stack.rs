//! The activation stack as the user meets it: entering a call,
//! settling which function a failure suspended, clearing the top
//! entry, and taking a suspended function up again.

use apl_ast::Defn;
use apl_value::{AplError, AplResult, Array, ErrorKind, Number};
use apl_workspace::{Output, Workspace, render};

use crate::call::run_body;
use apl_scan::labels;
use apl_workspace::Run;

/// Shadow everything the call makes local -- result, arguments,
/// locals, labels -- then give the arguments and the labels their
/// values. A label holds the number of the line it names.
///
/// # Errors
/// DEPTH ERROR when calls nest too deeply, WS FULL when the
/// arguments and labels will not fit in what the quota leaves.
pub fn bind(
    ws: &mut Workspace,
    defn: &Defn,
    args: (Option<Array>, Option<Array>),
) -> AplResult<usize> {
    let labels = labels(&defn.body);
    let mut names = defn.names();
    names.extend(labels.iter().map(|(n, _)| n.clone()));
    let at = ws.enter(&defn.name, &names)?;
    for (n, v) in [(&defn.left, args.0), (&defn.right, args.1)] {
        if let (Some(n), Some(v)) = (n, v) {
            ws.set(n, v)?;
        }
    }
    for (n, line) in labels {
        let line = i64::try_from(line).unwrap_or_default();
        ws.set(&n, Array::scalar(Number::Int(line)))?;
    }
    Ok(at)
}

/// Mark the innermost activation as the suspended one. The error has
/// reached the terminal, so this is the function the user can look
/// into and take up again; the ones outside it are pendent, waiting
/// on a call that stopped.
pub fn suspend(ws: &mut Workspace) {
    if let Some(at) = ws.si().len().checked_sub(1) {
        let line = ws.si()[at].line;
        ws.stop(at, line, true);
    }
}

/// Clear the top of the state indicator: the suspended function, and
/// the callers that were waiting on it, which have nothing to go back
/// to. A second suspension underneath is left alone.
pub fn clear(ws: &mut Workspace) {
    if ws.si().is_empty() {
        return;
    }
    ws.leave();
    while ws.si().last().is_some_and(|a| !a.suspended) {
        ws.leave();
    }
}

/// Take the suspended function up again at `from`. When it returns,
/// the caller waiting on it called it as a whole statement, so it
/// simply carries on at its next line with whatever came back.
///
/// # Errors
/// SYNTAX ERROR when nothing is suspended or the line is not one;
/// anything the resumed lines raise, which suspends again.
pub fn resume(ws: &mut Workspace, from: i64, run: Run) -> AplResult<Output> {
    let syntax = || AplError::new(ErrorKind::Syntax);
    let mut line = usize::try_from(from).map_err(|_| syntax())?;
    loop {
        let at = ws.si().len().checked_sub(1).ok_or_else(syntax)?;
        let name = ws.si()[at].name.clone();
        let defn = ws
            .function(&name)
            .ok_or_else(|| AplError::new(ErrorKind::Value))?;
        ws.stop(at, line, false);
        run_body(ws, &defn, run, at, line)?;
        let result = defn.result.as_ref().and_then(|r| ws.get(r).cloned());
        ws.leave();
        let Some(caller) = ws.si().last() else {
            return Ok(result.map_or(Output::Nothing, Output::Value));
        };
        line = caller.line + 1;
        if let Some(value) = result {
            let lines = render(&Output::Value(value), ws.saved.print);
            ws.output.push(Output::Lines(lines));
        }
    }
}
