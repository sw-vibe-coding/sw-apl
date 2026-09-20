//! The browser's end of the protocol.
//!
//! A browser cannot open a raw socket, so the same frames travel in
//! WebSocket text messages: one message, one protocol line, exactly
//! as `aplterm` sends one line per write. Nothing else differs, which
//! is the point of the `Link` trait -- the service above cannot tell
//! the two terminals apart.

use std::io;
use std::net::TcpStream;

use apl_wire::{Frame, Link, text, typed};
use tungstenite::{Message, WebSocket};

/// A terminal in a browser.
#[derive(Debug)]
pub struct Browser(pub WebSocket<TcpStream>);

impl Link for Browser {
    fn send(&mut self, frame: &Frame) -> io::Result<()> {
        self.0
            .send(Message::text(text(frame)?))
            .map_err(io::Error::other)
    }

    /// The next typed line. A close, a ping, or anything that is not
    /// text is the terminal talking to the transport rather than to
    /// APL: a close ends the session and the rest is skipped.
    fn recv(&mut self) -> io::Result<Option<String>> {
        loop {
            match self.0.read() {
                Ok(Message::Text(text)) => return Ok(Some(typed(&text))),
                Ok(Message::Close(_)) | Err(_) => return Ok(None),
                Ok(_) => (),
            }
        }
    }
}
