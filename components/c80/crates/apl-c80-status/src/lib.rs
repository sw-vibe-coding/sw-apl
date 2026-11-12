//! APL80's two operators of its own (docs/mode-c.md): `s`, status,
//! which (C) types as the I-beam it stands for, and `#`, which turns
//! characters into APL80's character codes and codes back into
//! characters. The screen control codes `#` also sends, and Call, its
//! dyadic form, belong to the screen and are not here.

mod codes;
mod special;
mod status;

pub use codes::{code, glyph};
pub use status::monadic;
