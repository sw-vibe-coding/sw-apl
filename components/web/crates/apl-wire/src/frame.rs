//! One message from the service to the terminal, the line that
//! comes back, and what carries them.
//!
//! A frame is `Reply` as it goes on the wire: the transcript lines
//! this input produced, the prompt to type the next one at, and
//! whether the session has ended. Nothing richer, because nothing
//! richer is needed -- the terminal wants the lines, where the
//! carriage goes, and whether there is a next line at all.
//!
//! A line `⍞←` left open is carried as the prompt rather than as a
//! flag: the carriage stops on that line, so what it holds *is* what
//! the terminal prints before the typing. `Reply::split` is where
//! that distinction starts and `apl-serve` is where it is made.

use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::io::{self, Write};

/// What the service sends for one input line.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Frame {
    /// The transcript lines, finished, in order.
    pub lines: Vec<String>,
    /// What to print before the typing: the six-space indent, the
    /// `[n]` of the del editor, or the line `⍞←` left open. `None`
    /// when nothing more will be read.
    pub prompt: Option<String>,
    /// Set by `)OFF`: the session has ended and the link closes.
    pub off: bool,
    /// The session's mode, by letter: `A` for (A) '70, `B` for (B)
    /// '75. The terminal composes overstrikes for it -- (B) forms
    /// execute and format from their 5100 pairs, (A) does not. Empty
    /// from a service that predates modes, which is (A).
    #[serde(default)]
    pub mode: String,
}

/// Write one JSON value and its newline, and flush it: the terminal
/// at the other end is waiting for it before it types.
///
/// # Errors
/// Whatever the transport reports writing.
pub fn send<T: Serialize>(out: &mut impl Write, value: &T) -> io::Result<()> {
    serde_json::to_writer(&mut *out, value)?;
    out.write_all(b"\n")?;
    out.flush()
}

/// One value as the line it goes on the wire as, for a transport that
/// carries whole messages rather than a byte stream.
///
/// # Errors
/// A value that will not serialize, which a `Frame` never is.
pub fn text<T: Serialize>(value: &T) -> io::Result<String> {
    serde_json::to_string(value).map_err(io::Error::other)
}

/// One terminal, at the service's end.
pub trait Link: Debug {
    /// Send one frame.
    ///
    /// # Errors
    /// Whatever the transport reports writing.
    fn send(&mut self, frame: &Frame) -> io::Result<()>;

    /// Block until the terminal sends a line. `None` when it has
    /// gone -- which, part way through a statement, is a console read
    /// with no line to be had, and so an INTERRUPT.
    ///
    /// # Errors
    /// A protocol line the terminal should not have sent.
    fn recv(&mut self) -> io::Result<Option<String>>;
}
