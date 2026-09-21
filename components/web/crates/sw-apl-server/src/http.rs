//! Handing a browser the terminal, and then the protocol.
//!
//! This is the whole of the HTTP the service speaks: one page, and
//! one upgrade to a WebSocket. It is hand-written rather than taken
//! from a web framework because that is all it is -- a local process
//! serving one file to one reader on their own machine.

use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use tungstenite::WebSocket;
use tungstenite::handshake::derive_accept_key;
use tungstenite::protocol::Role;

/// The terminal page. It is the client; there is no build step and
/// nothing to fetch.
const PAGE: &str = include_str!("../static/index.html");

/// The longest request head accepted, in lines.
const HEAD: usize = 64;

/// Read the request head and hand back the WebSocket key, when the
/// request is an upgrade. Everything else in the head is a browser
/// telling a one-page local service things it has no use for.
///
/// # Errors
/// Whatever the socket reports reading.
fn key(lines: &mut BufReader<TcpStream>) -> io::Result<Option<String>> {
    let mut found = None;
    for _ in 0..HEAD {
        let mut line = String::new();
        if lines.read_line(&mut line)? == 0 || line.trim_end().is_empty() {
            break;
        }
        if let Some((name, value)) = line.split_once(':')
            && name.eq_ignore_ascii_case("sec-websocket-key")
        {
            found = Some(value.trim().to_string());
        }
    }
    Ok(found)
}

/// Answer one browser: the link behind the page, or the page itself.
/// `None` means the reader was handed the terminal and will be back
/// on another connection to use it.
///
/// # Errors
/// Whatever the socket reports.
pub fn greet(socket: TcpStream) -> io::Result<Option<WebSocket<TcpStream>>> {
    let mut lines = BufReader::new(socket.try_clone()?);
    match key(&mut lines)? {
        Some(key) => upgrade(socket, &key).map(Some),
        None => page(socket).map(|()| None),
    }
}

/// Send the page. No caching: the reader restarts the service to get
/// a new one, and a stale terminal is a confusing thing to debug.
///
/// The two isolation headers go out with it. `SharedArrayBuffer`
/// needs a cross-origin isolated page, and a service is free to say
/// so in a header -- which is what a static host cannot do, and the
/// only reason `pages/` installs a service worker to forge them. A
/// reader served by this one is isolated on the first response and
/// never meets that machinery.
fn page(mut socket: TcpStream) -> io::Result<()> {
    let head = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\
        Cross-Origin-Opener-Policy: same-origin\r\n\
        Cross-Origin-Embedder-Policy: require-corp\r\n\
        Cache-Control: no-store\r\nConnection: close\r\nContent-Length: ";
    write!(socket, "{head}{}\r\n\r\n{PAGE}", PAGE.len())?;
    socket.flush()
}

/// Accept the upgrade and take the socket over as a WebSocket.
fn upgrade(mut socket: TcpStream, key: &str) -> io::Result<WebSocket<TcpStream>> {
    let accept = derive_accept_key(key.as_bytes());
    write!(
        socket,
        "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\n\
         Connection: Upgrade\r\nSec-WebSocket-Accept: {accept}\r\n\r\n"
    )?;
    socket.flush()?;
    Ok(WebSocket::from_raw_socket(socket, Role::Server, None))
}
