//! `)DIALECT`: which mode the session is in.

use apl_eval::Mode;

/// The mode as sw-apl writes it. `)DIALECT` only asks: the mode is
/// chosen when a session starts -- `--mode` at the CLI and the
/// service, the tab in a browser -- and a workspace in hand is never
/// read again in another mode, so there is no form that switches.
#[must_use]
pub fn dialect(mode: Mode) -> &'static str {
    match mode {
        Mode::A => "(A) '70",
        Mode::B => "(B) '75",
    }
}
