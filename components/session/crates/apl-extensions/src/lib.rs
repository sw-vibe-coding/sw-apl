//! The system commands sw-apl adds, which neither APL\360 nor the
//! IBM 5100 family had. They are system commands and never quad
//! names, so no program or saved workspace can come to depend on
//! them, and a workspace written for either mode stays one its
//! historical system could run. `docs/commands-reference.md`, "What
//! sw-apl adds", lists them.

mod dialect;

pub use dialect::dialect;

use apl_eval::Workspace;

/// The reply to a command given an argument it does not take.
const INCORRECT: &str = "INCORRECT COMMAND";

/// Answer one of sw-apl's own commands, or `None` when `name` is not
/// one: the caller then tries the commands it knows itself.
#[must_use]
pub fn command(ws: &Workspace, name: &str, rest: &[&str]) -> Option<Vec<String>> {
    Some(match (name, rest) {
        ("DIALECT", []) => vec![dialect(ws.mode).to_string()],
        ("DIALECT", _) => vec![INCORRECT.to_string()],
        _ => return None,
    })
}
