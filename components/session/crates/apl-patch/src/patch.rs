//! A line edit in progress: waiting for its marks, then for its fill.

use apl_value::AplResult;

use crate::fill::fill;
use crate::marks::{cells, mark};

/// Where a line edit stands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Patch {
    /// The statement is displayed; the marks come next, typed from
    /// position `at`.
    Marks { shown: String, at: usize },
    /// The statement is displayed again with its marks applied; what
    /// is typed next lands on it from position `from`.
    Fill { edited: String, from: usize },
}

/// What one typed line did to a line edit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Taken {
    /// The marks: the edit goes on, and this is the line to display.
    Again(Patch, String),
    /// The fill: the edit is over, and this is the line it leaves, to
    /// be taken as if it had been typed.
    Line(String),
}

impl Patch {
    /// Begin an edit of `shown`, the statement as it is displayed --
    /// number and all -- with the carriage under position `at`.
    #[must_use]
    pub fn open(shown: &str, at: usize) -> Patch {
        Patch::Marks {
            shown: shown.to_string(),
            at,
        }
    }

    /// What the reader's line is typed behind: blanks to position
    /// `at` for the marks, and for the fill the edited statement up
    /// to its first gap, or all of it when there is none.
    #[must_use]
    pub fn prompt(&self) -> String {
        match self {
            Patch::Marks { at, .. } => " ".repeat(*at),
            Patch::Fill { edited, from } => cells(edited)[..*from].concat(),
        }
    }

    /// Take one typed line. `also` is the mode's overstrikes beyond
    /// APL\360's, for a character typed over another in the fill.
    ///
    /// # Errors
    /// CHARACTER ERROR from the fill, as `fill` gives it.
    pub fn take(self, typed: &str, also: &[(char, char, char)]) -> AplResult<Taken> {
        match self {
            Patch::Marks { shown, at } => {
                let (edited, gap) = mark(&shown, at, typed);
                let from = gap.unwrap_or_else(|| cells(&edited).len());
                let next = Patch::Fill {
                    edited: edited.clone(),
                    from,
                };
                Ok(Taken::Again(next, edited))
            }
            Patch::Fill { edited, from } => fill(&edited, from, typed, also).map(Taken::Line),
        }
    }
}
