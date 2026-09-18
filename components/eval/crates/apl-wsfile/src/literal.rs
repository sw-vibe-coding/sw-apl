//! An array as the APL that would produce it.

use apl_value::{Array, Data, Number};

/// The expression that evaluates to `array`, exactly: a scalar as
/// itself, a vector of more than one as its elements, and anything
/// else reshaped, so the rank and the shape survive the round trip.
#[must_use]
pub fn literal(array: &Array) -> String {
    let data = data_text(&array.data);
    match array.shape.as_slice() {
        [] => data,
        [n] if *n > 1 => data,
        shape => {
            let dims: Vec<String> = shape.iter().map(ToString::to_string).collect();
            format!("{}⍴{data}", dims.join(" "))
        }
    }
}

/// The elements in order. An empty array still needs something to
/// reshape, so it gets the value APL fills with.
fn data_text(data: &Data) -> String {
    match data {
        Data::Num(v) if v.is_empty() => "0".to_string(),
        Data::Num(v) => {
            let numbers: Vec<String> = v.iter().map(|n| number_text(*n)).collect();
            numbers.join(" ")
        }
        Data::Char(v) => {
            let quoted: String = v.iter().flat_map(|c| quote(*c)).collect();
            format!("'{quoted}'")
        }
    }
}

/// One number, in APL's spelling: a high minus rather than a dash,
/// and a capital E for an exponent. Floats are written to as many
/// figures as it takes to read back the same number.
fn number_text(number: Number) -> String {
    let text = match number {
        Number::Int(i) => i.to_string(),
        Number::Float(f) => format!("{f:?}"),
    };
    text.replace('-', "¯").replace('e', "E")
}

/// One character inside a literal; a quote is doubled.
fn quote(c: char) -> Vec<char> {
    match c {
        '\'' => vec!['\'', '\''],
        c => vec![c],
    }
}
