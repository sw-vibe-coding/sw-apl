//! The terminal the session runs on: the two consoles a statement
//! reads through. The interactive one shares the line editor with the
//! prompt loop, so a read part way through a statement has the same
//! history as any other line; the batch one shares the script with the
//! run, so a read takes the next line and the run carries on after it.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use apl_session::{Console, INDENT, Shown};
use rustyline::DefaultEditor;

use crate::repl::read_line;
use crate::shell::Line;

/// The line editor, shared between the loop and the console so a read
/// part way through a statement uses the same history.
pub type Editor = Rc<RefCell<DefaultEditor>>;

/// The console the interactive session reads through, over the same
/// editor the loop uses.
#[derive(Debug)]
pub struct Terminal(pub Editor);

impl Console for Terminal {
    fn read(&mut self, shown: &Shown, prompt: &str) -> Option<String> {
        let (lines, open) = shown.split();
        for line in lines {
            println!("{line}");
        }
        if !prompt.is_empty() {
            println!("{prompt}");
        }
        let at = open.unwrap_or(INDENT);
        read_line(&mut self.0.borrow_mut(), at).ok().flatten()
    }
}

/// The lines not yet run, shared between the run and the console so a
/// read and the loop draw from the same place.
pub type Pending = Rc<RefCell<VecDeque<Line>>>;

/// The console a batch run reads through: it prints as it goes and
/// takes a line from the script when a statement asks for one, with
/// the flag saying whether input lines are echoed.
#[derive(Debug)]
pub struct Script(pub Pending, pub bool);

impl Console for Script {
    fn read(&mut self, shown: &Shown, prompt: &str) -> Option<String> {
        let (lines, open) = shown.split();
        for line in lines {
            println!("{line}");
        }
        if let Some(open) = open {
            print!("{open}");
        }
        if !prompt.is_empty() {
            println!("{prompt}");
        }
        let typed = self.0.borrow_mut().pop_front()?.text;
        match (self.1, open.is_some() && prompt.is_empty()) {
            (true, true) => println!("{typed}"),
            (true, false) => println!("{INDENT}{typed}"),
            (false, true) => println!(),
            (false, false) => {}
        }
        Some(typed)
    }
}

/// Ask a running statement to stop when the terminal sends an
/// interrupt. The handler does nothing but store a flag, which is
/// what makes it safe to run in a signal; a body reads it between its
/// lines. The line editor puts the terminal in raw mode while it
/// reads, so Ctrl-C at a prompt never reaches here: it cancels the
/// line, as it always did.
pub fn catch_interrupt() {
    #[cfg(unix)]
    {
        extern "C" fn stop(_signal: libc::c_int) {
            apl_call::interrupt();
        }
        // SAFETY: the handler only stores an atomic flag, which is
        // async-signal-safe.
        unsafe {
            let handler = stop as *const () as libc::sighandler_t;
            libc::signal(libc::SIGINT, handler);
        }
    }
}
