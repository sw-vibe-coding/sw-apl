//! A function as characters and back: canonical representation,
//! `⎕CR`, and fix, `⎕FX`.

use apl_ast::Defn;
use apl_lex::tokenize;
use apl_scan::{header_text, parse_header};
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};
use apl_workspace::Workspace;

use crate::chars::{matrix, rows};

/// `⎕CR r`: the function `r` names as a matrix, its header first and
/// every line flush left, without line numbers or dels. Anything but
/// an unlocked function gives a matrix of no rows and no columns: a
/// locked function's characters are not to be had.
///
/// # Errors
/// RANK ERROR past a vector, DOMAIN ERROR unless `r` is characters.
pub fn canonical(ws: &Workspace, r: &Array) -> AplResult<Array> {
    if r.shape.len() > 1 {
        return Err(AplError::new(ErrorKind::Rank));
    }
    let name = rows(r)?.concat();
    let lines: Option<Vec<String>> = ws.function(&name).filter(|f| !f.locked).map(|f| {
        let body = f.body.iter().map(|l| l.trim().to_string());
        std::iter::once(header_text(&f)).chain(body).collect()
    });
    Ok(matrix(&lines.unwrap_or_default()))
}

/// `⎕FX r`: define the function the rows of `r` spell, header first,
/// replacing one of the same name, and give its name. When the
/// editor could not have made it, nothing changes and the result is
/// the first row at fault, numbered as the function's lines are -- the
/// header is 0 -- which is the 5110 manual's "row in error minus
/// one" whatever the index origin.
///
/// # Errors
/// RANK ERROR unless `r` is a matrix, DOMAIN ERROR unless it is
/// characters, and WS FULL when the function does not fit.
pub fn fix(ws: &mut Workspace, r: &Array) -> AplResult<Array> {
    if r.shape.len() != 2 {
        return Err(AplError::new(ErrorKind::Rank));
    }
    match definition(ws, &rows(r)?) {
        Ok(defn) => {
            let name: Vec<char> = defn.name.chars().collect();
            ws.define(defn)?;
            Ok(Array {
                shape: vec![name.len()],
                data: Data::Char(name),
            })
        }
        Err(row) => Ok(Array::scalar(Number::Int(row))),
    }
}

/// The function the rows define, or the number of the first row the
/// editor would not have taken. The header must parse and name
/// something the editor could open. A body row may not be what the
/// editor reads as a command or a closing del, and must lex in the
/// mode, as the 5110 refuses a stray character or an unpaired quote.
/// A blank row is an empty line, which the editor makes.
fn definition(ws: &Workspace, rows: &[String]) -> Result<Defn, i64> {
    let header = rows.first().and_then(|h| parse_header(h).ok());
    let header = header.filter(|d| open(ws, &d.name)).ok_or(0)?;
    let body = &rows[1..];
    let bad = |row: &String| {
        row.starts_with('[') || row == "∇" || row == "⍫" || tokenize(row, ws.mode.glyphs()).is_err()
    };
    if let Some(at) = body.iter().position(bad) {
        return Err(i64::try_from(at + 1).unwrap_or(i64::MAX));
    }
    Ok(Defn {
        body: body.to_vec(),
        ..header
    })
}

/// True when the editor could define `name`: it holds nothing, or a
/// function that is not locked. Not a function that is running or
/// waiting, and not a name a running function has made local --
/// local function names are not implemented.
fn open(ws: &Workspace, name: &str) -> bool {
    let busy = |a: &apl_workspace::Activation| a.name == name || a.locals.iter().any(|n| n == name);
    let unlocked = ws.function(name).is_none_or(|f| !f.locked);
    let free = ws.get(name).is_none() && !ws.saved.groups.contains_key(name);
    unlocked && free && !ws.si().iter().any(busy)
}
