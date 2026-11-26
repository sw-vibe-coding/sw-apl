//! The run.

use std::cell::RefCell;
use std::rc::Rc;

use apl_clock::moment;
use apl_session::{Host, Session, TimeSource};
use apl_u74_card::{Card, deck};
use apl_u74_listing::{NO_FIN, card};
use apl_u74_reader::{Reader, Shared, Tray};

use crate::control::{control, finish, preamble};
use crate::statement::punch;

/// Where the run is: before its `@APL` card, in the APL, or done.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Before,
    In,
    After,
}

/// A run under way.
#[derive(Debug)]
pub struct Batch {
    pub session: Session,
    pub tray: Shared,
    pub phase: Phase,
    /// Set while the cards of a definition refused are passed over.
    pub skipping: bool,
    /// The date and time for the banner, when the host has a clock.
    pub when: Option<String>,
}

/// The printout of running `text`, a deck one card a line, on `host`.
#[must_use]
pub fn run(text: &str, host: Host) -> Vec<String> {
    let cards = deck(text);
    let mut batch = Batch::new(cards.clone(), host);
    preamble(&mut batch, &cards);
    loop {
        let Some(next) = batch.tray.borrow_mut().cards.pop_front() else {
            break;
        };
        batch.card(&next);
    }
    if batch.phase == Phase::In {
        finish(&mut batch, NO_FIN);
    }
    std::mem::take(&mut batch.tray.borrow_mut().listing)
}

impl Batch {
    /// A run of `cards` on a session attached to their reader.
    fn new(cards: Vec<Card>, host: Host) -> Batch {
        let tray = Rc::new(RefCell::new(Tray {
            cards: cards.into(),
            ..Tray::default()
        }));
        let timed = host.clock == TimeSource::Hardware;
        let session = Session::attached(Box::new(Reader(Rc::clone(&tray))), host);
        let when = timed.then(|| moment(&(session.ws.clock)()));
        let (phase, skipping) = (Phase::Before, false);
        Batch {
            session,
            tray,
            phase,
            skipping,
            when,
        }
    }

    /// Add `lines` to the printout.
    pub fn list<I: IntoIterator<Item = String>>(&self, lines: I) {
        self.tray.borrow_mut().list(lines);
    }

    /// List `next`, and act on it: a control card always, an APL one
    /// only between `@APL` and `@FIN`.
    fn card(&mut self, next: &Card) {
        self.list([card(next)]);
        if let Some(word) = next.control() {
            control(self, word);
        } else if self.phase == Phase::In {
            punch(self, next);
        }
    }
}
