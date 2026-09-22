//! The `⍝!MODES` line: how a workspace file says where it runs.

use crate::mode::{LETTERS, Mode, Modes};

/// The line's prefix. It is a directive -- a comment to APL, an
/// instruction to sw-apl -- like the others a workspace file carries.
const PREFIX: &str = "⍝!MODES";

/// How far down the line may be. It is written second, and a file
/// that mentions the directive further down is saying something else:
/// a comment in a function, a character vector.
const NEAR_THE_TOP: usize = 6;

/// The modes as the line writes them: `(A)`, `(B)`, `(A)(B)`.
#[must_use]
pub fn render(modes: Modes) -> String {
    let named = LETTERS.iter().filter(|(mode, _)| modes.has(*mode));
    named.flat_map(|(_, letter)| ['(', *letter, ')']).collect()
}

/// The modes a workspace file runs in. A file with no line was saved
/// before sw-apl had modes, and is a '70 one.
#[must_use]
pub fn modes(text: &str) -> Modes {
    let mut lines = text.lines().take(NEAR_THE_TOP);
    let Some(line) = lines.find_map(|l| l.strip_prefix(PREFIX)) else {
        return Modes::only(Mode::A);
    };
    // Every mode the line does not name is one the workspace does
    // not run in.
    let absent = LETTERS
        .iter()
        .filter(|(_, letter)| !line.contains(&format!("({letter})")));
    absent.fold(Modes::ALL, |left, (mode, _)| left.minus(Modes::only(*mode)))
}

/// `text` with its modes line saying `modes`: the line replaced where
/// there is one, and added second -- after the file's opening
/// comment -- where there is not.
#[must_use]
pub fn retag(text: &str, modes: Modes) -> String {
    let line = format!("{PREFIX} {}", render(modes));
    let mut lines: Vec<&str> = text.lines().collect();
    let found = lines
        .iter()
        .take(NEAR_THE_TOP)
        .position(|l| l.starts_with(PREFIX));
    match found {
        Some(at) => lines[at] = &line,
        None => lines.insert(1.min(lines.len()), &line),
    }
    lines.join("\n") + "\n"
}

impl Mode {
    /// The mode a host was asked for: its year or its letter, as
    /// `--mode 70` or `--mode B`. `None` for anything else.
    #[must_use]
    pub fn parse(word: &str) -> Option<Mode> {
        match word.to_ascii_uppercase().as_str() {
            "70" | "A" => Some(Mode::A),
            "75" | "B" => Some(Mode::B),
            _ => None,
        }
    }
}
