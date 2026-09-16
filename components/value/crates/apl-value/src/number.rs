//! `Number`: `Int` when the value is exactly integral and fits, else
//! `Float`. Arithmetic in other crates computes in `f64` and demotes
//! with [`Number::from_f64`].

/// The APL\360 comparison tolerance (fuzz), relative, not settable.
pub const FUZZ: f64 = 1e-13;

/// Largest magnitude that `f64` represents exactly as an integer.
const EXACT_INT_LIMIT: f64 = 9_007_199_254_740_992.0; // 2^53

/// A single APL numeric value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    /// Exact integer.
    Int(i64),
    /// IEEE double.
    Float(f64),
}

impl Number {
    /// Demote to `Int` when `x` is integral and exactly representable.
    #[must_use]
    pub fn from_f64(x: f64) -> Self {
        if x.fract() == 0.0 && x.abs() < EXACT_INT_LIMIT {
            // `as` is exact here: integral and below 2^53.
            #[allow(clippy::cast_possible_truncation)]
            Number::Int(x as i64)
        } else {
            Number::Float(x)
        }
    }

    /// Tolerant equality (quad-CT): equal when the difference is
    /// within `ct` times the larger magnitude. Zero has no slack.
    #[must_use]
    pub fn tolerant_eq(self, other: Number, ct: f64) -> bool {
        if let (Number::Int(lhs), Number::Int(rhs)) = (self, other) {
            return lhs == rhs;
        }
        let (lhs, rhs) = (self.as_f64(), other.as_f64());
        (lhs - rhs).abs() <= ct * lhs.abs().max(rhs.abs())
    }

    /// Exact integer `+`, `-`, `×` when both operands are `Int` and
    /// the result fits; `None` means "use the floating path".
    #[must_use]
    pub fn exact_int(f: char, l: Number, r: Number) -> Option<Number> {
        let (Number::Int(lhs), Number::Int(rhs)) = (l, r) else {
            return None;
        };
        match f {
            '+' => lhs.checked_add(rhs),
            '-' => lhs.checked_sub(rhs),
            '×' => lhs.checked_mul(rhs),
            _ => None,
        }
        .map(Number::Int)
    }

    /// The value as a double (lossless for `Int` below 2^53).
    #[must_use]
    pub fn as_f64(self) -> f64 {
        match self {
            // Precision loss above 2^53 is inherent to the f64 path.
            #[allow(clippy::cast_precision_loss)]
            Number::Int(i) => i as f64,
            Number::Float(f) => f,
        }
    }
}
