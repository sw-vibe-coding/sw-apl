use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Read, Write};

/// Each server message contains finished output and optionally the next prompt.
#[derive(Debug, Serialize, Deserialize)]
pub struct Frame {
    pub lines: Vec<String>,
    pub prompt: Option<String>,
    pub off: bool,
}

pub fn send<T: Serialize>(out: &mut impl Write, value: &T) -> io::Result<()> {
    serde_json::to_writer(&mut *out, value)?;
    out.write_all(b"\n")?;
    out.flush()
}

pub fn receive<T: for<'a> Deserialize<'a>>(input: &mut impl BufRead) -> io::Result<Option<T>> {
    let mut line = String::new();
    let size = input.take(1_048_577).read_line(&mut line)?;
    if size == 0 {
        return Ok(None);
    }
    if size > 1_048_576 || !line.ends_with('\n') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid or oversized protocol line",
        ));
    }
    serde_json::from_str(&line)
        .map(Some)
        .map_err(io::Error::other)
}
