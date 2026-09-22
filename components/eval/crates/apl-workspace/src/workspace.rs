//! The active workspace, in two halves: what a `)SAVE` writes and a
//! `)LOAD` reads back, and what the terminal it is running on adds.
//! The boundary is the point of the split -- a later step writes the
//! first half to a file, and cannot be handed the second by mistake.

use std::path::PathBuf;

use apl_clock::{Clock, stopped};
use apl_console::{Console, Output, Print, Shown, Transcript, render_all};
use apl_modes::Mode;
use apl_prims::Env;
use apl_space::{Funcs, Groups, Vars, of_value, room, used};
use apl_store::{Files, Store};
use apl_value::{AplResult, Array};

use crate::frame::Activation;

/// How to evaluate one line. The evaluator passes its own
/// `eval_line`; the crates below it take this and stay ignorant of
/// what evaluation means.
pub type Run = fn(&mut Workspace, &str) -> AplResult<Output>;

/// Everything `)SAVE` writes and `)LOAD` reads back: the symbol
/// table, the state indicator, and the settings kept beside them.
///
/// What the terminal adds is deliberately not here -- the console,
/// the clock, the moment the session signed on, and whatever the
/// current line has displayed. None of it can be written to a file
/// and read back, and a loaded workspace must not carry someone
/// else's terminal along with it.
/// Clonable so that a command which fills a workspace by running APL
/// -- `)LOAD`, `)COPY` -- can put the old one aside and give it back
/// if the new one does not fit.
#[derive(Debug, Default, Clone)]
pub struct Saved {
    /// Names that hold a value. A name holds a variable or a
    /// function, never both, which `define` and `set` keep true.
    pub vars: Vars,
    /// Names that hold a defined function.
    pub funcs: Funcs,
    /// Names that stand for a list of other names. A group is a
    /// handle for copying or erasing several things at once; its
    /// members need not exist, so this holds names, not referents.
    pub groups: Groups,
    /// The activation stack: running and stopped calls, outermost
    /// first. It is the state indicator.
    pub stack: Vec<Activation>,
    /// Index origin and random link. The link is saved so a loaded
    /// workspace carries on its sequence and a transcript that rolls
    /// still reproduces.
    pub env: Env,
    /// Print precision and width.
    pub print: Print,
    /// The workspace identifier `)WSID` reports. `None` until it is
    /// named, which the session shows as CLEAR WS.
    pub id: Option<String>,
}

/// A workspace running on a terminal.
#[derive(Debug)]
pub struct Workspace {
    /// What `)SAVE` writes and `)LOAD` reads back.
    pub saved: Saved,
    /// What the current line has displayed, in order, not yet sent
    /// to the terminal.
    pub output: Vec<Output>,
    /// The terminal: where a statement reads a line when it reads one.
    pub console: Box<dyn Console>,
    /// Where the I-beams read the time. A clear workspace has a clock
    /// that does not move, so nothing reads the real world until a
    /// host installs one that does.
    pub clock: Clock,
    /// Sixtieths of a second since midnight when the session began,
    /// which `⌶24` reports. The host sets it with the clock.
    pub signed_on: i64,
    /// Where the workspace libraries are kept: a directory for the
    /// CLI and the service, the browser's own storage for the demo.
    /// It belongs to the session, not to the workspace: a saved
    /// workspace does not carry the machine it was saved on.
    pub store: Box<dyn Store>,
    /// How many bytes this workspace may hold, and so what the space
    /// available reads against. Session state for the same reason the
    /// store is: a workspace saved under a large quota need not
    /// fit under a small one, exactly as on APL\360.
    pub quota: usize,
    /// Which of sw-apl's languages this workspace speaks, and so
    /// which libraries it sees. Set by the host, like the store and
    /// the quota, and session state for the same reason: a saved
    /// workspace says which modes it runs in, not which one it was
    /// saved from.
    pub mode: Mode,
}

impl Default for Workspace {
    fn default() -> Self {
        Workspace {
            saved: Saved::default(),
            output: Vec::new(),
            console: Box::new(Transcript::default()),
            clock: stopped,
            signed_on: 0,
            store: Box::new(Files(PathBuf::from("."))),
            quota: apl_space::DEFAULT,
            mode: Mode::default(),
        }
    }
}

impl Workspace {
    /// Everything shown so far, taken away and rendered, so a read
    /// can put its prompt after it rather than before it.
    pub fn flush(&mut self) -> Shown {
        let shown: Vec<Output> = self.output.drain(..).collect();
        render_all(&shown, self.saved.print)
    }

    /// Look up a variable.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Array> {
        self.saved.vars.get(name)
    }

    /// Assign a variable, replacing any previous value.
    ///
    /// Every assignment comes through here, which is why the quota is
    /// checked here rather than at the call sites: a value that will
    /// not fit is not stored, and the workspace is left as it was.
    ///
    /// # Errors
    /// WS FULL when the value does not fit in what the quota leaves.
    pub fn set(&mut self, name: &str, value: Array) -> AplResult<()> {
        let freed = self.saved.vars.get(name).map_or(0, of_value);
        let held = used(&self.saved.vars, &self.saved.funcs, &self.saved.groups);
        room(self.quota, held, of_value(&value), freed)?;
        self.saved.vars.insert(name.to_string(), value);
        Ok(())
    }
}
