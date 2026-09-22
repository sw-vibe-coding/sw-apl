//! The line to the service: frames in, typed lines out.
//!
//! Frames are read on a thread of their own. While the service is
//! working the keyboard is watched for ATTN instead, and ATTN is sent
//! the moment it is pressed -- which a single thread blocked on the
//! socket could never do.

use std::io::{self, BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;

use apl_keyboard::Keyboard;
use apl_modes::Mode;
use apl_paper::display;
use apl_typing::{read_line, wait};
use apl_wire::{ATTENTION, Frame, receive, send};

/// Print what the service sent and type what it asks for, until it
/// signs off or the reader hangs up.
///
/// The keyboard and the history live here, across the whole session,
/// so a read part way through a statement recalls the same lines as
/// any other prompt -- `⎕` is typed at the same keyboard as
/// everything else.
///
/// # Errors
/// Whatever the socket or the terminal reports.
pub fn run(mut socket: TcpStream, mut keyboard: Keyboard) -> io::Result<()> {
    let frames = listen(socket.try_clone()?);
    let mut history = Vec::new();
    while let Some(frame) = next(&frames, &mut socket)? {
        // The service's mode decides which overstrikes compose.
        keyboard.also = Mode::parse(&frame.mode).unwrap_or_default().overstrikes();
        for line in &frame.lines {
            println!("{}", display(line));
        }
        if frame.off {
            break;
        }
        let Some(prompt) = frame.prompt else {
            break;
        };
        let Some(typed) = read_line(&prompt, &mut keyboard, &mut history)? else {
            break;
        };
        send(&mut socket, &typed)?;
    }
    Ok(())
}

/// Frames from the service, read on a thread of their own.
fn listen(socket: TcpStream) -> Receiver<io::Result<Option<Frame>>> {
    let (tx, frames) = mpsc::channel();
    thread::spawn(move || {
        let mut lines = BufReader::new(socket);
        loop {
            let frame = receive::<Frame>(&mut lines);
            let last = !matches!(frame, Ok(Some(_)));
            if tx.send(frame).is_err() || last {
                return;
            }
        }
    });
    frames
}

/// The next frame, with the keyboard locked and ATTN live while it
/// comes. ATTN goes as its own protocol line, not as a typed one: it is
/// an object, and a typed line is a string.
fn next(
    frames: &Receiver<io::Result<Option<Frame>>>,
    socket: &mut TcpStream,
) -> io::Result<Option<Frame>> {
    let ready = || match frames.try_recv() {
        Ok(frame) => Some(frame),
        Err(TryRecvError::Disconnected) => Some(Ok(None)),
        Err(TryRecvError::Empty) => None,
    };
    wait(ready, || {
        socket.write_all(format!("{ATTENTION}\n").as_bytes())
    })?
}
