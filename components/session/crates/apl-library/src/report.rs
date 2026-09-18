//! The trouble reports, worded as the IBM APL\360 User's Manual
//! (Aug 1968) words them in its table of trouble report forms.
//!
//! The distinction the table draws, and the one sw-apl got wrong
//! until it was read: INCORRECT COMMAND is for a command given an
//! argument it does not take. A name that is simply not there is a
//! different answer, and saying INCORRECT COMMAND for it tells the
//! user to look at what they typed when the fault is elsewhere.

/// Report 7: no stored workspace of that name. `)LOAD`, `)COPY`,
/// `)PCOPY` and `)DROP` all give it.
pub const WS_NOT_FOUND: &str = "WS NOT FOUND";

/// Report 9: the workspace is there but holds no global object of
/// that name. Only a `)COPY` or `)PCOPY` that named one can give it.
pub const OBJECT_NOT_FOUND: &str = "OBJECT NOT FOUND";

/// Report 14: a library that is not one. The manual gives it for
/// "another user's private library, or ... a non-existent library";
/// sw-apl has no accounts, so only the second half applies.
pub const IMPROPER_LIBRARY: &str = "IMPROPER LIBRARY REFERENCE";

/// Report 16: the command itself is wrong -- a missing argument, one
/// too many, or a value outside what the command takes.
pub const INCORRECT: &str = "INCORRECT COMMAND";

/// Report 13: `)SAVE name` where a stored workspace of that name
/// exists and is not this one. The active workspace's own name
/// follows, which is what makes the reply useful.
#[must_use]
pub fn not_saved(active: &str) -> String {
    format!("NOT SAVED, THIS WS IS {active}")
}
