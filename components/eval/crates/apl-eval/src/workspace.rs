//! The active workspace: named variables.

use std::collections::HashMap;

use apl_value::Array;

/// Variables of the active workspace.
#[derive(Debug, Default)]
pub struct Workspace {
    vars: HashMap<String, Array>,
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
