//! One number to text.

use apl_value::Number;

/// Fixed form is used down to this power of ten; below it, E form
/// (matching classic APL's switch at five decimal places).
const EXP_LOW: i32 = -5;

/// How many digits a number is shown with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Precision {
    /// Significant digits (`)DIGITS`, `⎕PP`).
    pub digits: usize,
    /// How many digits a whole number is always shown in full with,
    /// whatever `digits` says. None in (A): APL\360 shows a whole
    /// number longer than its digits in E form. Ten in (B): the 5110
    /// applies its precision only to a whole number of more than ten
    /// digits (IBM 5110 APL Reference Manual, Chapter 5, under `⎕PP`).
    pub whole: usize,
}

impl From<usize> for Precision {
    /// `digits` significant digits, as APL\360 shows them.
    fn from(digits: usize) -> Self {
        Precision { digits, whole: 0 }
    }
}

/// Format `n` with at most `digits` significant digits, a whole number
/// in full up to the precision's `whole` digits.
#[must_use]
pub fn format_number(n: Number, precision: impl Into<Precision>) -> String {
    let Precision { digits, whole } = precision.into();
    let digits = digits.max(1);
    let full = u32::try_from(digits.max(whole)).unwrap_or(19);
    match n {
        Number::Int(i) if i.unsigned_abs() < 10u64.saturating_pow(full) => {
            i.to_string().replace('-', "¯")
        }
        _ => format_float(n.as_f64(), digits),
    }
}

/// Fixed form when the exponent lies in `EXP_LOW..digits`, else E
/// form; trailing zeros dropped; high minus for negatives.
fn format_float(x: f64, digits: usize) -> String {
    if x == 0.0 {
        return "0".to_string();
    }
    let sci = format!("{:.*e}", digits - 1, x);
    let (mantissa, exp) = sci.split_once('e').unwrap_or((&sci, "0"));
    let exp: i32 = exp.parse().unwrap_or(0);
    let high = i32::try_from(digits).unwrap_or(i32::MAX);
    let text = if (EXP_LOW..high).contains(&exp) {
        let decimals = usize::try_from(high - 1 - exp).unwrap_or(0);
        trim_zeros(&format!("{x:.decimals$}"))
    } else {
        format!("{}E{}", trim_zeros(mantissa), exp)
    };
    text.replace('-', "¯")
}

/// Drop trailing zeros after a decimal point, and the point itself.
fn trim_zeros(s: &str) -> String {
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s.to_string()
    }
}
