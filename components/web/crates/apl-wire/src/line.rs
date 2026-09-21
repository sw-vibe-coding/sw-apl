//! Lines on the wire, in both directions.
//!
//! One protocol line is one line of UTF-8. The service sends a JSON
//! object and the terminal sends a JSON string, so that a line is one
//! line whatever it holds -- but a line that is not JSON is taken
//! verbatim, which is what makes `nc host port` an emergency client
//! and a debugging window.

use serde::Deserialize;
use std::io::{self, BufRead, Read};

/// The longest protocol line accepted, in bytes. A terminal sends one
/// typed line at a time; anything approaching a megabyte is a client
/// that has lost its framing, not an APL statement.
const LIMIT: u64 = 1_048_576;

/// Read one line. `None` at end of input, which is the other end
/// going away.
///
/// # Errors
/// A line too long to be a typed line, or one with no ending.
pub fn read(input: &mut impl BufRead) -> io::Result<Option<String>> {
    let mut line = String::new();
    let size = input.take(LIMIT + 1).read_line(&mut line)?;
    if size == 0 {
        return Ok(None);
    }
    if size as u64 > LIMIT || !line.ends_with('\n') {
        let bad = io::ErrorKind::InvalidData;
        return Err(io::Error::new(bad, "invalid or oversized protocol line"));
    }
    Ok(Some(line))
}

/// Read one JSON value from a line.
///
/// # Errors
/// A line that is not the value expected, or that `read` refused.
pub fn receive<T: for<'a> Deserialize<'a>>(input: &mut impl BufRead) -> io::Result<Option<T>> {
    let Some(line) = read(input)? else {
        return Ok(None);
    };
    serde_json::from_str(&line)
        .map(Some)
        .map_err(io::Error::other)
}

/// The line a terminal sent: the JSON string a client sends, or the
/// line itself when it is not one.
#[must_use]
pub fn typed(line: &str) -> String {
    serde_json::from_str(line).unwrap_or_else(|_| line.trim_end_matches(['\n', '\r']).to_string())
}

/// What a terminal sends to stop a running statement: the 2741's ATTN
/// key, as a protocol line. A terminal may send it at any time, typing
/// or not, and the service acts on it at once rather than queueing it
/// behind the lines the session has not read yet.
///
/// A JSON object, where every typed line is a JSON string or, from
/// `nc`, raw text. So it is never mistaken for a line, and it cannot
/// be sent by accident: no line of APL is this object, because `{` and
/// `"` are not APL. A reader at `nc` who wants it types it.
pub const ATTENTION: &str = "{\"attn\":true}";

/// Whether a protocol line is ATTN.
#[must_use]
pub fn attention(line: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(line)
        .is_ok_and(|v| v.get("attn") == Some(&serde_json::Value::Bool(true)))
}
