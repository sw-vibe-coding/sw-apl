//! What each thing in a workspace costs, in bytes.

use std::collections::HashMap;
use std::rc::Rc;

use apl_ast::Defn;
use apl_value::{Array, Data};

/// Bytes a value's descriptor costs whatever its shape: what kind of
/// elements it holds, its rank, and where the elements are.
const DESCRIPTOR: usize = 16;
/// Bytes one axis length costs in the descriptor.
const AXIS: usize = 4;
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

/// What the whole symbol table costs: every name, and what it holds.
/// Recomputed rather than carried along, because a running total has
/// to be given back in every place a name goes away -- a local going
/// out of scope, a rename, a clear -- and one forgotten place is a
/// workspace that fills up and never empties.
#[must_use]
pub fn used(vars: &Vars, funcs: &Funcs, groups: &Groups) -> usize {
    let held: usize = vars.iter().map(|(n, v)| of_name(n) + of_value(v)).sum();
    let defined: usize = funcs.iter().map(|(n, f)| of_name(n) + of_function(f)).sum();
    // A group is its own name and the names it lists. The members
    // are names, not referents: a member that holds nothing still
    // costs the entry that records it.
    let gathered: usize = groups
        .iter()
        .map(|(n, m)| of_name(n) + m.iter().map(|x| of_name(x)).sum::<usize>())
        .sum();
    held + defined + gathered
}
