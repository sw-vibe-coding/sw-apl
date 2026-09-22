//! Reading a system variable, and where the evaluator hands one over.

use apl_ast::Expr;
use apl_modes::Mode;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, FUZZ, Number};
use apl_workspace::{Workspace, free, used};

use crate::assign::assign;
use crate::av::atomic_vector;

/// What `⎕AI` and `⎕TS` hold until one is assigned: the 5110 has one
/// user and no clock, and keeps them for compatibility with APLSV.
/// The manual gives the time stamp; the accounting information is
/// APLSV's four numbers, all zero.
const ACCOUNT: [i64; 4] = [0, 0, 0, 0];
const STAMP: [i64; 7] = [1900, 0, 0, 0, 0, 0, 0];

/// The expression `expr` when it reads or assigns a system variable:
/// what it gives, with the caret on the name for an error of its own.
/// `eval` evaluates what is assigned. `None` for any other expression.
///
/// Indexed assignment into one is not implemented: NONCE ERROR.
pub fn form(
    ws: &mut Workspace,
    expr: &Expr,
    eval: fn(&mut Workspace, &Expr) -> AplResult<Array>,
) -> Option<AplResult<Array>> {
    let system = |name: &str| name.starts_with('⎕');
    Some(match expr {
        Expr::Name(name, pos) if system(name) => read(ws, name).map_err(|e| e.at(*pos)),
        Expr::Assign { name, pos, value } if system(name) => eval(ws, value).and_then(|v| {
            assign(ws, name, &v).map_err(|e| e.at(*pos))?;
            Ok(v)
        }),
        Expr::IndexedAssign { name, pos, .. } if system(name) => {
            Err(AplError::new(ErrorKind::Nonce).at(*pos))
        }
        _ => return None,
    })
}

/// The value of the system variable `name`.
///
/// # Errors
/// SYNTAX ERROR for a name the system does not have, as for any
/// name that cannot stand where it is written.
pub fn read(ws: &Workspace, name: &str) -> AplResult<Array> {
    if let Some(value) = scalar(ws, name) {
        return Ok(Array::scalar(value));
    }
    let ints = |v: &[i64]| Array::vector(v.iter().map(|n| Number::Int(*n)).collect());
    let fixed = |v: &[i64]| ws.compatible.get(name).cloned().unwrap_or_else(|| ints(v));
    let lines = ws.si().iter().rev().map(|a| a.line.try_into().unwrap_or(0));
    let empty = || Array {
        shape: vec![0],
        data: Data::Char(Vec::new()),
    };
    Ok(match name {
        "⎕LC" => ints(&lines.collect::<Vec<i64>>()),
        "⎕LX" => ws.saved.latent.clone().unwrap_or_else(empty),
        "⎕AV" => atomic_vector(),
        "⎕AI" => fixed(&ACCOUNT),
        "⎕TS" => fixed(&STAMP),
        _ => return Err(AplError::new(ErrorKind::Syntax)),
    })
}

/// The ones that are one number: the comparison tolerance, index
/// origin, printing precision and width, random link, the space the
/// workspace has left, and the terminal type, user load and delay,
/// which the 5110 fixes. `⎕DL` is a delay function in APLSV; the 5110
/// keeps it as a variable holding 0, and so does sw-apl. `None` for anything else.
fn scalar(ws: &Workspace, name: &str) -> Option<Number> {
    let int = |n: usize| i64::try_from(n).unwrap_or(i64::MAX);
    let held = || used(&ws.saved.vars, &ws.saved.funcs, &ws.saved.groups);
    Some(Number::Int(match name {
        "⎕CT" => return Some(Number::Float(FUZZ)),
        "⎕IO" => ws.saved.env.io,
        "⎕PP" => int(ws.saved.print.digits),
        "⎕PW" => int(ws.saved.print.width),
        "⎕RL" => i64::try_from(ws.saved.env.link).unwrap_or(i64::MAX),
        "⎕WA" => int(free(ws.quota, held())),
        "⎕TT" | "⎕DL" => 0,
        "⎕UL" => 1,
        _ => return None,
    }))
}

/// The line that runs a loaded workspace's latent expression, when it
/// has one and the session is in (B): `⍎⎕LX`, which is what the 5110
/// runs, so an error in it is reported against that line.
#[must_use]
pub fn latent(ws: &Workspace) -> Option<&'static str> {
    (ws.mode == Mode::B && ws.saved.latent.is_some()).then_some("⍎⎕LX")
}
