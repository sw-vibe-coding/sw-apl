//! The bracketed editor commands, and the line numbers they name.
//! Numbers are held in thousandths so a fractional insert is exact.

use apl_value::{AplError, AplResult, ErrorKind};

/// What a bracketed command asks the editor to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edit {
    /// `[n]`: the next line typed becomes line n. Line 0 is the header.
    At(i64),
    /// `[n⎕]`: display from line n. `[⎕]` is line 0, the whole
    /// function, header and closing del included.
    Show(i64),
    /// `[∆n]`: delete line n.
    Delete(i64),
}

/// The bracketed command a line starts with and whatever followed it.
/// `None` when the line does not start with a bracket, which makes it
/// ordinary text for the line the prompt is offering.
///
/// # Errors
/// DEFN ERROR for an unclosed bracket, or one holding something that
/// is not an editor command.
pub fn parse_edit(line: &str) -> AplResult<Option<(Edit, &str)>> {
    let Some(body) = line.strip_prefix('[') else {
        return Ok(None);
    };
    let Some(close) = body.find(']') else {
        return Err(AplError::new(ErrorKind::Defn));
    };
    let (inside, rest) = (body[..close].trim(), &body[close + 1..]);
    let edit = if let Some(n) = inside.strip_prefix('∆') {
        Edit::Delete(line_number(n)?)
    } else if let Some(n) = inside.strip_suffix('⎕') {
        Edit::Show(if n.trim().is_empty() {
            0
        } else {
            line_number(n)?
        })
    } else {
        Edit::At(line_number(inside)?)
    };
    Ok(Some((edit, rest)))
}

/// A line number in thousandths: `2` is 2000 and `2.1` is 2100. Three
/// decimal places is as fine as an insert goes.
///
/// # Errors
/// DEFN ERROR for anything that is not such a number.
pub fn line_number(text: &str) -> AplResult<i64> {
    let text = text.trim();
    let (whole, frac) = text.split_once('.').unwrap_or((text, ""));
    let digits = |s: &str| s.chars().all(|c| c.is_ascii_digit());
    if whole.is_empty() || frac.len() > 3 || !digits(whole) || !digits(frac) {
        return Err(AplError::new(ErrorKind::Defn));
    }
    let scale = 10i64.pow(3 - u32::try_from(frac.len()).unwrap_or(3));
    let whole: i64 = whole.parse().map_err(|_| AplError::new(ErrorKind::Defn))?;
    let frac: i64 = frac.parse().unwrap_or(0);
    Ok(whole * 1000 + frac * scale)
}

/// A line number as the editor prints it: 2000 is `2`, 2100 is `2.1`.
#[must_use]
pub fn format_line(n: i64) -> String {
    let (whole, frac) = (n / 1000, n % 1000);
    if frac == 0 {
        return whole.to_string();
    }
    format!("{whole}.{frac:03}")
        .trim_end_matches('0')
        .to_string()
}

/// The number the prompt offers after `n`: one step at the grain `n`
/// itself uses, so 2 is followed by 3 and 2.1 by 2.2.
#[must_use]
pub fn advance(n: i64) -> i64 {
    let step = [1000, 100, 10, 1]
        .into_iter()
        .find(|s| n % s == 0)
        .unwrap_or(1);
    n + step
}
