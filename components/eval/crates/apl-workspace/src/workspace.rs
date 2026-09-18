//! The active workspace: variables, defined functions, the index
//! origin, pending output.

use std::collections::HashMap;
use std::rc::Rc;

use apl_ast::Defn;
use apl_console::{Console, Output, Print, Shown, Transcript, render_all};
use apl_prims::Env;
use apl_value::{AplResult, Array};

use crate::frame::Activation;

/// How to evaluate one line. The evaluator passes its own
/// `eval_line`; the crates below it take this and stay ignorant of
/// what evaluation means.
pub type Run = fn(&mut Workspace, &str) -> AplResult<Output>;

/// State of the active workspace.
#[derive(Debug)]
pub struct Workspace {
    pub(crate) vars: HashMap<String, Array>,
    pub(crate) funcs: HashMap<String, Rc<Defn>>,
    /// The activation stack: running and stopped calls, outermost
    /// first. It is the state indicator.
    pub(crate) stack: Vec<Activation>,
    /// Index origin and random link.
    pub env: Env,
    /// What the current line has displayed, in order, not yet sent
    /// to the terminal.
    pub output: Vec<Output>,
    /// Print precision and width, which APL\360 keeps here so they
    /// are saved with the workspace.
    pub print: Print,
    /// The terminal: where a statement reads a line when it reads one.
    pub console: Box<dyn Console>,
}

impl Default for Workspace {
    fn default() -> Self {
        Workspace {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            stack: Vec::new(),
            env: Env::default(),
            output: Vec::new(),
            print: Print::default(),
            console: Box::new(Transcript::default()),
        }
    }
}

impl Workspace {
    /// Everything shown so far, taken away and rendered, so a read
    /// can put its prompt after it rather than before it.
    pub fn flush(&mut self) -> Shown {
        let shown: Vec<Output> = self.output.drain(..).collect();
        render_all(&shown, self.print)
    }

    /// Look up a variable.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Array> {
        self.vars.get(name)
    }

    /// Assign a variable, replacing any previous value.
    pub fn set(&mut self, name: &str, value: Array) {
        self.vars.insert(name.to_string(), value);
    }
}
