//! What each mode can do, asked as questions rather than by mode.
//!
//! Code that differs by mode asks the session's profile -- has it the
//! I-beams, a scan, a limit on functions -- and never which mode it is
//! in, so a mode is added here and not as an arm in every `match` on
//! `Mode` (docs/architecture.md, "Adding a mode").

use crate::mode::Mode;

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
const APL80_HAS: [Feature; 7] = [
    Feature::Scan,
    Feature::DefineByCommand,
    Feature::Auto,
    Feature::Trace,
    Feature::PrintPrecision,
    Feature::FormatStatement,
    Feature::Status,
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

impl Mode {
    /// What this mode can do.
    #[must_use]
    pub fn profile(self) -> Profile {
        let all = Profile {
            features: &[],
            cuts: &[],
            functions: usize::MAX,
            lines: usize::MAX,
        };
        match self {
            Mode::A => Profile {
                features: &[Feature::Ibeams, Feature::Settings],
                ..all
            },
            Mode::B => Profile {
                features: &[Feature::Scan, Feature::Latent],
                ..all
            },
            Mode::C => Profile {
                features: &APL80_HAS,
                cuts: &APL80,
                functions: 32,
                lines: 255,
            },
        }
    }
}
