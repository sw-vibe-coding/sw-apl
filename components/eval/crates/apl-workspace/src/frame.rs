//! The activation stack: one entry per defined function that has been
//! called and not yet returned. It is both the dynamic scoping
//! mechanism -- each entry holds the values its locals displaced --
//! and the state indicator, since a function that fails stays on the
//! stack rather than unwinding.

use apl_value::{AplError, AplResult, Array, ErrorKind};

use crate::workspace::Workspace;

/// How deep calls may nest, and how long the state indicator may grow,
/// before DEPTH ERROR.
///
/// This is not a taste question: the evaluator recurses on the machine
/// stack, so the guard has to fire first, and a test thread's stack is
/// smaller than a program's. Measured in a debug build, recursion
/// overflowed somewhere past 192, so this leaves a wide margin.
/// `recursion_is_bounded_by_depth_error` in `apl-eval` is what
/// notices if a change makes each level cost more: it overflows
/// instead of reporting DEPTH ERROR. Raise this only with that test
/// re-measured, never to make a program fit.
const MAX_DEPTH: usize = 128;

/// One call: what it displaced, where it is, and whether it stopped.
#[derive(Debug)]
pub struct Activation {
    /// The function's name.
    pub name: String,
    /// The line it is on, or the line it stopped on.
    pub line: usize,
    /// The names it made local, for `)SIV`.
    pub locals: Vec<String>,
    /// True for the function an error came from; a caller waiting on
    /// one that stopped is pendent, not suspended.
    pub suspended: bool,
    /// The values the locals displaced, to put back on return.
    saved: Vec<(String, Option<Array>)>,
}

impl Workspace {
    /// Start a call: shadow `names` (each becomes undefined) and
    /// return where the activation sits on the stack.
    ///
    /// # Errors
    /// DEPTH ERROR when calls nest, or suspensions pile up, too deep.
    pub fn enter(&mut self, name: &str, names: &[String]) -> AplResult<usize> {
        if self.saved.stack.len() >= MAX_DEPTH {
            return Err(AplError::new(ErrorKind::Depth));
        }
        let saved = names
            .iter()
            .map(|n| (n.clone(), self.saved.vars.remove(n)))
            .collect();
        self.saved.stack.push(Activation {
            name: name.to_string(),
            line: 1,
            locals: names.to_vec(),
            suspended: false,
            saved,
        });
        Ok(self.saved.stack.len() - 1)
    }

    /// Record where activation `at` stopped. `suspended` marks the one
    /// the error came from; a caller waiting on it only notes its line.
    pub fn stop(&mut self, at: usize, line: usize, suspended: bool) {
        if let Some(activation) = self.saved.stack.get_mut(at) {
            activation.line = line;
            activation.suspended = suspended;
        }
    }

    /// End the innermost activation, putting the displaced values back.
    pub fn leave(&mut self) {
        let Some(activation) = self.saved.stack.pop() else {
            return;
        };
        for (name, was) in activation.saved {
            match was {
                Some(value) => self.saved.vars.insert(name, value),
                None => self.saved.vars.remove(&name),
            };
        }
    }

    /// The state indicator: every activation, outermost first.
    #[must_use]
    pub fn si(&self) -> &[Activation] {
        &self.saved.stack
    }
}
