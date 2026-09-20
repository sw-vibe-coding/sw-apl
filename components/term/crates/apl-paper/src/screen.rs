//! Drawing the line being typed, and printing it when it is sent.

use apl_keyboard::Keyboard;
use crossterm::{
    cursor::MoveToColumn,
    execute,
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};

use crate::display::display;
use unicode_width::UnicodeWidthStr;

/// Redraw the line the carriage is on. The whole line is rewritten
/// rather than patched: an overstrike changes a cell already passed,
/// and a glyph is not always one column wide.
///
/// # Errors
/// Whatever the terminal reports.
pub fn redraw(prompt: &str, keyboard: &Keyboard) -> io::Result<()> {
    let mut stdout = io::stdout();
    let width = usize::from(terminal::size()?.0).saturating_sub(1).max(1);
    let prefix = display(prompt);
    let before = display(&keyboard.before_cursor());
    let cursor = (prefix.width() + before.width()).saturating_sub(usize::from(keyboard.pending));
    let full = format!("{prefix}{}", display(&keyboard.text()));
    let (visible, cursor) = viewport(&full, cursor, width);
    let cursor = u16::try_from(cursor).unwrap_or(u16::MAX);
    execute!(stdout, MoveToColumn(0), Clear(ClearType::CurrentLine))?;
    write!(stdout, "{visible}")?;
    execute!(stdout, MoveToColumn(cursor))?;
    stdout.flush()
}

/// The part of a long line that fits, and where the carriage sits
/// in it. A 2741 had a carriage return and a wide platen; a terminal
/// window scrolls instead.
fn viewport(mut text: &str, mut cursor: usize, width: usize) -> (&str, usize) {
    while cursor >= width && !text.is_empty() {
        let n = text.chars().next().unwrap().len_utf8();
        cursor = cursor.saturating_sub(text[..n].width());
        text = &text[n..];
    }
    let mut end = 0;
    for (at, c) in text.char_indices() {
        if text[..at + c.len_utf8()].width() > width {
            break;
        }
        end = at + c.len_utf8();
    }
    (&text[..end], cursor)
}

/// Print the finished line and keep it for recall. The whole line is
/// reprinted, including any part that had scrolled out of view, so
/// the page holds what was sent.
///
/// # Errors
/// Whatever the terminal reports.
pub fn submit(prompt: &str, keyboard: &Keyboard, history: &mut Vec<String>) -> io::Result<String> {
    let line = keyboard.text();
    // Reprint the entire submitted line, including any scrolled portion.
    execute!(io::stdout(), MoveToColumn(0), Clear(ClearType::CurrentLine))?;
    print!("{}{}\r\n", display(prompt), display(&line));
    io::stdout().flush()?;
    if !line.is_empty() {
        history.push(line.clone());
    }
    Ok(line)
}
