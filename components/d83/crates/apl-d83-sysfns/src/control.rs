//! Access control, `⎕SVC`.

use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};
use apl_workspace::Workspace;

use crate::call::Class;
use crate::offer::names;

/// `⎕SVC names`, and `l ⎕SVC names`: each name's access control
/// vector, four ones and zeros, after any setting; for a matrix of
/// names a matrix of them. A setting of one number or four is every
/// name's; a matrix gives a row to each.
///
/// # Errors
/// DOMAIN ERROR for a setting not all ones and zeros; LENGTH ERROR
/// for one the wrong size; and the names' own.
pub fn controls(ws: &mut Workspace, l: Option<&Array>, r: &Array, c: Class) -> AplResult<Array> {
    let (names, matrix) = names(ws, r, c)?;
    let settings = l.map(|l| rows(l, names.len())).transpose()?;
    let mut got = Vec::new();
    for (i, name) in names.iter().enumerate() {
        let row = settings.as_ref().map(|s| s[i]);
        got.extend(
            ws.shares
                .control(name, row)
                .map(|b| Number::Int(i64::from(b))),
        );
    }
    let shape = if matrix {
        vec![names.len(), 4]
    } else {
        vec![4]
    };
    Ok(Array {
        shape,
        data: Data::Num(got),
    })
}

/// A setting as one row of four for each of `n` names.
fn rows(l: &Array, n: usize) -> AplResult<Vec<[bool; 4]>> {
    let Data::Num(bits) = &l.data else {
        return Err(AplError::new(ErrorKind::Domain));
    };
    let bit = |b: &Number| match Number::from_f64(b.as_f64()) {
        Number::Int(0) => Ok(false),
        Number::Int(1) => Ok(true),
        _ => Err(AplError::new(ErrorKind::Domain)),
    };
    let bits = bits.iter().map(bit).collect::<AplResult<Vec<bool>>>()?;
    let row = |at: usize| [0, 1, 2, 3].map(|k| bits[at + k]);
    match (l.shape.as_slice(), bits.len()) {
        ([] | [_], 1) => Ok(vec![[bits[0]; 4]; n]),
        ([] | [_], 4) => Ok(vec![row(0); n]),
        ([m, 4], _) if *m == n => Ok((0..n).map(|i| row(4 * i)).collect()),
        _ => Err(AplError::new(ErrorKind::Length)),
    }
}
