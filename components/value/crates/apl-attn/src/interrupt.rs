//! Ctrl-C at a terminal, as ATTN: the CLI's own way to ask.

use std::sync::OnceLock;

use crate::flag::Flag;

/// Ask a running statement to stop when the terminal sends an
/// interrupt. The handler does nothing but store a flag, which is
/// what makes it safe to run in a signal; a body reads it between its
/// lines. The line editor puts the terminal in raw mode while it
/// reads, so Ctrl-C at a prompt never reaches here: it cancels the
/// line, as it always did.
pub fn catch_interrupt() {
    // This session's flag, installed on this thread, which is the one
    // the session runs on. The handler keeps a clone: it cannot reach
    // a thread-local, and does not need to -- asking only stores.
    let flag = Flag::default();
    crate::attend(Box::new(flag.clone()));
    let _ = ASKED.set(flag);
    #[cfg(unix)]
    {
        extern "C" fn stop(_signal: libc::c_int) {
            if let Some(flag) = ASKED.get() {
                flag.ask();
            }
        }
        // SAFETY: the handler only loads a pointer and stores an
        // atomic flag, both of which are async-signal-safe.
        unsafe {
            let handler = stop as *const () as libc::sighandler_t;
            libc::signal(libc::SIGINT, handler);
        }
    }
}

/// The flag the signal handler asks. A static because a handler can
/// take no arguments; there is one CLI session per process, so one
/// is all there is.
static ASKED: OnceLock<Flag> = OnceLock::new();
