//! The terminal a session is attached to, and the one question the
//! service ever asks it: here is what you have printed, now type.

use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use apl_wire::{Frame, Link};

/// The link, and whether the terminal has gone.
///
/// The flag is worth keeping rather than waiting for the next write
/// to fail: a socket whose far end has closed accepts one more write
/// into the kernel's buffer, so a session that lost its terminal part
/// way through a statement would otherwise print a whole error report
/// into the void before noticing.
#[derive(Debug)]
pub struct Held {
    link: Box<dyn Link>,
    /// Set when a read found the terminal gone.
    pub gone: bool,
    /// The session's mode, by letter, which every frame carries so
    /// the terminal composes overstrikes for it.
    mode: char,
}

/// The terminal, shared between the prompt loop and the console. One
/// connection is read by one thread, so the sharing needs no lock.
pub type Terminal = Rc<RefCell<Held>>;

impl Held {
    /// A terminal on `link`, with nobody yet gone.
    #[must_use]
    pub fn new(link: Box<dyn Link>, mode: char) -> Terminal {
        Rc::new(RefCell::new(Held {
            link,
            gone: false,
            mode,
        }))
    }

    /// Show `lines`, prompt, and wait for the line typed back.
    /// `None` when the terminal has gone.
    ///
    /// A control character is not something a 2741 can type, so a
    /// line carrying one is a client that has lost its framing rather
    /// than an operator: the session ends rather than feeding it to
    /// the lexer as a CHARACTER ERROR per line.
    ///
    /// # Errors
    /// A transport failure, or such a line.
    pub fn ask(&mut self, lines: Vec<String>, prompt: String) -> io::Result<Option<String>> {
        let frame = Frame {
            lines,
            prompt: Some(prompt),
            mode: self.mode.to_string(),
            ..Frame::default()
        };
        self.link.send(&frame)?;
        let typed = self.link.recv()?;
        self.gone = typed.is_none();
        match typed {
            Some(line) if line.chars().any(char::is_control) => {
                let bad = io::ErrorKind::InvalidData;
                Err(io::Error::new(bad, "expected a composed Unicode line"))
            }
            line => Ok(line),
        }
    }

    /// Send lines a statement printed while it runs: no prompt, and
    /// more to come. Nothing is read back.
    ///
    /// # Errors
    /// Whatever the transport reports writing.
    pub fn tell(&mut self, lines: Vec<String>) -> io::Result<()> {
        self.link.send(&Frame {
            lines,
            more: true,
            mode: self.mode.to_string(),
            ..Frame::default()
        })
    }

    /// Send the last frame of a session: what `)OFF` printed, and no
    /// prompt, because nothing more will be typed.
    ///
    /// # Errors
    /// Whatever the transport reports writing.
    pub fn sign_off(&mut self, lines: Vec<String>) -> io::Result<()> {
        self.link.send(&Frame {
            lines,
            off: true,
            mode: self.mode.to_string(),
            ..Frame::default()
        })
    }
}
