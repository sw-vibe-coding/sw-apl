//! The board's row of overstrikes, held to the overstrike table.
//!
//! The browser board's APL mode has a button for each glyph a 2741
//! made by overstriking, read from `overstrikes` in
//! `data/board.json`. A reader who taps one should get a glyph that
//! means something in the mode they are in, so every entry is run
//! there; and the row should be the whole table in `data/glyphs.toml`,
//! less only what a mode does not have, so nothing is left off by
//! oversight.

use std::fs;
use std::path::PathBuf;

use apl_session::{Mode, Session};

/// The repository root: four levels up from this crate.
fn root() -> PathBuf {
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    here.join("../../../..").canonicalize().expect("the root")
}

/// The row, as the page reads it.
fn row() -> Vec<serde_json::Value> {
    let text = fs::read_to_string(root().join("data/board.json")).expect("data/board.json");
    let board: serde_json::Value = serde_json::from_str(&text).expect("JSON");
    board["overstrikes"]
        .as_array()
        .expect("an overstrikes list")
        .clone()
}

/// The overstrike table: each glyph and the mode it is limited to.
fn table() -> Vec<(String, Option<String>)> {
    let text = fs::read_to_string(root().join("data/glyphs.toml")).expect("data/glyphs.toml");
    let data: toml::Table = toml::from_str(&text).expect("TOML");
    let rows = data["overstrike"].as_array().expect("[[overstrike]]");
    let field = |r: &toml::Value, k: &str| r.get(k).and_then(|v| v.as_str()).map(String::from);
    rows.iter()
        .map(|r| (field(r, "glyph").expect("a glyph"), field(r, "mode")))
        .collect()
}

fn letter(mode: Mode) -> &'static str {
    if mode == Mode::A { "A" } else { "B" }
}

/// Whether the board offers `entry` in `mode`.
fn offered(entry: &serde_json::Value, mode: Mode) -> bool {
    entry["modes"]
        .as_str()
        .is_none_or(|m| m.contains(letter(mode)))
}

#[test]
fn every_overstrike_on_the_board_runs_in_its_modes() {
    for mode in [Mode::A, Mode::B] {
        let mut s = Session::default();
        s.ws.mode = mode;
        s.respond(")CLEAR");
        for entry in row().iter().filter(|e| offered(e, mode)) {
            let glyph = entry["insert"].as_str().expect("a glyph");
            let tried = entry["try"].as_str().expect("a line to try");
            assert!(tried.contains(glyph), "{glyph}: {tried:?} does not use it");
            let reply = s.respond(tried).lines.join("\n");
            assert!(
                !reply.contains("ERROR"),
                "{mode:?} {glyph}: {tried:?} answered:\n{reply}"
            );
        }
    }
}

#[test]
fn the_row_is_the_whole_overstrike_table() {
    let row = row();
    let on = |g: &str| row.iter().find(|e| e["insert"] == g);
    for (glyph, mode) in table() {
        let entry = on(&glyph).unwrap_or_else(|| panic!("{glyph} is not on the board"));
        if let (Some(m), Some(offered)) = (mode, entry["modes"].as_str()) {
            assert_eq!(offered, m, "{glyph} is offered outside its mode");
        }
    }
    let known: Vec<String> = table().into_iter().map(|(g, _)| g).collect();
    for entry in &row {
        let glyph = entry["insert"].as_str().expect("a glyph");
        assert!(
            known.iter().any(|g| g == glyph),
            "{glyph} is not an overstrike"
        );
    }
}

#[test]
fn a_mode_is_offered_only_the_overstrikes_it_has() {
    let shown = |mode| -> String {
        row()
            .iter()
            .filter(|e| offered(e, mode))
            .filter_map(|e| e["insert"].as_str().map(String::from))
            .collect()
    };
    let (a, b) = (shown(Mode::A), shown(Mode::B));
    assert!(a.contains('⌶') && !b.contains('⌶'), "{a} / {b}");
    assert!(b.contains('⍎') && b.contains('⍕') && !a.contains('⍎') && !a.contains('⍕'));
    assert!(
        a.starts_with('⍝') && b.starts_with('⍝'),
        "most used first: {a}"
    );
}
