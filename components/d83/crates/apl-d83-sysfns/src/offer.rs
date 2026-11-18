//! Offering names, and asking or retracting them: the names an
//! argument spells, and a number for each.

use apl_fix::rows;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};
use apl_workspace::Workspace;

use crate::call::Class;

/// `p ⎕SVO names`: offer each name to its processor. A scalar or one
/// processor is offered every name; otherwise one each. A label's or
/// a function's name is not offered, and stays at 0.
///
/// # Errors
/// DOMAIN ERROR for a processor that is not a whole number from 1 to
/// 32767; LENGTH ERROR when there are neither one nor one a name;
/// RANK ERROR past a vector; and the names' own.
pub fn offers(ws: &mut Workspace, l: &Array, r: &Array, class: Class) -> AplResult<Array> {
    let (names, matrix) = names(ws, r, class)?;
    let Data::Num(ps) = &l.data else {
        return Err(AplError::new(ErrorKind::Domain));
    };
    let whole = |n: &Number| match Number::from_f64(n.as_f64()) {
        Number::Int(p @ 1..=32767) => Ok(p),
        _ => Err(AplError::new(ErrorKind::Domain)),
    };
    let ps = ps.iter().map(whole).collect::<AplResult<Vec<i64>>>()?;
    let ps = match (l.shape.len(), ps.len()) {
        (2.., _) => return Err(AplError::new(ErrorKind::Rank)),
        (_, 1) => vec![ps[0]; names.len()],
        (_, n) if n == names.len() => ps,
        _ => return Err(AplError::new(ErrorKind::Length)),
    };
    let depth = ws.si().len();
    let got = names.iter().zip(ps).map(|(n, p)| match class(ws, n) {
        1 | 3 => 0,
        _ => ws.shares.offer(p, n, depth),
    });
    let got: Vec<u8> = got.collect();
    Ok(numbers(got, matrix))
}

/// `f` of each name `r` spells: one number for a vector, a vector of
/// them for a matrix of names.
///
/// # Errors
/// The names' own.
pub fn couplings(
    ws: &mut Workspace,
    r: &Array,
    class: Class,
    f: impl Fn(&mut Workspace, &str) -> u8,
) -> AplResult<Array> {
    let (names, matrix) = names(ws, r, class)?;
    let got: Vec<u8> = names.iter().map(|n| f(ws, n)).collect();
    Ok(numbers(got, matrix))
}

/// The names `r` spells, one to a row, and whether it is a matrix. A
/// row may be a name and its surrogate, which has no effect.
///
/// # Errors
/// DOMAIN ERROR for a row that is not one name or two, or numbers;
/// RANK ERROR past a matrix.
pub fn names(ws: &Workspace, r: &Array, class: Class) -> AplResult<(Vec<String>, bool)> {
    let first = |row: &String| {
        let words: Vec<&str> = row.split_whitespace().collect();
        match words.as_slice() {
            [name] | [name, _] if class(ws, name) != 4 => Ok((*name).to_string()),
            _ => Err(AplError::new(ErrorKind::Domain)),
        }
    };
    let names = rows(r)?.iter().map(first).collect::<AplResult<_>>()?;
    Ok((names, r.shape.len() == 2))
}

/// One number for one name, or a vector.
fn numbers(got: Vec<u8>, matrix: bool) -> Array {
    let got: Vec<Number> = got.into_iter().map(|g| Number::Int(i64::from(g))).collect();
    match (matrix, got.as_slice()) {
        (false, [one]) => Array::scalar(*one),
        _ => Array::vector(got),
    }
}
