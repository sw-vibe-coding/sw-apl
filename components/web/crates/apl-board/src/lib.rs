//! The 2741 keyboard in a browser, or in (C) the TRS-80's and its
//! 64-by-16 screen (`Trs80`, over `apl-c80-screen`).
//!
//! `apl-keyboard` holds the whole of how a glyph is typed -- the
//! keymap, the overstrike state machine, the cells a line is made of
//! -- and depends on nothing but the overstrike table. This is the
//! browser's way in to it: one keystroke in, the line out, so the
//! page never learns what an overstrike is and the table is not
//! copied into JavaScript.
//!
//! What a keystroke *means* is decided in `press`, which is ordinary
//! Rust and tested natively. Only the binding is wasm.

mod draw;
mod press;

pub use draw::state;
pub use press::{Act, act, trs80};

#[cfg(target_arch = "wasm32")]
mod board;
#[cfg(target_arch = "wasm32")]
pub use board::Board;
#[cfg(target_arch = "wasm32")]
mod screen;
#[cfg(target_arch = "wasm32")]
pub use screen::Trs80;
