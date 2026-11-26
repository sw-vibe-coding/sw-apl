//! The lines of a (U) '74 printout: the banner, each card as punched,
//! and where an error was found on it (docs/mode-u.md, "The listing").

mod locate;
mod printout;

pub use locate::{column, located};
pub use printout::{BANNER, banner, card, character};

/// The run ended at its `@FIN` card.
pub const END_OF_RUN: &str = "END OF RUN";
/// The deck ran out with no `@FIN` card.
pub const NO_FIN: &str = "END OF DECK, NO @FIN CARD";
/// The deck has no `@RUN` card.
pub const NO_RUN: &str = "NO @RUN CARD";
/// The deck has no `@APL` card, so its APL starts at the first card.
pub const NO_APL: &str = "NO @APL CARD, APL STARTS AT CARD 1";
/// A statement read input and no data card was left.
pub const NO_INPUT: &str = "NO CARD LEFT TO READ";
/// The deck ran out part way through a function definition.
pub const IN_DEFINITION: &str = "END OF DECK IN A FUNCTION DEFINITION";
/// A system command, which a deck has none of.
pub const INCORRECT: &str = "INCORRECT COMMAND";
