//! The active workspace: variables, system variables, pending
//! quad output.

use std::collections::HashMap;

use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

/// State of the active workspace.
#[derive(Debug)]
pub struct Workspace {
    vars: HashMap<String, Array>,
    /// Index origin (`⎕IO`), 0 or 1.
    pub io: i64,
    /// Values written with `⎕←`, in order, not yet displayed.
    pub output: Vec<Array>,
}

impl Default for Workspace {
    fn default() -> Self {
        Workspace {
            vars: HashMap::new(),
            io: 1,
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

    /// Assign a system variable by its name without the quad.
    ///
    /// # Errors
    /// VALUE ERROR for an unknown name; DOMAIN ERROR for a bad value.
    pub fn set_system(&mut self, name: &str, value: &Array) -> AplResult<()> {
        match (name, &value.data) {
            ("IO", Data::Num(v)) if value.shape.is_empty() => match v[0] {
                Number::Int(io @ (0 | 1)) => {
                    self.io = io;
                    Ok(())
                }
                _ => Err(AplError::new(ErrorKind::Domain)),
            },
            ("IO", _) => Err(AplError::new(ErrorKind::Domain)),
            _ => Err(AplError::new(ErrorKind::Value)),
        }
    }
}
