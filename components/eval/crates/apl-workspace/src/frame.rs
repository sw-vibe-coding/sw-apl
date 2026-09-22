//! The activation stack: one entry per defined function that has been
//! called and not yet returned. It is both the dynamic scoping
//! mechanism -- each entry holds the values its locals displaced --
//! and the state indicator, since a function that fails stays on the
//! stack rather than unwinding.

use apl_saved::Activation;
use apl_value::{AplError, AplResult, ErrorKind};

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

impl Workspace {
    /// Start a call: shadow `names` (each becomes undefined, whatever
    /// it held -- a function as well as a variable) and
    /// return where the activation sits on the stack.
    ///
    /// # Errors
    /// DEPTH ERROR when calls nest, or suspensions pile up, too deep.
    pub fn enter(&mut self, name: &str, names: &[String]) -> AplResult<usize> {
        if self.saved.stack.len() >= MAX_DEPTH {
            return Err(AplError::new(ErrorKind::Depth));
        }
        let displaced = names
            .iter()
            .map(|n| (n.clone(), self.saved.swap(n, None)))
            .collect();
        self.saved.stack.push(Activation {
            name: name.to_string(),
            line: 1,
            locals: names.to_vec(),
            suspended: false,
            displaced,
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

    /// End the innermost activation, putting back what its locals
    /// displaced. Whatever a local holds goes, a function fixed under
    /// its name included.
    pub fn leave(&mut self) {
        let Some(activation) = self.saved.stack.pop() else {
            return;
        };
        for (name, was) in activation.displaced {
            self.saved.swap(&name, was);
        }
    }

    /// The state indicator: every activation, outermost first.
    #[must_use]
    pub fn si(&self) -> &[Activation] {
        &self.saved.stack
    }
}
