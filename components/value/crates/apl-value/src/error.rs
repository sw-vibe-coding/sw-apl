//! APL\360 error kinds and the error value with its caret position.

use std::fmt;

use apl_glyphs::{LATER, LOOKALIKE};

/// The APL\360 error vocabulary (plus a temporary `NotImplemented`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    Syntax,
    Value,
    Domain,
    Rank,
    Length,
    Index,
    WsFull,
    Defn,
    /// A character outside the accepted set; carries the offender.
    Character(char),
    Depth,
    Interrupt,
    /// Primitive or feature not yet implemented in sw-apl.
    NotImplemented,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Syntax => write!(f, "SYNTAX ERROR"),
            ErrorKind::Value => write!(f, "VALUE ERROR"),
            ErrorKind::Domain => write!(f, "DOMAIN ERROR"),
            ErrorKind::Rank => write!(f, "RANK ERROR"),
            ErrorKind::Length => write!(f, "LENGTH ERROR"),
            ErrorKind::Index => write!(f, "INDEX ERROR"),
            ErrorKind::WsFull => write!(f, "WS FULL"),
            ErrorKind::Defn => write!(f, "DEFN ERROR"),
            ErrorKind::Character(c) => write_character(f, *c),
            ErrorKind::Depth => write!(f, "DEPTH ERROR"),
            ErrorKind::Interrupt => write!(f, "INTERRUPT"),
            ErrorKind::NotImplemented => write!(f, "NOT IMPLEMENTED"),
        }
    }
}

/// `CHARACTER ERROR: U+XXXX` with a hint when there is one.
fn write_character(f: &mut fmt::Formatter<'_>, c: char) -> fmt::Result {
    write!(f, "CHARACTER ERROR: U+{:04X}", u32::from(c))?;
    if let Some((_, glyph)) = LOOKALIKE.iter().find(|(bad, _)| *bad == c) {
        return write!(f, " (use {glyph} U+{:04X})", u32::from(*glyph));
    }
    match LATER.iter().find(|(bad, _)| *bad == c) {
        Some((_, what)) => write!(f, " ({what}, not APL\\360)"),
        None => Ok(()),
    }
}

/// An error with the character offset (into the statement) where it
/// was detected, when known.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AplError {
    pub kind: ErrorKind,
    pub caret: Option<usize>,
}

impl AplError {
    /// An error with no caret yet.
    #[must_use]
    pub fn new(kind: ErrorKind) -> Self {
        AplError { kind, caret: None }
    }

    /// Attach a caret position unless one is already set (the
    /// innermost detection point wins).
    #[must_use]
    pub fn at(mut self, pos: usize) -> Self {
        self.caret.get_or_insert(pos);
        self
    }
}
