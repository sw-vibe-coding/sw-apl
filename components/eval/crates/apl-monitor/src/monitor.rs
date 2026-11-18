//! What a session draws, away from the lines it prints.

/// The machine's other display. Every console is one, so a picture
/// reaches whatever the session is attached to; one with no screen to
/// show it on ignores it, which is what the default does.
pub trait Monitor {
    /// Show `picture`, what (D)'s screen processor now shows, as
    /// `apl-d83-screen` writes it.
    fn draw(&mut self, _picture: &str) {}
}
