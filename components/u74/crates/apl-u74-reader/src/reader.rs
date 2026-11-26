//! The console a statement reads its input through: the deck.

use apl_session::{Console, INDENT, Monitor, Shown};
use apl_u74_card::Card;
use apl_u74_listing::card;
use apl_u74_mnemonic::spell;

use crate::tray::{Shared, Tray};

/// Reads data cards for `⎕` and `⍞`, and prints to the printout.
#[derive(Debug)]
pub struct Reader(pub Shared);

impl Monitor for Reader {}

impl Console for Reader {
    /// `⍞` (no prompt) takes the card's columns 1-72 as punched; `⎕`
    /// its statement spelled out. A control card is not data: the
    /// read finds none, and the run ends.
    fn read(&mut self, shown: &Shown, prompt: &str) -> Option<String> {
        let mut tray = self.0.borrow_mut();
        tray.list(shown.lines.iter().cloned());
        if !prompt.is_empty() {
            tray.list([prompt.to_string()]);
        }
        let Some(data) = next(&mut tray) else {
            tray.dry = true;
            return None;
        };
        tray.list([card(&data)]);
        if prompt.is_empty() {
            return Some(data.statement());
        }
        let text = spell(&data.statement()).map_or_else(|_| data.statement(), |s| s.text);
        tray.list([format!("{INDENT}{text}")]);
        Some(text)
    }

    fn show(&mut self, lines: &[String]) -> bool {
        self.0.borrow_mut().list(lines.iter().cloned());
        true
    }
}

/// The next card, when it is a data card.
fn next(tray: &mut Tray) -> Option<Card> {
    tray.cards.front()?.control().is_none().then_some(())?;
    tray.cards.pop_front()
}
