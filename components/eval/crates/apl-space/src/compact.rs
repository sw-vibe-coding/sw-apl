//! (D)'s measure: IBM PC APL's four internal types, as its manual
//! gives them -- a boolean in a bit, an integer in two bytes, any other
//! number in eight, a character in one. A value is held in the
//! smallest type its elements fit, so a picture of 64000 ones and
//! zeros costs 8000 bytes, as it did on the PC.

use apl_value::{Array, Data, Number};

use crate::size::{AXIS, DESCRIPTOR, Funcs, Groups, Vars, of_function, of_name, of_value};

/// What a value costs held compactly: its descriptor and axes as
/// `of_value` charges them, and its elements at the narrowest type
/// that holds them all.
#[must_use]
pub fn of_compact(value: &Array) -> usize {
    let Data::Num(v) = &value.data else {
        return of_value(value);
    };
    let fits = |lo: i64, hi: i64| {
        v.iter().all(
            |n| matches!(Number::from_f64(n.as_f64()), Number::Int(i) if (lo..=hi).contains(&i)),
        )
    };
    let elements = if fits(0, 1) {
        v.len().div_ceil(8)
    } else if fits(-32768, 32767) {
        2 * v.len()
    } else {
        return of_value(value);
    };
    DESCRIPTOR + value.shape.len() * AXIS + elements
}

/// The measure a mode keeps its workspace by: compact, as (D)'s, or
/// eight bytes a number, as everyone else's.
#[must_use]
pub fn measure(compact: bool) -> fn(&Array) -> usize {
    if compact { of_compact } else { of_value }
}

/// What the whole symbol table costs: every name, and what it holds,
/// each value by `measure`. Recomputed rather than carried along,
/// because a running total has to be given back in every place a
/// name goes away -- a local going out of scope, a rename, a clear --
/// and one forgotten place is a workspace that fills up and never
/// empties.
#[must_use]
pub fn used_by(vars: &Vars, funcs: &Funcs, groups: &Groups, measure: fn(&Array) -> usize) -> usize {
    let held: usize = vars.iter().map(|(n, v)| of_name(n) + measure(v)).sum();
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
