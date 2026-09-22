//! Which quad names are functions, and applying one.

use apl_console_control::{control, national};
use apl_fix::{canonical, fix, rows};
use apl_value::{AplError, AplResult, Array, ErrorKind, Number};
use apl_workspace::Workspace;

use crate::list::name_list;
use crate::names::{class, expunge};

/// The system functions (B) has. `⎕DL`, a function in APLSV, the 5110
/// keeps as a variable, and so does sw-apl.
const FUNCTIONS: [&str; 6] = ["⎕CC", "⎕CR", "⎕EX", "⎕FX", "⎕NC", "⎕NL"];

/// True when `name` is a system function, so that the parser reads
/// `⎕NC 'A'` as a call rather than a name beside an array.
#[must_use]
pub fn is_function(name: &str) -> bool {
    FUNCTIONS.contains(&name)
}

/// `name r`.
///
/// # Errors
/// SYNTAX ERROR for a function that takes a left argument only, or is
/// not one; otherwise the function's own.
pub fn monadic(ws: &mut Workspace, name: &str, r: &Array) -> AplResult<Array> {
    match name {
        "⎕CC" => national(r),
        "⎕CR" => canonical(ws, r),
        "⎕EX" => each(ws, r, |ws, n| i64::from(expunge(ws, n))),
        "⎕FX" => fix(ws, r),
        "⎕NC" => each(ws, r, |ws, n| class(ws, n)),
        "⎕NL" => name_list(ws, None, r),
        _ => Err(AplError::new(ErrorKind::Syntax)),
    }
}

/// `l name r`.
///
/// # Errors
/// SYNTAX ERROR for a function that takes no left argument, or is
/// not one; otherwise the function's own.
pub fn dyadic(ws: &mut Workspace, name: &str, l: &Array, r: &Array) -> AplResult<Array> {
    match name {
        "⎕CC" => control(l, r),
        "⎕NL" => name_list(ws, Some(l), r),
        _ => Err(AplError::new(ErrorKind::Syntax)),
    }
}

/// `f` applied to each name `r` spells: a number for one name, a
/// vector for a matrix of them.
fn each(
    ws: &mut Workspace,
    r: &Array,
    f: impl Fn(&mut Workspace, &str) -> i64,
) -> AplResult<Array> {
    let got: Vec<Number> = rows(r)?.iter().map(|n| Number::Int(f(ws, n))).collect();
    Ok(match (r.shape.len(), got.as_slice()) {
        (0 | 1, [one]) => Array::scalar(*one),
        _ => Array::vector(got),
    })
}
