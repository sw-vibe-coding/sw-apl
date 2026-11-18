//! What each thing in a workspace costs, in bytes.

use std::collections::HashMap;
use std::rc::Rc;

use apl_ast::Defn;
use apl_value::{Array, Data};

use crate::compact::used_by;

/// Bytes a value's descriptor costs whatever its shape: what kind of
/// elements it holds, its rank, and where the elements are.
pub(crate) const DESCRIPTOR: usize = 16;
/// Bytes one axis length costs in the descriptor.
pub(crate) const AXIS: usize = 4;
/// Bytes one number costs. APL\360 held every number as a double and
/// so does sw-apl, so this does not depend on the number being whole:
/// the accounting must not move when `4÷2` comes back as an integer.
const NUMBER: usize = 8;
/// Bytes one character costs, as on a 360. A glyph takes three bytes
/// in UTF-8, but what is charged is the character, not the encoding.
const CHARACTER: usize = 1;
/// Bytes a symbol table entry costs beyond the name's characters.
const ENTRY: usize = 8;
/// Bytes a line of a defined function costs beyond its text.
const LINE: usize = 4;

/// The names holding a variable. Named here because this is where it
/// is measured, and `Saved` uses the same alias.
pub type Vars = HashMap<String, Array>;

/// The names holding a defined function.
pub type Funcs = HashMap<String, Rc<Defn>>;

/// The names holding a group: a name standing for a list of names.
/// A member need not exist, which is why this holds names and not
/// references to anything.
pub type Groups = HashMap<String, Vec<String>>;

/// What a value costs: its descriptor, one entry per axis, and its
/// elements. An empty array still costs the first two.
#[must_use]
pub fn of_value(value: &Array) -> usize {
    let elements = match &value.data {
        Data::Num(v) => v.len() * NUMBER,
        Data::Char(v) => v.len() * CHARACTER,
    };
    DESCRIPTOR + value.shape.len() * AXIS + elements
}

/// What a name costs in the symbol table: the entry and its
/// characters.
#[must_use]
pub fn of_name(name: &str) -> usize {
    ENTRY + name.chars().count()
}

/// What a defined function costs: a descriptor, the names its header
/// makes local, and each body line as it was typed. The body is held
/// as text and parsed when it runs, so text is what it costs.
#[must_use]
pub fn of_function(defn: &Defn) -> usize {
    let names: usize = defn.names().iter().map(|n| of_name(n)).sum();
    let body: usize = defn.body.iter().map(|l| LINE + l.chars().count()).sum();
    DESCRIPTOR + names + body
}

/// What the whole symbol table costs, eight bytes a number
/// (`used_by`).
#[must_use]
pub fn used(vars: &Vars, funcs: &Funcs, groups: &Groups) -> usize {
    used_by(vars, funcs, groups, of_value)
}
