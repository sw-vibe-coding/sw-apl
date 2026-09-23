//! The active workspace, in two halves: what a `)SAVE` writes and a
//! `)LOAD` reads back, and what the terminal it is running on adds.
//! The boundary is the point of the split -- a later step writes the
//! first half to a file, and cannot be handed the second by mistake.

use std::collections::BTreeMap;
use std::path::PathBuf;

use apl_clock::{Clock, stopped};
use apl_console::{Console, Output, Shown, render_all};
use apl_modes::Mode;
use apl_space::{of_value, room, used};
use apl_store::{Files, Store};
use apl_transcript::Transcript;
use apl_value::{AplResult, Array};

use apl_saved::Saved;

/// How to evaluate one line. The evaluator passes its own
/// `eval_line`; the crates below it take this and stay ignorant of
/// what evaluation means.
pub type Run = fn(&mut Workspace, &str) -> AplResult<Output>;

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
    /// What has been assigned to (B)'s `⎕AI` and `⎕TS`, which the 5110
    /// keeps for compatibility only: it has one user and no clock.
    /// Session state, since neither is saved with a workspace.
    pub compatible: BTreeMap<String, Array>,
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
            compatible: BTreeMap::new(),
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
