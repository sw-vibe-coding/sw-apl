//! Which glyphs take an axis bracket.

use apl_value::{AXIS, AplError, ErrorKind};

/// An axis bracket on a glyph that never takes one is not APL\360
/// syntax; on a glyph that does, the particular form is simply not
/// implemented (monadic ravel with axis, for instance, is APL2).
pub fn no_axis(f: char) -> AplError {
    let kind = if AXIS.contains(f) {
        ErrorKind::NotImplemented
    } else {
        ErrorKind::Syntax
    };
    AplError::new(kind)
}
