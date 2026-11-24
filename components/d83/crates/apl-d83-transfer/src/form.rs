//! An object as its transfer form.

use apl_display::format_number;
use apl_fix::canonical;
use apl_value::{Array, Data};
use apl_workspace::Workspace;

/// The system variables a transfer form carries: the settings a
/// workspace keeps, which `)OUT` writes with everything else.
pub const SYSTEM: [&str; 7] = ["⎕CT", "⎕FC", "⎕IO", "⎕LX", "⎕PP", "⎕PW", "⎕RL"];

/// How many significant digits a number is written with: the
/// manual's "any numeric conversions are done to 15 digits".
const DIGITS: usize = 15;

/// The transfer form of what `name` names: a variable as `N` or `C`,
/// a function as `F` and its canonical form, one of [`SYSTEM`] as a
/// variable. `None` when there is none to be had -- nothing by that
/// name, or a locked function, which cannot be displayed.
#[must_use]
pub fn form(ws: &Workspace, name: &str) -> Option<String> {
    if SYSTEM.contains(&name) {
        return apl_sysvars::read(ws, name).ok().map(|v| value(name, &v));
    }
    if let Some(v) = ws.get(name) {
        return Some(value(name, v));
    }
    let named = Array {
        shape: vec![name.chars().count()],
        data: Data::Char(name.chars().collect()),
    };
    let rows = canonical(ws, &named).ok()?;
    (rows.shape.first() != Some(&0)).then(|| typed('F', name, &rows))
}

/// A variable's form: `N` for numbers, `C` for characters.
fn value(name: &str, v: &Array) -> String {
    let code = match v.data {
        Data::Num(_) => 'N',
        Data::Char(_) => 'C',
    };
    typed(code, name, v)
}

/// The four parts: the type, the name and a blank, the rank and shape
/// and a blank, and the elements in row major order -- numbers each
/// apart from the next by a blank, characters as they are.
fn typed(code: char, name: &str, v: &Array) -> String {
    let mut head = vec![v.shape.len().to_string()];
    head.extend(v.shape.iter().map(ToString::to_string));
    let elements: String = match &v.data {
        Data::Num(numbers) => {
            let each: Vec<String> = numbers.iter().map(|n| format_number(*n, DIGITS)).collect();
            each.join(" ")
        }
        Data::Char(chars) => chars.iter().collect(),
    };
    format!("{code}{name} {} {elements}", head.join(" "))
}
