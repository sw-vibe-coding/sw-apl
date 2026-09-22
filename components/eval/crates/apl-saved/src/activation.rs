//! One call on the activation stack, and what its locals hid.

use std::rc::Rc;

use apl_ast::Defn;
use apl_value::Array;

/// The referent of a name: a variable's value or a function. A name
/// holds one or the other, never both, and a local hides either.
#[derive(Debug, Clone)]
pub enum Referent {
    /// A variable.
    Value(Array),
    /// A defined function.
    Function(Rc<Defn>),
}

/// One call: what it displaced, where it is, and whether it stopped.
/// Clonable because a workspace is: `)LOAD` and `)COPY` put a copy
/// aside so a load that fails part way can be undone.
#[derive(Debug, Clone)]
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
    /// What the locals displaced -- a variable or a function -- to put
    /// back on return. It is public because `)VARS` and `)FNS` list
    /// *global* names: under a suspension a local shadows a global of
    /// the same name, and the displaced entry is the only record that
    /// the global is there. Nothing outside the workspace writes it.
    pub displaced: Vec<(String, Option<Referent>)>,
}
