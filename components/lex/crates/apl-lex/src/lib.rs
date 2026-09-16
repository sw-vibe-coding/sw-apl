//! sw-apl tokenizer: traditional glyphs as Unicode, numbers with the
//! high minus, names, comments. Positions are character indexes so
//! the session can print an APL\360 caret line.

mod number;
mod scan;
mod token;

pub use scan::tokenize;
pub use token::{Token, TokenKind};
