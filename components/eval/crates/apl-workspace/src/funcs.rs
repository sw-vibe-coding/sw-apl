//! The function half of the symbol table. Functions and variables
//! share one name space: a name holds one or the other.

use std::rc::Rc;

use apl_ast::Defn;

use crate::workspace::Workspace;

impl Workspace {
    /// The function a name holds, if it holds one.
    #[must_use]
    pub fn function(&self, name: &str) -> Option<Rc<Defn>> {
        self.saved.funcs.get(name).map(Rc::clone)
    }

    /// True when the name holds a function. The parser needs this to
    /// read `F B` as a call rather than a syntax error.
    #[must_use]
    pub fn is_function(&self, name: &str) -> bool {
        self.saved.funcs.contains_key(name)
    }

    /// Store a definition, replacing whatever the name held.
    pub fn define(&mut self, defn: Defn) {
        self.saved.vars.remove(&defn.name);
        self.saved.funcs.insert(defn.name.clone(), Rc::new(defn));
    }

    /// Forget whatever a name holds. Nothing happens if it holds
    /// nothing, which is what the editor wants after a rename.
    pub fn erase(&mut self, name: &str) {
        self.saved.vars.remove(name);
        self.saved.funcs.remove(name);
    }
}
