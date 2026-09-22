//! Which modes a workspace runs in, from what it uses.
//!
//! Only the code counts. A glyph in a character literal is data and a
//! glyph in a comment is prose; neither needs a mode to run.

use apl_modes::{Mode, Modes};
use apl_workspace::Saved;

/// The modes `saved` runs in: every mode, less the ones it uses
/// something missing from.
///
/// '70-only: an I-beam, which the 5100 family replaced with system
/// variables and functions, and a group, whose commands it dropped
/// (docs/mode-b.md). '75-only: execute, format, and a quad-named
/// system variable or function. Quad and quote-quad alone are I/O
/// both modes have.
#[must_use]
pub fn runs_in(saved: &Saved) -> Modes {
    let lines = saved.funcs.values().flat_map(|f| f.body.iter());
    let code: Vec<String> = lines.map(|line| code(line)).collect();
    let mut runs = Modes::ALL;
    if !saved.groups.is_empty() || code.iter().any(|l| l.contains('⌶')) {
        runs = runs.minus(Modes::only(Mode::B));
    }
    if code.iter().any(|l| added(l)) {
        runs = runs.minus(Modes::only(Mode::A));
    }
    runs
}

/// A line with its character literals and its comment taken out. A
/// doubled quote inside a literal closes it and opens it again, which
/// comes to the same thing.
fn code(line: &str) -> String {
    let mut quoted = false;
    let mut kept = String::new();
    for c in line.chars() {
        match c {
            '\'' => quoted = !quoted,
            _ if quoted => {}
            '⍝' => break,
            _ => kept.push(c),
        }
    }
    kept
}

/// Whether code uses something only '75 has: execute, format, or a
/// quad followed by a name.
fn added(code: &str) -> bool {
    let chars: Vec<char> = code.chars().collect();
    let named = chars
        .windows(2)
        .any(|w| w[0] == '⎕' && w[1].is_alphabetic());
    named || code.contains('⍎') || code.contains('⍕')
}
