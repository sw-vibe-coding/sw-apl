//! The browser's end of the protocol.
//!
//! A browser cannot open a raw socket, so the same frames travel in
//! WebSocket text messages: one message, one protocol line, exactly
//! as `aplterm` sends one line per write. Nothing else differs, which
//! is the point of the `Link` trait -- the service above cannot tell
//! the two terminals apart.
//!
//! The connection is read the whole time, as a raw socket is, so that
//! an ATTN sent while a statement runs is seen at once. A WebSocket is
//! one object for both directions -- a Ping read has to be answered
//! with a Pong written on the same connection -- so it is not split
//! into a reading half and a writing half. It sits behind a lock, and
//! the reading thread takes the lock only for one short read at a
//! time, which is what the read timeout is for; a frame going out takes
//! the lock between those reads.

use std::io::{self, ErrorKind::TimedOut, ErrorKind::WouldBlock};
use std::net::{Shutdown, TcpStream};
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use apl_attn::Flag;
use apl_wire::{Frame, Link, attention, text, typed};
use tungstenite::{Error, Message, WebSocket};

/// How long the reading thread holds the connection for one read. It
/// bounds how long a frame waits to go out, and costs a wakeup each
/// time it passes with nothing read.
const TURN: Duration = Duration::from_millis(20);

/// A terminal in a browser.
#[derive(Debug)]
pub struct Browser {
    lines: Receiver<String>,
    socket: Arc<Mutex<WebSocket<TcpStream>>>,
}

impl Browser {
    /// The link a WebSocket carries, read on a thread of its own and
    /// raising `attn` whenever the page sends ATTN. A read that times
    /// out finds nothing this turn, and the thread sleeps a moment so a
    /// frame waiting to go out can take the connection.
    ///
    /// # Errors
    /// Failure to set the read timeout the reading thread relies on.
    pub fn new(socket: WebSocket<TcpStream>, attn: Flag) -> io::Result<Browser> {
        socket.get_ref().set_read_timeout(Some(TURN))?;
        let socket = Arc::new(Mutex::new(socket));
        let reading = Arc::clone(&socket);
        let (tx, lines) = mpsc::channel();
        thread::spawn(move || {
            loop {
                let text = match reading.lock().map(|mut s| s.read()) {
                    Ok(Ok(Message::Text(text))) => text,
                    Ok(Err(Error::Io(e))) if matches!(e.kind(), WouldBlock | TimedOut) => {
                        thread::sleep(Duration::from_millis(1));
                        continue;
                    }
                    Ok(Ok(Message::Close(_)) | Err(_)) | Err(_) => return,
                    Ok(Ok(_)) => continue,
                };
                if attention(&text) {
                    attn.ask();
                } else if tx.send(typed(&text)).is_err() {
                    return;
                }
            }
        });
        Ok(Browser { lines, socket })
    }
}

impl Link for Browser {
    fn send(&mut self, frame: &Frame) -> io::Result<()> {
        let mut socket = self
            .socket
            .lock()
            .map_err(|_| io::Error::other("the browser link failed"))?;
        socket
            .send(Message::text(text(frame)?))
            .map_err(io::Error::other)
    }

    /// The next typed line, or `None` when the page has closed: the
    /// reading thread ends and drops what it was sending on.
    fn recv(&mut self) -> io::Result<Option<String>> {
        Ok(self.lines.recv().ok())
    }
}

impl Drop for Browser {
    /// Close the connection when the session is done with it, which
    /// also ends the reading thread holding a share of it.
    fn drop(&mut self) {
        if let Ok(socket) = self.socket.lock() {
            let _ = socket.get_ref().shutdown(Shutdown::Both);
        }
    }
}
