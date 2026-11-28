//! A record as a line: its code's letter, `N` or `C`, its shape, a
//! colon, then its elements (`elements.rs`).

use apl_d83_records::{Code, Record};
use apl_value::Array;

use crate::elements::{data_of, text_of};

/// The line for `record`.
pub(crate) fn encode(record: &Record) -> String {
    let v = &record.value;
    let shape: Vec<String> = v.shape.iter().map(ToString::to_string).collect();
    let (kind, body) = text_of(&v.data);
    format!("{} {kind} {}:{body}", record.code.letter(), shape.join(","))
}

/// The record a line holds, or None for one that is not.
pub(crate) fn decode(line: &str) -> Option<Record> {
    let (head, body) = line.split_once(':')?;
    let mut parts = head.split(' ');
    let code = Code::of(parts.next()?)?;
    let kind = parts.next()?;
    let dims = parts.next()?.split(',').filter(|s| !s.is_empty());
    let shape: Vec<usize> = dims.map(str::parse).collect::<Result<_, _>>().ok()?;
    let value = Array::new(shape, data_of(kind, body)?).ok()?;
    Some(Record { code, value })
}
