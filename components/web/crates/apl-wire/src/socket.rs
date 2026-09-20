//! A terminal on a raw TCP socket: what `aplterm` dials, and what
//! `nc` gets.

use std::io::{self, BufReader};
use std::net::TcpStream;

use crate::frame::{Frame, Link, send};
use crate::line::{read, typed};

/// A terminal on a TCP socket.
#[derive(Debug)]
pub struct Socket {
    lines: BufReader<TcpStream>,
    out: TcpStream,
}

impl Socket {
    /// The link a connected socket carries. Nagle is off: a frame is
    /// small and the terminal is waiting for it.
    ///
    /// # Errors
    /// Failure to take a second handle on the socket, or to set it up.
    pub fn new(socket: TcpStream) -> io::Result<Socket> {
        socket.set_nodelay(true)?;
        Ok(Socket {
            lines: BufReader::new(socket.try_clone()?),
            out: socket,
        })
    }
}

impl Link for Socket {
    fn send(&mut self, sent: &Frame) -> io::Result<()> {
        send(&mut self.out, sent)
    }

    fn recv(&mut self) -> io::Result<Option<String>> {
        Ok(read(&mut self.lines)?.map(|line| typed(&line)))
    }
}
