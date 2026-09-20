//! Accepting terminals, and what bounds them.
//!
//! One thread per connection, holding one session, blocking in the
//! console read. That is the simplest thing that works and the only
//! thing that makes `Console::read` possible at all; what bounds it
//! is a fixed number of sessions, refused at the door rather than
//! queued. A local service for one reader does not need more, and a
//! thread that is asleep on a socket costs its stack and nothing
//! else.

use std::io;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

use apl_serve::serve;
use apl_session::Files;
use apl_wire::{Link, Socket};

use crate::http::greet;

/// Which terminal is on the other end.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dialled {
    /// `aplterm`, or `nc`: the protocol straight down the socket.
    Line,
    /// A browser: the page first, then the same protocol over a
    /// WebSocket.
    Browser,
}

/// What every session is given, and what limits how many there are.
///
/// The library root rather than a store: a store belongs to one
/// session and is built for it in `hold`, so no two terminals share
/// one.
#[derive(Clone, Debug)]
pub struct Service {
    /// The workspace size in bytes and the directory the libraries
    /// are under, as the CLI takes them.
    pub ws: (usize, PathBuf),
    /// How many sessions are held right now.
    pub held: Arc<AtomicUsize>,
    /// How many may be.
    pub limit: usize,
}

/// One held session, counted while it lasts. A `Drop` rather than a
/// decrement at the end of the thread, so that a session that panics
/// still gives its place back.
struct Place(Arc<AtomicUsize>);

impl Drop for Place {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::Relaxed);
    }
}

/// Accept terminals until the process is stopped. A connection
/// arriving with every session held is closed at once rather than
/// queued: the reader should be told, not left waiting.
///
/// # Errors
/// A failure accepting, which is the listener itself going wrong.
pub fn accept(listener: &TcpListener, service: &Service, over: Dialled) -> io::Result<()> {
    for socket in listener.incoming() {
        let socket = socket?;
        if service.held.fetch_add(1, Ordering::Relaxed) >= service.limit {
            service.held.fetch_sub(1, Ordering::Relaxed);
            continue;
        }
        let (place, service) = (Place(Arc::clone(&service.held)), service.clone());
        thread::spawn(move || {
            let held = hold(socket, &service, over);
            drop(place);
            if let Err(err) = held {
                eprintln!("sw-apl-server: terminal released: {err}");
            }
        });
    }
    Ok(())
}

/// Hold one session until it ends.
fn hold(socket: TcpStream, service: &Service, over: Dialled) -> io::Result<()> {
    let link: Box<dyn Link> = match over {
        Dialled::Line => Box::new(Socket::new(socket)?),
        Dialled::Browser => match greet(socket)? {
            Some(browser) => Box::new(browser),
            None => return Ok(()),
        },
    };
    let (quota, root) = service.ws.clone();
    serve(link, (quota, Box::new(Files(root))))
}
