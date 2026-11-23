//! A pattern over an array of numbers.

use std::ops::Range;

use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

use crate::field::render;
use apl_d83_field::fields;

/// `⎕FC` in a clear workspace: the decimal point, the comma, the fill
/// of an 8, the fill of a field too small (0: DOMAIN ERROR instead),
/// and the character printed as a blank.
pub const CLEAR: &str = ".,*0_";

/// `l⍕r`, `l` a pattern: each row of `r` laid out by it. With one
/// field the pattern is repeated, a copy for each number of a row;
/// with more, there is a field for each.
///
/// # Errors
/// RANK ERROR for a pattern above a vector; DOMAIN ERROR for numbers
/// that are characters, a pattern with no field, or a number too
/// wide for its field when `⎕FC[4]` is 0; LENGTH ERROR when there are
/// several fields and not one for each number of a row.
pub fn picture(l: &Array, r: &Array, fc: &[char]) -> AplResult<Array> {
    let (pattern, ns, fields) = checked(l, r)?;
    let fc: Vec<char> = CLEAR
        .chars()
        .enumerate()
        .map(|(i, c)| fc.get(i).copied().unwrap_or(c))
        .collect();
    let cols = r.shape.last().copied().unwrap_or(1);
    let mut data = Vec::new();
    for row in ns.chunks(cols.max(1)) {
        data.extend(line(pattern, &fields, row, &fc)?);
    }
    let copies = if fields.len() == 1 { cols } else { 1 };
    let mut shape = r.shape[..r.shape.len().saturating_sub(1)].to_vec();
    shape.push(pattern.len() * copies);
    Ok(Array {
        shape,
        data: Data::Char(data),
    })
}

/// A pattern, its numbers and the pattern's fields.
type Checked<'a> = (&'a [char], &'a [Number], Vec<Range<usize>>);

/// The pattern, the numbers and the pattern's fields, when they can
/// make a picture.
fn checked<'a>(l: &'a Array, r: &'a Array) -> AplResult<Checked<'a>> {
    let err = |kind| Err(AplError::new(kind));
    let (Data::Char(pattern), Data::Num(ns)) = (&l.data, &r.data) else {
        return err(ErrorKind::Domain);
    };
    if l.shape.len() > 1 {
        return err(ErrorKind::Rank);
    }
    let fields = fields(pattern);
    let cols = r.shape.last().copied().unwrap_or(1);
    match fields.len() {
        0 => err(ErrorKind::Domain),
        n if n > 1 && n != cols => err(ErrorKind::Length),
        _ => Ok((pattern, ns, fields)),
    }
}

/// One row: the pattern once for each number with one field, or once
/// with a number in each field.
fn line(
    pattern: &[char],
    fields: &[Range<usize>],
    row: &[Number],
    fc: &[char],
) -> AplResult<Vec<char>> {
    if fields.len() > 1 {
        return filled(pattern, fields, row, fc);
    }
    let mut out = Vec::new();
    for x in row {
        out.extend(filled(pattern, fields, std::slice::from_ref(x), fc)?);
    }
    Ok(out)
}

/// The pattern with `xs` in its fields, the blank of `⎕FC[5]` blank.
fn filled(
    pattern: &[char],
    fields: &[Range<usize>],
    xs: &[Number],
    fc: &[char],
) -> AplResult<Vec<char>> {
    let mut out: Vec<char> = pattern
        .iter()
        .map(|&c| if c == fc[4] { ' ' } else { c })
        .collect();
    for (field, x) in fields.iter().zip(xs) {
        let shown = match render(&pattern[field.clone()], x.as_f64(), fc) {
            Ok(shown) => shown,
            Err(_) if fc[3] != '0' => vec![fc[3]; field.len()],
            Err(_) => return Err(AplError::new(ErrorKind::Domain)),
        };
        out[field.clone()].copy_from_slice(&shown);
    }
    Ok(out)
}
