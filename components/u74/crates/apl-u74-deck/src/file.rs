//! A deck from a file or stdin, and its listing printed. The mode has
//! no session to type at.

use std::fs;
use std::io::{self, IsTerminal, Read};
use std::path::Path;

use apl_session::Host;

/// Run the deck in `path`, or on stdin, and print its listing.
///
/// # Errors
/// I/O errors reading the deck, or a terminal on stdin with no deck.
pub fn run_deck(path: Option<&Path>, host: Host) -> io::Result<()> {
    let mut bytes = Vec::new();
    match path {
        Some(p) => bytes = fs::read(p)?,
        None if io::stdin().is_terminal() => {
            return Err(io::Error::other(
                "(U) '74 runs a deck: -f DECK, or a deck on stdin",
            ));
        }
        None => _ = io::stdin().lock().read_to_end(&mut bytes)?,
    }
    for line in apl_u74_batch::run(&String::from_utf8_lossy(&bytes), host) {
        println!("{line}");
    }
    Ok(())
}
