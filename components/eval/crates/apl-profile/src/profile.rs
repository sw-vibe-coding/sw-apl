//! The features and cuts, and the profile of each mode.

/// Something only some modes have, beyond the shared core.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Feature {
    /// The I-beams, `⌶N`, which the 5100 family replaced with system
    /// variables and functions.
    Ibeams,
    /// The groups and the settings commands `)ORIGIN`, `)DIGITS` and
    /// `)WIDTH`.
    Settings,
    /// Scan, which APL\360 has not got.
    Scan,
    /// A latent expression, run as `⍎⎕LX` when a workspace is loaded.
    Latent,
    /// Functions defined by `)DEF` and changed by `)EDIT`, not the
    /// del, and workspaces written so, which run in this mode alone.
    DefineByCommand,
    /// `)AUTO`, a latent expression run as typed when a workspace is
    /// loaded, and `)EXEC`, which runs it.
    Auto,
    /// `)TRON` and `)TROFF`, a trace of the lines as they run.
    Trace,
    /// `)PS` and `)PD`, six digits printed or fifteen.
    PrintPrecision,
    /// A statement that begins with `⍕` and sets the numeric format.
    FormatStatement,
    /// `⌶` as the status operator: the clock, the file pointers, the
    /// line counter and the free space.
    Status,
    /// APL80's characters, typed and shown: the shifted letters, the
    /// substitutes, round-bracket indexing and the rest.
    Apl80Keys,
}

/// Something of the core a mode leaves out. APL80's manual lists its
/// own under "Limitations" (docs/mode-c.md).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cut {
    /// Matrix inverse and divide, `⌹`.
    Domino,
    /// Lamination: catenation on a fractional axis.
    Laminate,
    /// The diagonal case of dyadic transpose: an axis named twice.
    Diagonal,
    /// The hyperbolic circle functions, `5○` to `7○` and their inverses.
    Hyperbolic,
    /// Encode and decode with an argument above a vector.
    MatrixRadix,
    /// Factorial and binomial of a number that is not whole.
    FractionalFactorial,
    /// Two specifications in one statement, as `X←Y←9`, or one whose
    /// value is used, as `÷X←9`.
    MultipleSpecification,
}

/// What a mode can do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Profile {
    features: &'static [Feature],
    cuts: &'static [Cut],
    /// The most functions a workspace may hold.
    pub functions: usize,
    /// The most lines a function may have.
    pub lines: usize,
}

/// What APL80's disk version has beyond the core.
const APL80_HAS: [Feature; 8] = [
    Feature::Scan,
    Feature::DefineByCommand,
    Feature::Auto,
    Feature::Trace,
    Feature::PrintPrecision,
    Feature::FormatStatement,
    Feature::Status,
    Feature::Apl80Keys,
];

/// Everything APL80's disk version leaves out.
const APL80: [Cut; 7] = [
    Cut::Domino,
    Cut::Laminate,
    Cut::Diagonal,
    Cut::Hyperbolic,
    Cut::MatrixRadix,
    Cut::FractionalFactorial,
    Cut::MultipleSpecification,
];

impl Profile {
    /// Whether the mode has `feature`.
    #[must_use]
    pub fn has(self, feature: Feature) -> bool {
        self.features.contains(&feature)
    }

    /// Whether the mode leaves `cut` out.
    #[must_use]
    pub fn cuts(self, cut: Cut) -> bool {
        self.cuts.contains(&cut)
    }
}

/// Nothing beyond the core, and no limits.
const CORE: Profile = Profile {
    features: &[],
    cuts: &[],
    functions: usize::MAX,
    lines: usize::MAX,
};

/// (A) '70: the I-beams and the settings commands.
pub const A70: Profile = Profile {
    features: &[Feature::Ibeams, Feature::Settings],
    ..CORE
};

/// (B) '75: scan and a latent expression.
pub const B75: Profile = Profile {
    features: &[Feature::Scan, Feature::Latent],
    ..CORE
};

/// (C) '80: what APL80's disk version has and leaves out, and its
/// limits of 32 functions and 255 lines.
pub const C80: Profile = Profile {
    features: &APL80_HAS,
    cuts: &APL80,
    functions: 32,
    lines: 255,
};
