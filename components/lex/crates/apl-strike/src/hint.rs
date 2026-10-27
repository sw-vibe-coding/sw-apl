//! What to say when a strike forms no glyph.

use apl_value::{OVERSTRIKE, UNDERBAR, UNDERSCORE, UNDERSCORED};

/// The note for `base` struck with `over` when the pair forms nothing:
/// that it formed nothing, and what `base` does form in the mode whose
/// extra pairs are `also`, so that a near miss -- the lamp tried as cap
/// and jot rather than cap and circle -- points at the right key.
///
/// Built from the table the strike reads, so it cannot offer a pair
/// the mode does not form.
#[must_use]
pub fn near_miss(base: char, over: char, also: &[(char, char, char)]) -> String {
    let forms = partners(base, also);
    let miss = format!("{base} with {over} forms no glyph.");
    if forms.is_empty() {
        return miss;
    }
    format!("{miss} {base} forms {}.", forms.join(", "))
}

/// Each glyph `base` forms, as "glyph with other", in table order,
/// then what the underbar rule adds.
fn partners(base: char, also: &[(char, char, char)]) -> Vec<String> {
    let mut forms: Vec<String> = OVERSTRIKE
        .iter()
        .chain(also)
        .filter_map(|&(glyph, a, b)| match (a == base, b == base) {
            (true, _) => Some(format!("{glyph} with {b}")),
            (_, true) => Some(format!("{glyph} with {a}")),
            _ => None,
        })
        .collect();
    if UNDERSCORED.contains(base) {
        forms.push(format!("{base}{UNDERSCORE} with {UNDERBAR}"));
    } else if base == UNDERBAR {
        forms.push("and an underscored letter with any letter".to_string());
    }
    forms
}
