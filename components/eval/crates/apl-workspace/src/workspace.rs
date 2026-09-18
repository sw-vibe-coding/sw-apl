//! The active workspace: variables, defined functions, the index
//! origin, pending output.

use std::collections::HashMap;
use std::rc::Rc;

use apl_ast::Defn;
use apl_prims::Env;
use apl_value::Array;

use crate::frame::Activation;

/// State of the active workspace.
#[derive(Debug, Default)]
pub struct Workspace {
    pub(crate) vars: HashMap<String, Array>,
    pub(crate) funcs: HashMap<String, Rc<Defn>>,
    /// The activation stack: running and stopped calls, outermost
    /// first. It is the state indicator.
    pub(crate) stack: Vec<Activation>,
    /// Index origin and random link.
    pub env: Env,
    /// Statements displayed while the current line runs, in order,
    /// not yet sent to the terminal.
    pub output: Vec<Output>,
}

impl Workspace {
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

/// What a statement produced: what to display, and where to go next.
#[derive(Debug, Clone, PartialEq)]
pub enum Output {
    /// Nothing to display (blank line, comment, assignment), and the
    /// next line follows this one.
    Nothing,
    /// One array to display.
    Value(Array),
    /// Mixed output: the parts are displayed side by side.
    Mixed(Vec<Array>),
    /// A branch. Nothing is displayed. `Some(n)` runs line n next, or
    /// returns when the function has no line n; `None` is a bare
    /// arrow, which falls through in a body and clears the top of the
    /// state indicator in immediate execution.
    Branch(Option<i64>),
}

impl Output {
    /// The single displayed array, if that is what this is.
    #[must_use]
    pub fn value(self) -> Option<Array> {
        match self {
            Output::Value(a) => Some(a),
            _ => None,
        }
    }
}
