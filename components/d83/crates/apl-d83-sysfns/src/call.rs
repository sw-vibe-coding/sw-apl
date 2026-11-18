//! Which quad names are the shared-variable functions, and applying
//! one.

use apl_value::{AplError, AplResult, Array, Data, ErrorKind};
use apl_workspace::Workspace;

use crate::control::controls;
use crate::offer::{couplings, offers};

/// What a name names now, as `⎕NC` gives it: an offer takes only a
/// name that is free or a variable's, and 4 is no name at all.
pub type Class = fn(&Workspace, &str) -> i64;

const FUNCTIONS: [&str; 6] = ["⎕DL", "⎕EA", "⎕SVC", "⎕SVO", "⎕SVQ", "⎕SVR"];

/// True when `name` is a shared-variable function.
#[must_use]
pub fn is_function(name: &str) -> bool {
    FUNCTIONS.contains(&name)
}

/// `name r`.
///
/// # Errors
/// SYNTAX ERROR for what is not one of them; otherwise the function's
/// own.
pub fn monadic(ws: &mut Workspace, name: &str, r: &Array, class: Class) -> AplResult<Array> {
    match name {
        "⎕SVO" => couplings(ws, r, class, |ws, n| ws.shares.coupling(n)),
        "⎕SVR" => couplings(ws, r, class, |ws, n| ws.shares.retract(n)),
        "⎕SVC" => controls(ws, None, r, class),
        "⎕SVQ" => query(r),
        "⎕DL" => apl_d83_delay::delay(r),
        _ => Err(AplError::new(ErrorKind::Syntax)),
    }
}

/// `l name r`.
///
/// # Errors
/// SYNTAX ERROR for `⎕SVR` and `⎕SVQ`, which take no left argument;
/// otherwise the function's own.
pub fn dyadic(
    ws: &mut Workspace,
    name: &str,
    l: &Array,
    r: &Array,
    class: Class,
) -> AplResult<Array> {
    match name {
        "⎕SVO" => offers(ws, l, r, class),
        "⎕SVC" => controls(ws, Some(l), r, class),
        _ => Err(AplError::new(ErrorKind::Syntax)),
    }
}

/// `⎕SVQ ⍳0`, the processors offering to share with this user, and
/// `⎕SVQ p`, the names `p` has offered and that are not shared yet.
/// The auxiliary processors wait for an offer and make none, so both
/// are empty: no processors, and a matrix of no names.
fn query(r: &Array) -> AplResult<Array> {
    let empty = |shape: Vec<usize>, data| Ok(Array { shape, data });
    match (r.shape.len(), r.shape.iter().product::<usize>(), &r.data) {
        (2.., _, _) => Err(AplError::new(ErrorKind::Rank)),
        (_, 2.., _) => Err(AplError::new(ErrorKind::Length)),
        (_, 1, Data::Char(_)) => Err(AplError::new(ErrorKind::Domain)),
        (_, 1, _) => empty(vec![0, 0], Data::Char(Vec::new())),
        _ => empty(vec![0], Data::Num(Vec::new())),
    }
}
