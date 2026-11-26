//! An APL card.

use apl_session::INDENT;
use apl_u74_card::Card;
use apl_u74_listing::{INCORRECT, NO_INPUT, character, column, located};
use apl_u74_mnemonic::{Spelled, spell};

use crate::batch::Batch;
use crate::control::finish;

/// Run an APL card: spelled out in glyphs, unless it holds what a
/// card cannot or is a system command, which a deck has none of.
pub fn punch(batch: &mut Batch, card: &Card) {
    if let Some(at) = card.unpunchable() {
        return batch.list(character(card, at));
    }
    let statement = card.statement();
    if statement.is_empty() {
        return;
    }
    let spelled = match spell(&statement) {
        Ok(spelled) => spelled,
        Err(at) => return batch.list(character(card, at)),
    };
    if batch.skipping {
        batch.skipping = !spelled.text.ends_with('∇');
    } else if spelled.text.trim_start().starts_with(')') {
        batch.list([INCORRECT.to_string(), located(card.number, None)]);
    } else {
        respond(batch, card, &spelled);
    }
}

/// Echo the statement as APL would, behind its prompt, run it, and
/// list what it printed; an error with its card and column.
fn respond(batch: &mut Batch, card: &Card, spelled: &Spelled) {
    let prompt = batch.session.prompt();
    let opening = prompt == INDENT && spelled.text.trim_start().starts_with('∇');
    batch.list([format!("{prompt}{}", spelled.text)]);
    let reply = batch.session.respond(&spelled.text);
    if batch.tray.borrow().dry {
        // The read found no card: that, not the interrupt it became.
        settle(batch);
        return finish(batch, NO_INPUT);
    }
    batch.list(reply.lines.iter().cloned());
    if reply.error {
        let at = column(&reply.lines, &spelled.text, &spelled.columns);
        batch.list([located(card.number, at)]);
        settle(batch);
    }
    let next = batch.session.prompt();
    if opening && next != INDENT && next.trim() != "[1]" {
        refuse(batch, card);
    }
}

/// Clear the state indicator: a batch run has no one to resume a
/// suspended function.
fn settle(batch: &mut Batch) {
    for _ in 0..batch.session.ws.si().len() {
        batch.session.respond("→");
    }
}

/// A `$DL` card naming a function already defined: APL would open it
/// to be changed, which a deck cannot do. Close it unchanged and pass
/// over the cards up to the closing del.
fn refuse(batch: &mut Batch, card: &Card) {
    batch.list(["DEFN ERROR".to_string(), located(card.number, None)]);
    batch.session.respond("∇");
    batch.skipping = true;
}
