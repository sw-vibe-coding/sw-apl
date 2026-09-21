//! The sw-apl terminal protocol: a frame of transcript and prompt out,
//! a typed line in, over any connection that carries UTF-8 lines.
//!
//! Overstrike composition is not here. The front end composes -- the
//! CLI has no service and must compose locally anyway -- so what
//! travels is an ordinary line of APL, which is what
//! `Session::respond` wants.

mod frame;
mod line;
mod socket;

pub use frame::{Frame, Link, send, text};
pub use line::{ATTENTION, attention, read, receive, typed};
pub use socket::Socket;
