//! A mode, a set of them, and what the host decides.

use apl_store::Store;

/// One of sw-apl's languages. (A) is '70, the default and everything
/// sw-apl was before it had modes; (B) is '75.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// '70.
    #[default]
    A,
    /// '75.
    B,
}

/// Each mode's letter, as the `⍝!MODES` line writes it, in order.
pub const LETTERS: [(Mode, char); 2] = [(Mode::A, 'A'), (Mode::B, 'B')];

/// Some of the modes: the ones a workspace runs in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Modes(u8);

impl Modes {
    /// No mode at all.
    pub const NONE: Modes = Modes(0);
    /// Every mode there is.
    pub const ALL: Modes = Modes(0b11);

    /// The one mode alone.
    #[must_use]
    pub fn only(mode: Mode) -> Modes {
        Modes(1 << mode as u8)
    }

    /// Whether `mode` is one of these.
    #[must_use]
    pub fn has(self, mode: Mode) -> bool {
        self.0 & Modes::only(mode).0 != 0
    }

    /// Whether these and `other` have a mode in common.
    #[must_use]
    pub fn meets(self, other: Modes) -> bool {
        self.0 & other.0 != 0
    }

    /// These, less the ones in `other`.
    #[must_use]
    pub fn minus(self, other: Modes) -> Modes {
        Modes(self.0 & !other.0)
    }
}

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
