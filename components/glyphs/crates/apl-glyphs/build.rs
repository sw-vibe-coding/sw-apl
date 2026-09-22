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
    overstrike: Vec<Overstrike>,
    underscored: Underscored,
}

#[derive(Deserialize)]
struct Primitive {
    glyph: String,
    name: String,
    monadic: String,
    dyadic: String,
    /// True when APL\360 lets the glyph take an axis bracket.
    #[serde(default)]
    axis: bool,
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
    /// The later mode that has the glyph, by letter: "B" for (B) '75.
    /// Empty for a glyph no mode of sw-apl has.
    #[serde(default)]
    mode: String,
}

/// A glyph formed by striking one character over another, as on a
/// 2741.
///
/// The TOML also carries a `source` for each pair, saying whether
/// the manual states it, it mirrors one the manual states, or the
/// composite is visibly its parts. That is for whoever reads the
/// table; serde ignores it here, and the generated const does not
/// need it.
#[derive(Deserialize)]
struct Overstrike {
    glyph: String,
    base: String,
    over: String,
}

/// The underscored alphabet, as a rule rather than twenty-six rows:
/// any of `letters` struck with `struck` gives that letter followed
/// by `mark`, the combining low line. Unicode has no precomposed
/// underscored Latin letter, so the glyph is two code points and one
/// column; `data/glyphs.toml` records why that was chosen.
#[derive(Deserialize)]
struct Underscored {
    letters: String,
    struck: String,
    mark: String,
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
    let struck = rows(
        tables
            .overstrike
            .iter()
            .map(|o| [quoted(&o.glyph), quoted(&o.base), quoted(&o.over)].join(", ")),
    );
    let later = rows(
        tables
            .later
            .iter()
            .map(|l| [quoted(&l.glyph), text(&l.name)].join(", ")),
    );
    let in_b: String = tables
        .later
        .iter()
        .filter(|l| l.mode == "B")
        .map(|l| l.glyph.as_str())
        .collect();
    [
        "// Generated from data/glyphs.toml by build.rs. Do not edit.".to_string(),
        konst(
            "The later glyphs the (B) '75 mode has, beyond APL\\360's: the\n             /// lexer takes them as primitives there and refuses them\n             /// everywhere else, as it refuses every glyph in `LATER`.",
            "MODE_B: &str",
            &text(&in_b),
        ),
        sets(&tables.primitive),
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
        table(
            "Glyphs struck from two characters on a 2741, as\n             /// `(glyph, base, over)`. Either order forms it: the two\n             /// land on one position, and no two pairs share their\n             /// characters. A pair not here is CHARACTER ERROR, which\n             /// is the manual's own answer -- \"Illegitimate\n             /// overstrike\" is what it gives as the cause.",
            "OVERSTRIKE",
            "(char, char, char)",
            tables.overstrike.len(),
            &struck,
        ),
        underscored(&tables.underscored),
    ]
    .join("\n\n")
}

/// The four glyph-set consts: every primitive, those with a monadic
/// meaning, those with a dyadic one, and those taking an axis.
fn sets(primitive: &[Primitive]) -> String {
    let glyphs = |keep: fn(&Primitive) -> bool| -> String {
        primitive
            .iter()
            .filter(|p| keep(p))
            .map(|p| p.glyph.as_str())
            .collect()
    };
    [
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
        konst(
            "Glyphs that accept an axis bracket. A glyph outside this\n             /// set followed by one is a SYNTAX ERROR: APL\\360 has no\n             /// such form.",
            "AXIS: &str",
            &text(&glyphs(|p| p.axis)),
        ),
    ]
    .join("\n\n")
}

/// The underscored alphabet, as the rule it is: the letters that
/// take an underbar, the character struck over them, and the
/// combining low line each is written with.
fn underscored(u: &Underscored) -> String {
    [
        konst(
            "The letters a 2741 could strike an underbar over. Each\n             /// gives a further character of the APL\\360 set, a letter\n             /// in its own right and distinct from the plain one: X and\n             /// X-underscored are two names.",
            "UNDERSCORED: &str",
            &text(&u.letters),
        ),
        konst(
            "The character struck over a letter to underscore it.",
            "UNDERBAR: char",
            &quoted(&u.struck),
        ),
        konst(
            "The combining low line an underscored letter is written\n             /// with. It follows its letter, continues a name, and takes\n             /// no column of its own -- see `columns`.",
            "UNDERSCORE: char",
            &quoted(&u.mark),
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
