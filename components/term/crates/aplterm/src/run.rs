//! The line to the service: frames in, typed lines out.

use std::io::{self, BufReader};
use std::net::TcpStream;

use apl_keyboard::Keyboard;
use apl_paper::display;
use apl_typing::read_line;
use apl_wire::{Frame, receive, send};

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
    let mut lines = BufReader::new(socket.try_clone()?);
    let mut history = Vec::new();
    while let Some(frame) = receive::<Frame>(&mut lines)? {
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
