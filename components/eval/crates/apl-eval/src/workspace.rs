//! The active workspace: variables, the index origin, pending quad
//! output.

use std::collections::HashMap;

use apl_prims::Env;
use apl_value::Array;

/// State of the active workspace.
#[derive(Debug, Default)]
pub struct Workspace {
    vars: HashMap<String, Array>,
    /// Index origin and random link.
    pub env: Env,
    /// Values written with `⎕←`, in order, not yet displayed.
    pub output: Vec<Array>,
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

/// What a statement produces for the terminal.
#[derive(Debug, Clone, PartialEq)]
pub enum Output {
    /// Nothing to display (blank line, comment, assignment).
    Nothing,
    /// One array to display.
    Value(Array),
    /// Mixed output: the parts are displayed side by side.
    Mixed(Vec<Array>),
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
