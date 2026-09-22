//! The atomic vector against the IBM 5110 manual's Appendix B.

use apl_sysvars::atomic_vector;
use apl_value::Data;

fn chars() -> Vec<char> {
    match atomic_vector().data {
        Data::Char(chars) => chars,
        Data::Num(_) => panic!("the atomic vector is characters"),
    }
}

/// The character at a 1-origin position, as the manual counts.
fn at(n: usize) -> char {
    chars()[n - 1]
}

#[test]
fn it_holds_256_characters_each_once() {
    let mut all = chars();
    assert_eq!(all.len(), 256);
    all.sort_unstable();
    all.dedup();
    assert_eq!(all.len(), 256, "no character twice");
}

#[test]
fn each_run_lands_where_the_manual_puts_it() {
    let expected = [
        (15, '['),
        (23, '→'),
        (26, '¨'),
        (47, '⍵'),
        (48, ','),
        (62, '_'),
        (64, '⌶'),
        (66, '⎕'),
        (71, '⍝'),
        (78, '⍕'),
        (79, '⍎'),
        (83, '$'),
        (87, 'A'),
        (112, 'Z'),
        (113, '∆'),
        (140, '⍙'),
        (141, '0'),
        (150, '9'),
        (153, ' '),
        (156, '∇'),
        (161, '⍫'),
        (173, '%'),
        (187, 'a'),
        (212, 'z'),
        (214, '}'),
        (221, '`'),
    ];
    for (n, c) in expected {
        assert_eq!(at(n), c, "position {n}");
    }
}

#[test]
fn a_position_with_no_character_of_sw_apls_is_private_use() {
    // Reserved (1), an underscored letter (114), line feed (160).
    for n in [1, 114, 160, 256] {
        let c = u32::from(at(n));
        assert_eq!(c, 0xE000 + u32::try_from(n - 1).unwrap(), "position {n}");
    }
}
