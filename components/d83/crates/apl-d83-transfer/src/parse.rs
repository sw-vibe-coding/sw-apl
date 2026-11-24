//! A transfer form read back.

use apl_value::{Array, Data, Number};

/// What a transfer form describes.
#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    /// A variable, or one of the system variables, and its value.
    Value(String, Array),
    /// A function, and its canonical form as a character matrix.
    Function(String, Array),
}

impl Object {
    /// The name the form gives.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Object::Value(name, _) | Object::Function(name, _) => name,
        }
    }
}

/// The object a transfer form at the start of `text` describes, and
/// how many characters the form took. The shape says how many
/// elements follow, so a form ends where its elements do and a
/// character value may hold anything at all, a line ending included.
/// `None` when `text` does not begin with a form.
#[must_use]
pub fn parse(text: &[char]) -> Option<(Object, usize)> {
    let code = *text.first()?;
    let mut at = 1;
    let name = word(text, &mut at)?;
    let rank: usize = word(text, &mut at)?.parse().ok()?;
    let shape = (0..rank)
        .map(|_| word(text, &mut at)?.parse().ok())
        .collect::<Option<Vec<usize>>>()?;
    let count: usize = shape.iter().product();
    let data = match code {
        'N' => Data::Num(numbers(text, &mut at, count)?),
        'C' | 'F' => {
            let chars = text.get(at..at.checked_add(count)?)?.to_vec();
            at += count;
            Data::Char(chars)
        }
        _ => return None,
    };
    let array = Array { shape, data };
    match code {
        'F' if rank == 2 => Some((Object::Function(name, array), at)),
        'F' => None,
        _ => Some((Object::Value(name, array), at)),
    }
}

/// The characters from `at` to the next blank, which is passed over.
/// `None` when no blank ends them, or there are none.
fn word(text: &[char], at: &mut usize) -> Option<String> {
    let rest = text.get(*at..)?;
    let len = rest.iter().position(|c| *c == ' ').filter(|n| *n > 0)?;
    *at += len + 1;
    Some(rest[..len].iter().collect())
}

/// `count` numbers from `at`, a blank between each: APL numbers, the
/// high minus for a negative and `E` before an exponent.
fn numbers(text: &[char], at: &mut usize, count: usize) -> Option<Vec<Number>> {
    let mut got = Vec::with_capacity(count);
    for i in 0..count {
        if i > 0 {
            (text.get(*at) == Some(&' ')).then_some(())?;
            *at += 1;
        }
        let rest = text.get(*at..)?;
        let len = rest.iter().position(|c| matches!(c, ' ' | '\n' | '\r'));
        let len = len.unwrap_or(rest.len());
        let numeral: String = rest[..len].iter().collect();
        if numeral.is_empty() || numeral.contains(['-', '+']) {
            return None;
        }
        got.push(Number::from_f64(numeral.replace('¯', "-").parse().ok()?));
        *at += len;
    }
    Some(got)
}
