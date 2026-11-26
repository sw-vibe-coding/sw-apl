//! What the run and the reader share.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use apl_u74_card::Card;

/// The cards not yet read and the printout so far.
#[derive(Debug, Default)]
pub struct Tray {
    /// The cards left, in order.
    pub cards: VecDeque<Card>,
    /// The printout's lines.
    pub listing: Vec<String>,
    /// Set when a statement read and there was no data card left.
    pub dry: bool,
}

/// The tray, as the run and the reader both hold it.
pub type Shared = Rc<RefCell<Tray>>;

impl Tray {
    /// Add `lines` to the printout.
    pub fn list<I: IntoIterator<Item = String>>(&mut self, lines: I) {
        self.listing.extend(lines);
    }
}
