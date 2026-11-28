//! The four codes, and a record written and read under one.

use apl_value::Array;

use crate::shape::{bits, chars, fit};
use crate::size::bytes;

/// How a file's records are read: the letter after the file name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Code {
    /// A: APL objects, with their headers.
    Apl,
    /// B: bits, eight to a byte.
    Bool,
    /// C: characters in APL's internal code.
    Chars,
    /// D: characters in ASCII.
    Ascii,
}

/// A record as written: its code and what it holds.
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub code: Code,
    pub value: Array,
}

impl Code {
    /// The code a letter names, in either case: `"A"` to `"D"`.
    #[must_use]
    pub fn of(letter: &str) -> Option<Code> {
        let all = [Code::Apl, Code::Bool, Code::Chars, Code::Ascii];
        let named = |c: &Code| letter.eq_ignore_ascii_case(&c.letter().to_string());
        all.into_iter().find(named)
    }

    /// Its letter.
    #[must_use]
    pub fn letter(self) -> char {
        match self {
            Code::Apl => 'A',
            Code::Bool => 'B',
            Code::Chars => 'C',
            Code::Ascii => 'D',
        }
    }
}

/// `value` written as a record of `code` and length `rs` bytes, or
/// None when it does not fit or is not of the code's kind (return
/// code 24). B, C and D fill the record.
#[must_use]
pub fn write(code: Code, value: &Array, rs: usize) -> Option<Record> {
    let value = match code {
        Code::Apl => (bytes(value) <= rs).then(|| value.clone())?,
        Code::Bool => bits(value, rs * 8)?,
        Code::Chars | Code::Ascii => chars(value, rs, code == Code::Ascii)?,
    };
    Some(Record { code, value })
}

/// `record` read under `code` with length `rs`, or None when it is
/// not of that kind (24).
#[must_use]
pub fn read(code: Code, record: &Record, rs: usize) -> Option<Array> {
    let text = |c: Code| matches!(c, Code::Chars | Code::Ascii);
    match code {
        Code::Apl if record.code == Code::Apl => Some(record.value.clone()),
        Code::Bool if record.code == Code::Bool => Some(fit(&record.value, rs * 8)),
        Code::Chars | Code::Ascii if text(record.code) => {
            chars(&fit(&record.value, rs), rs, code == Code::Ascii)
        }
        _ => None,
    }
}
