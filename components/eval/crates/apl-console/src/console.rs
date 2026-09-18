//! Where a statement gets a line when it reads one.

use std::collections::VecDeque;
use std::fmt::Debug;

use crate::output::Shown;

/// The six spaces immediate execution indents an input line by. It is
/// defined here, below everything that prints, so the prompt, the
/// error display and a console echoing what it read all use the one
/// width.
pub const INDENT: &str = "      ";

/// The terminal a statement reads from. A read happens part way
/// through a statement, so it is handed everything shown so far: the
/// prompt has to land after that, not before it.
pub trait Console: Debug {
    /// Show `shown`, then read a line for `prompt`. `None` when there
    /// is no line to be had, which the reader reports as INTERRUPT.
    fn read(&mut self, shown: &Shown, prompt: &str) -> Option<String>;

    /// The lines shown so far, taken away. A console that prints as it
    /// goes has none to give back.
    fn take(&mut self) -> Vec<String> {
        Vec::new()
    }
}

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
