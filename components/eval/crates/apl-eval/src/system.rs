//! System variables (`⎕IO`, `⎕CT`): read and assign.

use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

use crate::workspace::Workspace;

/// The value of a system variable by its name without the quad.
///
/// # Errors
/// VALUE ERROR for an unknown name.
pub fn get_system(ws: &Workspace, name: &str) -> AplResult<Array> {
    match name {
        "IO" => Ok(Array::scalar(Number::Int(ws.io))),
        "CT" => Ok(Array::scalar(Number::from_f64(ws.ct))),
        _ => Err(AplError::new(ErrorKind::Value)),
    }
}

/// Assign a system variable.
///
/// # Errors
/// VALUE ERROR for an unknown name; DOMAIN ERROR for a bad value.
pub fn set_system(ws: &mut Workspace, name: &str, value: &Array) -> AplResult<()> {
    match (name, scalar_number(value)?) {
        ("IO", Number::Int(io @ (0 | 1))) => ws.io = io,
        ("CT", n) if n.as_f64() >= 0.0 => ws.ct = n.as_f64(),
        ("IO" | "CT", _) => return Err(AplError::new(ErrorKind::Domain)),
        _ => return Err(AplError::new(ErrorKind::Value)),
    }
    Ok(())
}

/// The single number in a scalar (or one-element) numeric array.
fn scalar_number(value: &Array) -> AplResult<Number> {
    match &value.data {
        Data::Num(v) if v.len() == 1 && value.shape.len() <= 1 => Ok(v[0]),
        _ => Err(AplError::new(ErrorKind::Domain)),
    }
}
