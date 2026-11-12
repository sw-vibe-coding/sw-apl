//! The TRS-80's codes and its 64-by-16 screen (docs/mode-c.md).

use apl_c80_screen::{Screen, character, code, sent};

#[test]
fn control_codes_are_held_as_themselves_and_sent_as_terminal_codes() {
    assert_eq!(character(28), Some('\u{1c}'));
    assert_eq!(code('\u{1c}'), Some(28));
    assert_eq!(
        sent('\u{1c}').as_deref(),
        Some("\x1b[H"),
        "28 homes the cursor"
    );
    assert_eq!(
        sent('\u{1f}').as_deref(),
        Some("\x1b[J"),
        "31 clears to the end of the frame"
    );
    assert_eq!(
        sent('\u{1e}').as_deref(),
        Some("\x1b[K"),
        "30 to the end of the line"
    );
    assert_eq!(
        sent('\u{1d}').as_deref(),
        Some("\r"),
        "29 to the start of the line"
    );
    assert_eq!(sent('\u{0d}').as_deref(), Some("\n"), "13 a new line");
    assert_eq!(sent('A'), None, "a printing character is itself");
}

#[test]
fn graphics_are_the_sextant_blocks() {
    assert_eq!(character(128), Some(' '));
    assert_eq!(character(129), Some('\u{1fb00}'), "top left");
    assert_eq!(character(128 + 21), Some('▌'), "the left column");
    assert_eq!(character(128 + 42), Some('▐'), "the right column");
    assert_eq!(character(191), Some('█'));
    assert_eq!(character(128 + 22), Some('\u{1fb14}'));
    for n in 129..=191 {
        assert_eq!(code(character(n).unwrap()), Some(n), "{n}");
    }
}

#[test]
fn space_compression_and_print_at() {
    let eight = character(200).unwrap();
    assert_eq!(sent(eight).as_deref(), Some("        "));
    assert_eq!(code(eight), Some(200));
    let at = character(-130).unwrap();
    assert_eq!(
        sent(at).as_deref(),
        Some("\x1b[3;3H"),
        "PRINT @ 130: row 2, column 2"
    );
    assert_eq!(code(at), Some(-130));
    assert_eq!(character(-1024), None);
    assert_eq!(character(256), None);
}

#[test]
fn the_screen_is_64_by_16_and_scrolls() {
    let mut s = Screen::default();
    s.print("HELLO\nWORLD");
    assert_eq!(s.rows()[0].trim_end(), "HELLO");
    assert_eq!(s.rows()[1].trim_end(), "WORLD");
    assert_eq!(s.cursor(), (1, 5));
    for n in 0..20 {
        s.print(&format!("\n{n}"));
    }
    assert_eq!(s.rows().len(), 16);
    assert_eq!(
        s.rows()[15].trim_end(),
        "19",
        "the last line printed is at the foot"
    );
    assert_eq!(s.rows()[0].chars().count(), 64);
}

#[test]
fn a_long_line_wraps_at_64() {
    let mut s = Screen::default();
    s.print(&"X".repeat(70));
    assert_eq!(s.rows()[0], "X".repeat(64));
    assert_eq!(s.rows()[1].trim_end(), "XXXXXX");
}

#[test]
fn the_cursor_codes_move_and_clear() {
    let mut s = Screen::default();
    s.print("AAAA\nBBBB\nCCCC");
    s.print("\x1b[H");
    assert_eq!(s.cursor(), (0, 0));
    s.print("\x1b[2;3HZ");
    assert_eq!(s.rows()[1].trim_end(), "BBZB");
    s.print("\x1b[K");
    assert_eq!(s.rows()[1].trim_end(), "BBZ");
    s.print("\x1b[J");
    assert_eq!(s.rows()[2].trim_end(), "", "to the end of the frame");
    assert_eq!(s.rows()[0].trim_end(), "AAAA");
    s.print("\x1b[H\x1b[J");
    assert!(
        s.rows().iter().all(|r| r.trim().is_empty()),
        "home and clear: CLS"
    );
}
