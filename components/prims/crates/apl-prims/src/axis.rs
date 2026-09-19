//! An axis bracket where there can be none.

use apl_value::{AplError, ErrorKind};

/// An axis bracket on a glyph that does not take one here.
///
/// It is a SYNTAX ERROR whether the glyph never takes an axis or
/// takes one only in a form APL\360 has not got -- monadic ravel
/// with an axis, for instance, which is APL2. Either way there is no
/// such function to apply, so the sentence does not parse, and a
/// reply distinguishing the two would be telling the reader about
/// an APL they are not using.
pub fn no_axis(_: char) -> AplError {
    AplError::new(ErrorKind::Syntax)
}
