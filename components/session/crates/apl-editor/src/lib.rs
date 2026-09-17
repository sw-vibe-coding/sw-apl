//! The del editor. An opening del takes the session out of immediate
//! execution: the lines that follow are the function's body, typed
//! behind the number each will hold, and bracketed commands move
//! about, display, insert, and delete. A closing del ends it and
//! renumbers the lines from 1; del-tilde ends it locked.

mod apply;
mod command;
mod definition;

pub use definition::{Definition, Step};
