//! The parts of picture format that read a pattern: its numerical
//! fields and the positions in each, a number's digits as a field
//! shows them, and the decorations, fixed or floating
//! (`apl-d83-picture` puts them together).

mod decor;
mod layout;
mod number;

pub use decor::{decorate, fixed};
pub use layout::{Layout, fields, joins};
pub use number::{Overflow, digits, padded, reach};
