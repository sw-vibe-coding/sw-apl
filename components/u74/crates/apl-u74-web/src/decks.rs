//! The demo decks, which are the deck samples, and a deck's run.

use apl_session::{Host, Memory, Mode, QUOTA, TimeSource};

/// A deck the page offers.
pub struct Deck {
    pub title: &'static str,
    pub cards: &'static str,
}

/// The decks, the first being the one the page opens on.
pub const DECKS: [Deck; 8] = [
    Deck {
        title: "Domino",
        cards: include_str!("../../../../../samples/99-domino-74.deck"),
    },
    Deck {
        title: "A program",
        cards: include_str!("../../../../../samples/99-program-74.deck"),
    },
    Deck {
        title: "Typos",
        cards: include_str!("../../../../../samples/99-errors-74.deck"),
    },
    Deck {
        title: "Life",
        cards: include_str!("../../../../../samples/99-life-74.deck"),
    },
    Deck {
        title: "A sine plot",
        cards: include_str!("../../../../../samples/99-plot-74.deck"),
    },
    Deck {
        title: "Primes",
        cards: include_str!("../../../../../samples/99-primes-74.deck"),
    },
    Deck {
        title: "Statistics",
        cards: include_str!("../../../../../samples/99-stats-74.deck"),
    },
    Deck {
        title: "Sorting",
        cards: include_str!("../../../../../samples/99-sort-74.deck"),
    },
];

/// Every deck, for the page to list: a title line, then a card to a
/// line, and a form feed between one deck and the next.
#[must_use]
pub fn catalog() -> String {
    DECKS
        .iter()
        .map(|d| format!("{}\n{}", d.title, d.cards.trim_end()))
        .collect::<Vec<_>>()
        .join("\x0c")
}

/// The printout of a deck run as a batch job, a line to a line. The
/// browser has no clock, so the banner has no time.
#[must_use]
pub fn printout(cards: &str) -> String {
    let host = Host {
        quota: QUOTA,
        store: Box::new(Memory::default()),
        mode: Mode::U,
        clock: TimeSource::None,
    };
    apl_u74_batch::run(cards, host).join("\n")
}
