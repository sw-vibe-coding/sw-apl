//! One number to text.

use apl_value::Number;

/// Exponent form is used at or above this power of ten...
const EXP_HIGH: i32 = 10;
/// ...and below this one (matching classic APL's switch to E form).
const EXP_LOW: i32 = -5;

/// Format `n` with at most `pp` significant digits.
#[must_use]
pub fn format_number(n: Number, pp: usize) -> String {
    match n {
        Number::Int(i) => i.to_string().replace('-', "¯"),
        Number::Float(x) => format_float(x, pp.max(1)),
    }
}

fn format_float(x: f64, pp: usize) -> String {
    if x == 0.0 {
        return "0".to_string();
    }
    let sci = format!("{:.*e}", pp - 1, x);
    let (mantissa, exp) = sci.split_once('e').unwrap_or((&sci, "0"));
    let exp: i32 = exp.parse().unwrap_or(0);
    let text = if (EXP_LOW..EXP_HIGH).contains(&exp) {
        let decimals = usize::try_from(i32::try_from(pp).unwrap_or(0) - 1 - exp).unwrap_or(0);
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
