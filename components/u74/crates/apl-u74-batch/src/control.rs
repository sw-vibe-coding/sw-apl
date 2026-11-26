//! The control cards, and the run's start and end.

use apl_session::INDENT;
use apl_u74_card::Card;
use apl_u74_listing::{END_OF_RUN, IN_DEFINITION, NO_APL, NO_RUN, banner};

use crate::batch::{Batch, Phase};

/// What the deck lacks, said first. With no `@APL` card the APL
/// starts at once, so that a deck of statements alone still runs.
pub fn preamble(batch: &mut Batch, cards: &[Card]) {
    let has = |word| cards.iter().any(|c| c.control() == Some(word));
    if !has("RUN") {
        batch.list([NO_RUN.to_string()]);
    }
    if !has("APL") {
        batch.list([NO_APL.to_string()]);
        open(batch);
    }
}

/// `@APL` calls the processor; `@FIN` ends the run. Any other control
/// card, `@RUN` among them, is listed and nothing more.
pub fn control(batch: &mut Batch, word: &str) {
    match (word, batch.phase) {
        ("APL", Phase::Before) => open(batch),
        ("FIN", Phase::In) => finish(batch, END_OF_RUN),
        _ => {}
    }
}

/// The processor's banner and the clear workspace.
fn open(batch: &mut Batch) {
    let lines = banner(batch.when.as_deref());
    batch.list(lines);
    batch.phase = Phase::In;
}

/// End the APL, saying `how`, and first whether a definition was left
/// open.
pub fn finish(batch: &mut Batch, how: &str) {
    if batch.session.prompt() != INDENT {
        batch.list([IN_DEFINITION.to_string()]);
    }
    batch.list([how.to_string()]);
    batch.phase = Phase::After;
}
