//! What the host running a session decides for it.

use apl_store::Store;

use crate::mode::Mode;

/// What the host decides and the workspace cannot: how much it may
/// hold, where its libraries are kept, and which mode it speaks. None
/// of it is saved with a workspace, and all of it is set once, when a
/// session is attached to its terminal.
#[derive(Debug)]
pub struct Host {
    /// How many bytes the workspace may hold before WS FULL.
    pub quota: usize,
    /// Where the libraries are kept.
    pub store: Box<dyn Store>,
    /// Which mode the session is in: `--mode` at the CLI and the
    /// service, the tab in a browser.
    pub mode: Mode,
}
