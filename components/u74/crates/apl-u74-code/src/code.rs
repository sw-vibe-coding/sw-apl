//! A column's holes.

/// A column's twelve rows, top to bottom, by the names printed on
/// the card: the 12 and 11 zones, then 0 to 9.
pub const ROWS: [&str; 12] = ["12", "11", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

/// The 029's specials and their rows.
const SPECIALS: [(char, &[&str]); 26] = [
    (' ', &[]),
    ('&', &["12"]),
    ('-', &["11"]),
    ('/', &["0", "1"]),
    ('.', &["12", "3", "8"]),
    ('<', &["12", "4", "8"]),
    ('(', &["12", "5", "8"]),
    ('+', &["12", "6", "8"]),
    ('|', &["12", "7", "8"]),
    ('!', &["11", "2", "8"]),
    ('$', &["11", "3", "8"]),
    ('*', &["11", "4", "8"]),
    (')', &["11", "5", "8"]),
    (';', &["11", "6", "8"]),
    ('¬', &["11", "7", "8"]),
    (',', &["0", "3", "8"]),
    ('%', &["0", "4", "8"]),
    ('_', &["0", "5", "8"]),
    ('>', &["0", "6", "8"]),
    ('?', &["0", "7", "8"]),
    (':', &["2", "8"]),
    ('#', &["3", "8"]),
    ('@', &["4", "8"]),
    ('\'', &["5", "8"]),
    ('=', &["6", "8"]),
    ('"', &["7", "8"]),
];

/// The holes punched in a column: bit `r` set for row `ROWS[r]`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Holes(pub u16);

impl Holes {
    /// The rows punched, top to bottom, as indexes into `ROWS`.
    pub fn rows(self) -> impl Iterator<Item = usize> {
        (0..ROWS.len()).filter(move |r| self.0 & (1 << r) != 0)
    }
}

/// The holes an 029 punches for `c`, or `None` for a character it
/// cannot punch.
#[must_use]
pub fn holes(c: char) -> Option<Holes> {
    let zoned = |zone: &'static str, digit: u32| Some(vec![zone, ROWS[digit as usize + 2]]);
    let names = match c {
        '0'..='9' => Some(vec![ROWS[c as usize - '0' as usize + 2]]),
        'A'..='I' => zoned("12", c as u32 - 'A' as u32 + 1),
        'J'..='R' => zoned("11", c as u32 - 'J' as u32 + 1),
        'S'..='Z' => zoned("0", c as u32 - 'S' as u32 + 2),
        _ => SPECIALS
            .iter()
            .find(|(s, _)| *s == c)
            .map(|(_, r)| r.to_vec()),
    }?;
    Some(Holes(names.iter().map(|n| 1 << row(n)).sum()))
}

/// The index of the row named `name`.
fn row(name: &str) -> usize {
    ROWS.iter().position(|r| *r == name).unwrap_or_default()
}
