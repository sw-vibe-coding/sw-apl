//! The name list, `⎕NL`.

use std::collections::BTreeSet;

use apl_fix::matrix;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};
use apl_workspace::Workspace;

use crate::names::class;

/// `letters ⎕NL classes`: the names of the classes asked for -- 1
/// labels, 2 variables, 3 functions -- that begin with one of
/// `letters` when there are any, one to a row. The manual says the
/// order has no significance; sw-apl's is alphabetical.
///
/// # Errors
/// DOMAIN ERROR for a class that is not 1, 2 or 3, or letters that
/// are not characters; RANK ERROR for either past a vector.
pub fn name_list(ws: &Workspace, letters: Option<&Array>, classes: &Array) -> AplResult<Array> {
    let wanted = kinds(classes)?;
    let letters = letters.map(initials).transpose()?;
    let begins = |n: &str| {
        let first = n.chars().next();
        letters
            .as_ref()
            .is_none_or(|l| first.is_some_and(|c| l.contains(&c)))
    };
    let names: BTreeSet<&String> = ws.saved.vars.keys().chain(ws.saved.funcs.keys()).collect();
    let listed: Vec<String> = names
        .into_iter()
        .filter(|n| begins(n) && wanted.contains(&class(ws, n)))
        .cloned()
        .collect();
    Ok(matrix(&listed))
}

/// The classes a right argument asks for.
fn kinds(r: &Array) -> AplResult<Vec<i64>> {
    if r.shape.len() > 1 {
        return Err(AplError::new(ErrorKind::Rank));
    }
    let Data::Num(numbers) = &r.data else {
        return Err(AplError::new(ErrorKind::Domain));
    };
    let kind = |n: &Number| match Number::from_f64(n.as_f64()) {
        Number::Int(k @ 1..=3) => Ok(k),
        _ => Err(AplError::new(ErrorKind::Domain)),
    };
    numbers.iter().map(kind).collect()
}

/// The initial letters a left argument allows.
fn initials(l: &Array) -> AplResult<Vec<char>> {
    if l.shape.len() > 1 {
        return Err(AplError::new(ErrorKind::Rank));
    }
    match &l.data {
        Data::Char(chars) => Ok(chars.clone()),
        Data::Num(_) => Err(AplError::new(ErrorKind::Domain)),
    }
}
