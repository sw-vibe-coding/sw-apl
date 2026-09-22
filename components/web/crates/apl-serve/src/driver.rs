//! One session, held over one link, until `)OFF` or the terminal goes
//! away.

use std::io;
use std::rc::Rc;

use apl_session::{Host, Session, Shown};
use apl_wire::Link;

use crate::console::Reader;
use crate::held::{Held, Terminal};

/// Hold a session on `link` until it ends. `host` is the workspace
/// size, where the libraries are kept, and the mode, as the CLI takes
/// them.
/// The session is built here, inside the thread that will run it, so
/// no workspace is ever shared between terminals -- and neither is
/// the store, which is why each terminal is handed its own.
///
/// `Session::attached` installs the real clock, so `⌶20` answers with
/// the time of day and `⌶24` with this connection's sign-on. In a
/// browser that clock is the page's, which is what `apl-ibeam` asks
/// chrono for on wasm.
///
/// # Errors
/// A transport failure, or a protocol line the terminal should not
/// have sent.
pub fn serve(link: Box<dyn Link>, host: Host) -> io::Result<()> {
    let terminal = Held::new(link);
    let mut session = Session::attached(Box::new(Reader(Rc::clone(&terminal))), host);
    run(&mut session, &terminal)
}

/// Prompt, read, answer, and prompt again.
///
/// A terminal that goes away ends the session without signing it off.
/// The workspace goes with it: a dropped line is not `)OFF`, and
/// holding a workspace for a reconnection is what APL\360 had sign-on
/// numbers for, which are out of scope here. `)CONTINUE` still saves
/// what it always saved, because it saves before it ends.
fn run(session: &mut Session, terminal: &Terminal) -> io::Result<()> {
    let mut shown = Shown::default();
    loop {
        let prompt = carriage(&shown, &session.prompt());
        let Some(line) = terminal.borrow_mut().ask(shown.lines, prompt)? else {
            return Ok(());
        };
        let reply = session.respond(&line);
        if terminal.borrow().gone {
            return Ok(());
        }
        if reply.off {
            return terminal.borrow_mut().sign_off(reply.lines);
        }
        shown = Shown {
            lines: reply.lines,
            open: reply.open,
        };
    }
}

/// Where the carriage is when the typing starts: on the line `⍞←`
/// left open, or at the prompt for a fresh one.
fn carriage(shown: &Shown, prompt: &str) -> String {
    shown
        .split()
        .1
        .map_or_else(|| prompt.to_string(), str::to_string)
}
