//! Primitive and derived functions.

/// A primitive or derived function, as written before its argument.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Function {
    /// A primitive glyph.
    Prim(char),
    /// `f/` (or `f⌿` when `first`).
    Reduce { f: char, first: bool },
    /// `f\` (or `f⍀` when `first`).
    Scan { f: char, first: bool },
    /// `f.g` inner product.
    Inner { f: char, g: char },
    /// `∘.f` outer product.
    Outer { f: char },
    /// A function defined with the del form, by name.
    Defined(String),
}
