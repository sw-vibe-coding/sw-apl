//! The active workspace: variables, system variables, pending
//! quad output.

use std::collections::HashMap;

use apl_value::Array;

/// State of the active workspace.
#[derive(Debug)]
pub struct Workspace {
    vars: HashMap<String, Array>,
    /// Index origin (`⎕IO`), 0 or 1.
    pub io: i64,
    /// Comparison tolerance (`⎕CT`), non-negative.
    pub ct: f64,
    /// Values written with `⎕←`, in order, not yet displayed.
    pub output: Vec<Array>,
}

impl Default for Workspace {
    fn default() -> Self {
        Workspace {
            vars: HashMap::new(),
            io: 1,
            ct: 1e-13,
            output: Vec::new(),
        }
    }
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
