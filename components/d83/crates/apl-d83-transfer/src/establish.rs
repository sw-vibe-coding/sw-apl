//! `⎕TF`, and establishing what a transfer form describes.

use apl_fix::fix;
use apl_lex::{TokenKind, tokenize};
use apl_value::{AplError, AplResult, Array, Data, ErrorKind};
use apl_workspace::Workspace;

use crate::form::{SYSTEM, form};
use crate::parse::{Object, parse};

/// `⎕TF r`: given a name, its transfer form; given a transfer form,
/// the object established and its name. Either way an empty vector
/// when it cannot be done. A name has no blank in it and a form
/// always has one, which is how the two are told apart.
///
/// # Errors
/// RANK ERROR past a vector, DOMAIN ERROR for numbers.
pub fn transfer(ws: &mut Workspace, r: &Array) -> AplResult<Array> {
    if r.shape.len() > 1 {
        return Err(AplError::new(ErrorKind::Rank));
    }
    let text: Vec<char> = match &r.data {
        Data::Char(chars) => chars.clone(),
        Data::Num(n) if n.is_empty() => Vec::new(),
        Data::Num(_) => return Err(AplError::new(ErrorKind::Domain)),
    };
    let got = if text.contains(&' ') {
        parse(&text)
            .filter(|(_, used)| text[*used..].iter().all(|c| *c == ' '))
            .and_then(|(object, _)| establish(ws, object))
    } else {
        form(ws, &text.iter().collect::<String>())
    };
    let got: Vec<char> = got.unwrap_or_default().chars().collect();
    Ok(Array {
        shape: vec![got.len()],
        data: Data::Char(got),
    })
}

/// Make `object` what its name names, and give the name; `None` when
/// it cannot be made, and then nothing has changed.
///
/// A name class conflict is ignored, as the manual says: what held
/// the name goes first, a shared variable's share retracted with it.
/// Not a function running or waiting, nor a group's name.
pub fn establish(ws: &mut Workspace, object: Object) -> Option<String> {
    match object {
        Object::Value(name, v) if SYSTEM.contains(&name.as_str()) => {
            apl_sysvars::assign(ws, &name, &v).ok().map(|()| name)
        }
        Object::Value(name, v) if valid(&name) => {
            replace(ws, &name, |ws| ws.set(&name, v).is_ok()).then_some(name)
        }
        Object::Function(name, rows) => {
            let fixed = |ws: &mut Workspace| {
                let got = fix(ws, &rows).map(|a| a.data);
                matches!(got, Ok(Data::Char(c)) if c.iter().copied().eq(name.chars()))
            };
            replace(ws, &name, fixed).then_some(name)
        }
        Object::Value(..) => None,
    }
}

/// Take away what `name` holds, then `put` something there; if that
/// fails, give back what was taken.
fn replace(ws: &mut Workspace, name: &str, put: impl FnOnce(&mut Workspace) -> bool) -> bool {
    let busy = ws.si().iter().any(|a| a.name == name);
    if busy || ws.saved.groups.contains_key(name) {
        return false;
    }
    let held = ws.saved.vars.remove(name);
    let defined = ws.saved.funcs.remove(name);
    if put(ws) {
        if held.is_some() {
            ws.shares.retract(name);
        }
        return true;
    }
    held.map(|v| ws.saved.vars.insert(name.to_string(), v));
    defined.map(|f| ws.saved.funcs.insert(name.to_string(), f));
    false
}

/// True when `name` is one name, as an assignment takes it.
fn valid(name: &str) -> bool {
    let tokens = tokenize(name, "");
    matches!(tokens.as_deref(), Ok([t]) if t.kind == TokenKind::Name(name.to_string()))
}
