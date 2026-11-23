//! A pattern's fields, and what each position of a field is.

use std::ops::Range;

/// Where each numerical field of `pattern` is: a run of characters
/// bounded by blanks that holds a digit -- and, where a run of digits
/// holding a 6 is followed by a decoration, the field ends there.
#[must_use]
pub fn fields(pattern: &[char]) -> Vec<Range<usize>> {
    let mut out = Vec::new();
    let mut at = 0;
    while at < pattern.len() {
        if pattern[at] == ' ' {
            at += 1;
            continue;
        }
        let end = pattern[at..]
            .iter()
            .position(|&c| c == ' ')
            .map_or(pattern.len(), |n| at + n);
        out.extend(sixes(pattern, at..end));
        at = end;
    }
    out.retain(|r| pattern[r.clone()].iter().any(char::is_ascii_digit));
    out
}

/// A run, split after each cluster of digits holding a 6 where
/// something other than a digit, a point or a comma follows.
fn sixes(p: &[char], run: Range<usize>) -> Vec<Range<usize>> {
    let mut out = Vec::new();
    let (mut start, mut six) = (run.start, false);
    for at in run.clone() {
        if p[at].is_ascii_digit() {
            six |= p[at] == '6';
        } else if six && !joins(p, at) {
            out.push(start..at);
            (start, six) = (at, false);
        }
    }
    out.push(start..run.end);
    out
}

/// Whether the point or comma at `at` stands between two digits.
#[must_use]
pub fn joins(p: &[char], at: usize) -> bool {
    matches!(p[at], '.' | ',')
        && at > 0
        && p[at - 1].is_ascii_digit()
        && p.get(at + 1).is_some_and(char::is_ascii_digit)
}

/// The positions of a field: its digits, its decimal point (the one
/// point between digits, when there is just one), and the runs of
/// decoration against its first and last digits, which may float.
#[derive(Debug)]
pub struct Layout {
    pub digits: Vec<usize>,
    pub point: Option<usize>,
    pub left: Range<usize>,
    pub right: Range<usize>,
}

impl Layout {
    /// The layout of the field `f`; `blank` is `⎕FC[5]`, which is
    /// printed as a blank and parts a decoration from the digits.
    #[must_use]
    pub fn of(f: &[char], blank: char) -> Self {
        let digits: Vec<usize> = (0..f.len()).filter(|&i| f[i].is_ascii_digit()).collect();
        let (first, last) = (digits[0], digits[digits.len() - 1]);
        let points: Vec<usize> = (first..last)
            .filter(|&i| f[i] == '.' && joins(f, i))
            .collect();
        let point = (points.len() == 1).then(|| points[0]);
        let stop = |c: &char| *c == blank || c.is_ascii_digit();
        let lead = f[..first].iter().rev().take_while(|c| !stop(c)).count();
        let trail = f[last + 1..].iter().take_while(|c| !stop(c)).count();
        Self {
            digits,
            point,
            left: first - lead..first,
            right: last + 1..last + 1 + trail,
        }
    }
}
