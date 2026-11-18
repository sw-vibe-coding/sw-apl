//! A display a mode's machine adds beside the session's own lines:
//! (D) '83's color monitor, which AP205 draws on. The console a
//! session reads through is a monitor too, and most show nothing on
//! it; see `apl-console`.

mod monitor;

pub use monitor::Monitor;
