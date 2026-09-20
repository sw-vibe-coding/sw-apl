//! sw-apl as a service: one `Session` per connection, reading lines
//! from a terminal and writing the transcript back.
//!
//! The interpreter is untouched. What the service adds is a thread
//! that may block. Nothing above `apl-session` is used, and nothing
//! the CLI owns.

mod console;
mod driver;
mod held;

pub use console::Reader;
pub use driver::serve;
pub use held::{Held, Terminal};
