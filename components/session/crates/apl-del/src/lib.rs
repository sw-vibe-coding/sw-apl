//! The del, as a session takes it: an opening del starts a function in
//! the editor (`apl-editor`), and while one is open each typed line goes
//! to it, until a closing del stores it in the workspace. What is shown
//! comes back as the session's reply.

mod del;

pub use del::{line, open};
