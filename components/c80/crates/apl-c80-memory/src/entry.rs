//! Where the evaluator asks, and what every reach into memory checks.

use apl_ast::Function;
use apl_c80_machine::Machine;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};
use apl_workspace::Workspace;

use crate::memory::{call, peek, poke};

/// `⌷r` peeks and `l⌷r` pokes, `l#r` calls; `⌶r` and `#r` are status
/// and conversion (`apl-c80-status`).
///
/// # Errors
/// As each of them; SYNTAX ERROR for a dyadic status.
pub fn system(
    ws: &mut Workspace,
    func: &Function,
    left: Option<&Array>,
    r: &Array,
) -> AplResult<Array> {
    match (func, left) {
        (Function::Prim('⌷'), None) => peek(ws, r),
        (Function::Prim('⌷'), Some(l)) => poke(ws, l, r),
        (Function::Prim('#'), Some(l)) => call(ws, l, r),
        (_, None) => apl_c80_status::system(ws, func, r),
        (_, Some(_)) => Err(AplError::new(ErrorKind::Syntax)),
    }
}

/// The machine, once `)RAM` has opened it.
///
/// # Errors
/// NONCE ERROR before: without `)RAM`, as on the TRS-80, there is no
/// peek, poke or call.
pub(crate) fn opened(ws: &mut Workspace) -> AplResult<&mut Machine> {
    ws.machine
        .as_mut()
        .ok_or_else(|| AplError::new(ErrorKind::Nonce))
}

/// The whole numbers in `a`, which are addresses or bytes.
///
/// # Errors
/// DOMAIN ERROR for characters or a number that is not whole.
pub(crate) fn ints(a: &Array) -> AplResult<Vec<i64>> {
    let domain = || AplError::new(ErrorKind::Domain);
    let Data::Num(ns) = &a.data else {
        return Err(domain());
    };
    ns.iter()
        .map(|n| match n {
            Number::Int(i) => Ok(*i),
            Number::Float(_) => Err(domain()),
        })
        .collect()
}

/// Each byte with its address, one of either going with every one of
/// the other.
///
/// # Errors
/// LENGTH ERROR when there are more than one of each, and not as many.
pub(crate) fn paired(bytes: &[i64], at: &[i64]) -> AplResult<Vec<(i64, i64)>> {
    let n = bytes.len().max(at.len());
    let fits = |side: &[i64]| side.len() == n || side.len() == 1;
    if !fits(bytes) || !fits(at) {
        return Err(AplError::new(ErrorKind::Length));
    }
    Ok((0..n)
        .map(|i| (bytes[i % bytes.len()], at[i % at.len()]))
        .collect())
}
