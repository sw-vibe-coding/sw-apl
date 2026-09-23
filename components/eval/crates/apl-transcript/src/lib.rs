//! A console on paper, for a test or a batch run: what it is shown is
//! kept, and a read is answered from lines loaded in advance. The
//! console every workspace starts with.

use std::collections::VecDeque;

use apl_console::{Console, INDENT, Shown};

/// A console on paper: it keeps everything it is shown, and answers
/// a read from the lines it was given. A workspace starts with an
/// empty one, so nothing shown is lost and a read finds no input,
/// which is an INTERRUPT. Loading `typed` is how a test, or a batch
/// run, hands it what the user would have typed.
#[derive(Debug, Default)]
pub struct Transcript {
    /// What it has been shown, in order.
    pub shown: Vec<String>,
    /// The lines it will answer reads with, in order.
    pub typed: VecDeque<String>,
}

impl Console for Transcript {
    fn read(&mut self, shown: &Shown, prompt: &str) -> Option<String> {
        let (lines, open) = shown.split();
        self.shown.extend_from_slice(lines);
        if let Some(open) = open {
            self.shown.push(open.to_string());
        }
        if !prompt.is_empty() {
            self.shown.push(prompt.to_string());
        }
        let typed = self.typed.pop_front()?;
        match self
            .shown
            .last_mut()
            .filter(|_| open.is_some() && prompt.is_empty())
        {
            Some(last) => last.push_str(&typed),
            None => self.shown.push(format!("{INDENT}{typed}")),
        }
        Some(typed)
    }

    fn take(&mut self) -> Vec<String> {
        std::mem::take(&mut self.shown)
    }
}
