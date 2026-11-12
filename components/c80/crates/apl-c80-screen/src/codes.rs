//! The codes that are not printed, as (C) holds them and sends them.

/// Where PRINT @ 1 to 1023 is held: this plus the position.
const PRINT_AT: u32 = 0xE400;
/// Where the space compression codes, 192 to 255, are held: this plus
/// the number of spaces.
const SPACES: u32 = 0xE0C0;
/// Where the cursor is saved, as a poke on the screen does before it
/// moves there, and put back after.
pub const SAVE: char = '\u{E0BE}';
/// Where a saved cursor is put back.
pub const RESTORE: char = '\u{E0BF}';

/// The character (C) holds for code `n`, when it is one that is not
/// printed: a control code, a graphics block, a run of spaces, or a
/// position to print at. `None` for the rest, and for anything out of
/// range.
#[must_use]
pub fn character(n: i64) -> Option<char> {
    let u = u32::try_from(n.unsigned_abs()).ok()?;
    match n {
        -1023..=-1 => char::from_u32(PRINT_AT + u),
        0..=31 => char::from_u32(u),
        128..=191 => Some(sextant(u - 128)),
        192..=255 => char::from_u32(SPACES + u - 192),
        _ => None,
    }
}

/// The code a character `character` gives back. `None` for any other.
#[must_use]
pub fn code(c: char) -> Option<i64> {
    let u = u32::from(c);
    let block = match c {
        ' ' => return None,
        '▌' => 21,
        '▐' => 42,
        '█' => 63,
        '\u{1fb00}'..='\u{1fb3b}' => {
            let p = u - 0x1fb00 + 1;
            let p = if p >= 21 { p + 1 } else { p };
            if p >= 42 { p + 1 } else { p }
        }
        _ => {
            return match u {
                0..=31 => Some(i64::from(u)),
                0xE0C0..=0xE0FF => Some(i64::from(u - SPACES + 192)),
                0xE401..=0xE7FF => Some(-i64::from(u - PRINT_AT)),
                _ => None,
            };
        }
    };
    Some(128 + i64::from(block))
}

/// What a terminal is sent for `c`: the control codes as the terminal
/// codes that do the same, a run of spaces as the spaces, a position
/// as the code that moves there. `None` for a character that is
/// printed as it is.
#[must_use]
pub fn sent(c: char) -> Option<String> {
    let u = u32::from(c);
    let esc = |rest: &str| format!("\x1b[{rest}");
    Some(match u {
        0x08 => "\x08 \x08".to_string(),
        0x0A | 0x0D => "\n".to_string(),
        0x0E => esc("?25h"),
        0x0F => esc("?25l"),
        // 24 to 27: left, right, down and up.
        0x18..=0x1B => esc(&"DCBA"[(u - 0x18) as usize..][..1]),
        0x1C => esc("H"),
        0x1D => "\r".to_string(),
        0x1E => esc("K"),
        0x1F => esc("J"),
        0..=31 => String::new(),
        0xE0BE => esc("s"),
        0xE0BF => esc("u"),
        0xE0C0..=0xE0FF => " ".repeat((u - SPACES) as usize),
        0xE401..=0xE7FF => {
            let at = u - PRINT_AT;
            esc(&format!("{};{}H", at / 64 + 1, at % 64 + 1))
        }
        _ => return None,
    })
}

/// The block for the TRS-80's graphics bits: 1 the top left, 2 the top
/// right, 4 and 8 the middle, 16 and 32 the bottom. Unicode's sextants
/// number their six cells the same way, and leave out the four drawn
/// with characters of their own.
fn sextant(bits: u32) -> char {
    match bits {
        0 => ' ',
        21 => '▌',
        42 => '▐',
        63 => '█',
        p => {
            let skipped = u32::from(p > 21) + u32::from(p > 42);
            char::from_u32(0x1fb00 + p - 1 - skipped).unwrap_or(' ')
        }
    }
}
