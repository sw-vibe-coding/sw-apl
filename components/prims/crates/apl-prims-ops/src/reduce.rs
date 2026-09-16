//! `f/r`: fold each last-axis row from the right.

use apl_prims_scalar::{apply_dyadic, numbers};
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

/// Reduce along the last axis; a scalar reduces to itself.
///
/// # Errors
/// NOT IMPLEMENTED for functions without a scalar dyadic form;
/// DOMAIN ERROR from the function.
pub fn reduce(f: char, r: &Array) -> AplResult<Array> {
    let data = numbers(r)?;
    let Some((&last, lead)) = r.shape.split_last() else {
        return Ok(r.clone());
    };
    let rows = lead.iter().product::<usize>();
    let out = (0..rows)
        .map(|i| fold_row(f, &data[i * last..(i + 1) * last]))
        .collect::<AplResult<Vec<_>>>()?;
    Array::new(lead.to_vec(), Data::Num(out))
}

fn fold_row(f: char, row: &[Number]) -> AplResult<Number> {
    let mut rest = row.iter().rev();
    let Some(&start) = rest.next() else {
        return identity(f);
    };
    rest.try_fold(start, |acc, &x| apply_dyadic(f, x, acc))
}

/// The value of reducing an empty vector.
fn identity(f: char) -> AplResult<Number> {
    Ok(match f {
        '+' | '-' | '|' => Number::Int(0),
        '×' | '÷' | '*' => Number::Int(1),
        '⌈' => Number::Float(f64::MIN),
        '⌊' => Number::Float(f64::MAX),
        _ => return Err(AplError::new(ErrorKind::NotImplemented)),
    })
}
