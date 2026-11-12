//! A function open in definition mode, and what a typed line did.

use apl_ast::Defn;

/// How wide a line number is shown, with its brackets: as wide as
/// immediate execution indents, as the del editor shows them.
pub(crate) const WIDTH: usize = 6;

/// What one typed line did.
#[derive(Debug, PartialEq, Eq)]
pub enum Typed {
    /// The function is still open; these lines are shown.
    Stay(Vec<String>),
    /// BREAK: the function is finished.
    Leave,
    /// A system command that is not one of definition mode's, for the
    /// session to run as it would any.
    Command,
}

/// Where the next typed line goes when it is not the next new one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Slot {
    /// `)0`: a new header.
    Header,
    /// `)n`: line n was deleted, and this takes its place.
    Insert(usize),
    /// `)E n`: line n, typed over.
    Over(usize),
}

/// A function open for definition.
#[derive(Debug)]
pub struct Keyed {
    pub(crate) defn: Defn,
    pub(crate) slot: Option<Slot>,
    pub(crate) glyphs: &'static str,
}

impl Keyed {
    /// Open `defn`, new from `)DEF` or old from `)EDIT`, at the first
    /// unused line. `glyphs` is the mode's, as the lexer takes them, for
    /// a new header.
    #[must_use]
    pub fn open(defn: Defn, glyphs: &'static str) -> Keyed {
        Keyed {
            defn,
            slot: None,
            glyphs,
        }
    }

    /// The prompt: the number of the line the next typed one becomes,
    /// bracketed and padded as immediate execution is indented.
    #[must_use]
    pub fn prompt(&self) -> String {
        let n = match self.slot {
            Some(Slot::Header) => 0,
            Some(Slot::Insert(n) | Slot::Over(n)) => n,
            None => self.defn.body.len() + 1,
        };
        format!("{:<WIDTH$}", format!("[{n}]"))
    }

    /// The line `)E` offers to be typed over, already typed.
    #[must_use]
    pub fn offer(&self) -> Option<String> {
        let Some(Slot::Over(n)) = self.slot else {
            return None;
        };
        self.defn.body.get(n - 1).cloned()
    }

    /// The function as it stands, to be stored.
    #[must_use]
    pub fn close(self) -> Defn {
        self.defn
    }
}
