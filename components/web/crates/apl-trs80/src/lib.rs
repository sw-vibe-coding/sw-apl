//! The TRS-80's screen in a browser: the page prints what the session
//! shows to it and draws its 16 lines with the cursor
//! (`apl-c80-screen`). Only the binding is wasm.

#[cfg(target_arch = "wasm32")]
mod screen;
#[cfg(target_arch = "wasm32")]
pub use screen::Trs80;
