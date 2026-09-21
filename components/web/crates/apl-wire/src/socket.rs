//! A terminal on a raw TCP socket: what `aplterm` dials, and what
//! `nc` gets.
//!
//! The connection is read on a thread of its own, the whole time, and
//! not only when the session asks for a line. A statement in progress
//! is not reading, and an ATTN sent then would sit unread in the socket
//! until the statement ended -- which, for a loop, is never. So the
//! reading thread acts on ATTN the moment it arrives, by asking the
//! session's flag, and queues every other line for the session to take
//! when it wants one.

use std::io::{self, BufReader};
use std::net::{Shutdown, TcpStream};
use std::sync::mpsc::{self, Receiver};
use std::thread;

use apl_attn::Flag;

use crate::frame::{Frame, Link, send};
use crate::line::{attention, read, typed};

/// A terminal on a TCP socket.
#[derive(Debug)]
pub struct Socket {
    lines: Receiver<io::Result<String>>,
    out: TcpStream,
}

impl Socket {
    /// The link a connected socket carries, reading on a thread of its
    /// own and raising `attn` whenever the terminal sends ATTN. `attn`
    /// is the flag the session installed on its own thread. Nagle is
    /// off: a frame is small and the terminal is waiting for it.
    ///
    /// # Errors
    /// Failure to take a second handle on the socket, or to set it up.
    pub fn new(socket: TcpStream, attn: Flag) -> io::Result<Socket> {
        socket.set_nodelay(true)?;
        let mut input = BufReader::new(socket.try_clone()?);
        let (tx, lines) = mpsc::channel();
        thread::spawn(move || {
            loop {
                let line = match read(&mut input) {
                    Ok(Some(line)) if attention(&line) => {
                        attn.ask();
                        continue;
                    }
                    Ok(Some(line)) => Ok(typed(&line)),
                    // The terminal went away: dropping the sender is the
                    // session's end of input.
                    Ok(None) => return,
                    Err(error) => Err(error),
                };
                let failed = line.is_err();
                if tx.send(line).is_err() || failed {
                    return;
                }
            }
        });
        Ok(Socket { lines, out: socket })
    }
}

impl Link for Socket {
    fn send(&mut self, sent: &Frame) -> io::Result<()> {
        send(&mut self.out, sent)
    }

    fn recv(&mut self) -> io::Result<Option<String>> {
        match self.lines.recv() {
            Ok(line) => line.map(Some),
            Err(_) => Ok(None),
        }
    }
}

impl Drop for Socket {
    /// Close the connection when the session is done with it. The
    /// reading thread holds a clone of the socket, so dropping this one
    /// would leave the connection open and the terminal waiting -- `nc`
    /// would never see the service hang up after `)OFF`. Shutting it
    /// down closes it for every clone and ends the reading thread.
    fn drop(&mut self) {
        let _ = self.out.shutdown(Shutdown::Both);
    }
}
