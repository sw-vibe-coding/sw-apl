//! Monadic format: the display, as characters.

use apl_display::{Precision, format_array};
use apl_value::{AplResult, Array, Data};

/// `⍕r`: a character array identical in appearance to the display of
/// `r` at precision `digits`. A scalar or a vector gives a vector, and
/// anything of higher rank an array of that rank, its last axis the
/// width of a displayed line. Characters are themselves.
///
/// # Errors
/// None the display cannot meet; an array too big to hold is WS FULL
/// upstream.
pub fn monadic(r: &Array, digits: Precision) -> AplResult<Array> {
    if matches!(r.data, Data::Char(_)) {
        return Ok(r.clone());
    }
    let Some((&cols, lead)) = r.shape.split_last() else {
        return Ok(line(r, digits));
    };
    if lead.is_empty() {
        return Ok(line(r, digits));
    }
    // The planes of a higher rank are aligned as one matrix, so that
    // matrix, displayed without wrapping, is every line there is.
    let rows = lead.iter().product::<usize>();
    let flat = Array::new(vec![rows, cols], r.data.clone())?;
    let lines = format_array(&flat, digits, usize::MAX);
    let width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    let chars: Vec<char> = lines
        .iter()
        .flat_map(|l| l.chars().chain(std::iter::repeat(' ')).take(width))
        .collect();
    Array::new([lead, &[width]].concat(), Data::Char(chars))
}

/// A scalar or a vector: the one line of its display.
fn line(r: &Array, digits: Precision) -> Array {
    let text: Vec<char> = format_array(r, digits, usize::MAX)
        .concat()
        .chars()
        .collect();
    Array {
        shape: vec![text.len()],
        data: Data::Char(text),
    }
}
