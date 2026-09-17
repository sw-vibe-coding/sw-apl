//! Dynamic scoping: a call shadows its local names for as long as it
//! runs, and the values it displaced come back when it returns.

use apl_value::{AplError, AplResult, Array, ErrorKind};

use crate::workspace::Workspace;

/// How deep defined-function calls may nest before DEPTH ERROR. A
/// runaway recursion hits this long before the machine stack.
const MAX_DEPTH: usize = 256;

/// The values a call displaced, to be restored when it returns.
#[derive(Debug)]
pub struct Frame {
    saved: Vec<(String, Option<Array>)>,
}

impl Workspace {
    /// Start a call: shadow `names` (each becomes undefined) and
    /// return what they held.
    ///
    /// # Errors
    /// DEPTH ERROR when calls are nested too deeply.
    pub fn enter(&mut self, names: &[String]) -> AplResult<Frame> {
        if self.depth >= MAX_DEPTH {
            return Err(AplError::new(ErrorKind::Depth));
        }
        self.depth += 1;
        let saved = names
            .iter()
            .map(|n| (n.clone(), self.vars.remove(n)))
            .collect();
        Ok(Frame { saved })
    }

    /// End a call: put the displaced values back.
    pub fn leave(&mut self, frame: Frame) {
        self.depth -= 1;
        for (name, was) in frame.saved {
            match was {
                Some(value) => self.vars.insert(name, value),
                None => self.vars.remove(&name),
            };
        }
    }
}
