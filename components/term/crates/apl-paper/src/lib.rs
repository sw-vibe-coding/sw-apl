//! The 2741's paper: what the carriage has put on the page, and
//! where it is.
//!
//! An underscored capital is shown as a circled one, because most
//! fonts draw a combining low line badly or not at all. That is a
//! presentation and nothing more: the wire carries the capital and
//! U+0332, which is what the interpreter reads and writes.

mod display;
mod screen;

pub use display::display;
pub use screen::{redraw, submit};
