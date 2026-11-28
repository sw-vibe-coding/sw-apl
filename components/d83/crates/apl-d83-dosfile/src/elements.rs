//! A record's elements as the text after its colon: numbers apart by
//! blanks, characters as they are with a backslash and a line ending
//! escaped.

use apl_value::{Data, Number};

/// `data` as a line's kind, `N` or `C`, and its text.
pub(crate) fn text_of(data: &Data) -> (char, String) {
    let number = |x: &Number| match x {
        Number::Int(i) => i.to_string(),
        Number::Float(f) => format!("{f:?}"),
    };
    let escaped = |c: &char| match c {
        '\\' => "\\\\".to_string(),
        '\n' => "\\n".to_string(),
        c => c.to_string(),
    };
    match data {
        Data::Num(n) => ('N', n.iter().map(number).collect::<Vec<_>>().join(" ")),
        Data::Char(c) => ('C', c.iter().map(escaped).collect()),
    }
}

/// The elements a line of kind `kind` holds in `body`.
pub(crate) fn data_of(kind: &str, body: &str) -> Option<Data> {
    if kind != "N" {
        return Some(Data::Char(unescape(body)));
    }
    let number = |x: &str| {
        let int = x.parse().map(Number::Int).ok();
        int.or_else(|| x.parse().map(Number::Float).ok())
    };
    let words = body.split(' ').filter(|s| !s.is_empty());
    Some(Data::Num(words.map(number).collect::<Option<_>>()?))
}

/// Characters with their escapes undone.
fn unescape(body: &str) -> Vec<char> {
    let mut out = Vec::new();
    let mut chars = body.chars();
    while let Some(c) = chars.next() {
        out.push(match c {
            '\\' => match chars.next() {
                Some('n') => '\n',
                Some(other) => other,
                None => '\\',
            },
            c => c,
        });
    }
    out
}
