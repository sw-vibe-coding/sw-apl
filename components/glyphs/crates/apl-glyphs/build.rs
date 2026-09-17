//! Generates the glyph tables from `data/glyphs.toml` at the repo
//! root, the single source of truth for every glyph sw-apl knows.
//! The output is `include!`d by `lib.rs`.

use std::path::Path;
use std::{env, fs};

use serde::Deserialize;

#[derive(Deserialize)]
struct Tables {
    primitive: Vec<Primitive>,
    syntax: Vec<Syntax>,
    lookalike: Vec<Lookalike>,
    later: Vec<Later>,
}

#[derive(Deserialize)]
struct Primitive {
    glyph: String,
    name: String,
    monadic: String,
    dyadic: String,
}

#[derive(Deserialize)]
struct Syntax {
    glyph: String,
    name: String,
    meaning: String,
}

#[derive(Deserialize)]
struct Lookalike {
    typed: String,
    meant: String,
}

#[derive(Deserialize)]
struct Later {
    glyph: String,
    name: String,
}

fn main() {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../data/glyphs.toml");
    println!("cargo:rerun-if-changed={}", source.display());
    println!("cargo:rerun-if-changed=build.rs");
    let text = fs::read_to_string(&source).expect("read data/glyphs.toml");
    let tables: Tables = toml::from_str(&text).expect("parse data/glyphs.toml");
    let out = Path::new(&env::var("OUT_DIR").expect("OUT_DIR")).join("glyphs.rs");
    fs::write(out, render(&tables)).expect("write glyphs.rs");
}

/// The generated Rust: one const per table.
fn render(tables: &Tables) -> String {
    let glyphs = |keep: fn(&Primitive) -> bool| -> String {
        tables
            .primitive
            .iter()
            .filter(|p| keep(p))
            .map(|p| p.glyph.as_str())
            .collect()
    };
    let prims = rows(tables.primitive.iter().map(|p| {
        [
            quoted(&p.glyph),
            text(&p.name),
            text(&p.monadic),
            text(&p.dyadic),
        ]
        .join(", ")
    }));
    let syntax = rows(
        tables
            .syntax
            .iter()
            .map(|s| [quoted(&s.glyph), text(&s.name), text(&s.meaning)].join(", ")),
    );
    let looks = rows(
        tables
            .lookalike
            .iter()
            .map(|l| [quoted(&l.typed), quoted(&l.meant)].join(", ")),
    );
    let later = rows(
        tables
            .later
            .iter()
            .map(|l| [quoted(&l.glyph), text(&l.name)].join(", ")),
    );
    [
        "// Generated from data/glyphs.toml by build.rs. Do not edit.".to_string(),
        konst(
            "Every primitive function and operator glyph.",
            "PRIMITIVES: &str",
            &text(&glyphs(|_| true)),
        ),
        konst(
            "Glyphs with a monadic meaning.",
            "MONADIC: &str",
            &text(&glyphs(|p| !p.monadic.is_empty())),
        ),
        konst(
            "Glyphs with a dyadic meaning.",
            "DYADIC: &str",
            &text(&glyphs(|p| !p.dyadic.is_empty())),
        ),
        table(
            "Each primitive: glyph, name, monadic and dyadic meanings.",
            "PRIMITIVE_NAMES",
            "(char, &str, &str, &str)",
            tables.primitive.len(),
            &prims,
        ),
        table(
            "Punctuation and sentinels, with names and meanings.",
            "SYNTAX",
            "(char, &str, &str)",
            tables.syntax.len(),
            &syntax,
        ),
        table(
            "Characters often typed in place of an APL glyph.",
            "LOOKALIKE",
            "(char, char)",
            tables.lookalike.len(),
            &looks,
        ),
        table(
            "Glyphs from later APLs, named as they are there.",
            "LATER",
            "(char, &str)",
            tables.later.len(),
            &later,
        ),
    ]
    .join("\n\n")
}

/// A `pub const NAME: TYPE = VALUE;` item with a doc comment.
fn konst(doc: &str, declaration: &str, value: &str) -> String {
    format!("/// {doc}\npub const {declaration} = {value};")
}

/// A `pub const NAME: [TYPE; N] = [...];` item with a doc comment.
fn table(doc: &str, name: &str, element: &str, count: usize, entries: &str) -> String {
    konst(
        doc,
        &format!("{name}: [{element}; {count}]"),
        &format!("[{entries}]"),
    )
}

/// A character literal for the one character of `s`.
fn quoted(s: &str) -> String {
    format!("{:?}", s.chars().next().expect("one character per glyph"))
}

/// A string literal for `s`.
fn text(s: &str) -> String {
    format!("{s:?}")
}

/// Parenthesised, comma-terminated table entries.
fn rows(items: impl Iterator<Item = String>) -> String {
    items.fold(String::new(), |mut out, row| {
        out.push('(');
        out.push_str(&row);
        out.push_str("),");
        out
    })
}
