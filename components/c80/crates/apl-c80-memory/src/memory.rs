//! Peek, poke and call.

use apl_c80_machine::{RAM, SCREEN};
use apl_c80_screen::{RESTORE, SAVE, character};
use apl_c80_status::glyph;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};
use apl_workspace::{Output, Workspace};

use crate::entry::{ints, opened, paired};

/// `u r`: the byte at each address in `r`.
///
/// # Errors
/// NONCE ERROR without `)RAM`; DOMAIN ERROR for an address that is not
/// the screen's or the RAM's -- the ROM, the keyboard and the devices
/// are not modeled.
pub(crate) fn peek(ws: &mut Workspace, r: &Array) -> AplResult<Array> {
    let at = ints(r)?;
    let machine = opened(ws)?;
    let bytes = at
        .iter()
        .map(|a| machine.read(*a).map(|b| Number::Int(i64::from(b))))
        .collect::<Option<_>>()
        .ok_or_else(|| AplError::new(ErrorKind::Domain))?;
    Array::new(r.shape.clone(), Data::Num(bytes))
}

/// `l u r`: each byte in `l` put at its address in `r`, and the bytes
/// given back. A byte on the screen is shown there at once, the cursor
/// saved first and put back after, as the TRS-80's screen memory is
/// the screen. Nothing is put anywhere unless everything can be.
///
/// # Errors
/// As `peek`, and as `checked`; LENGTH ERROR for unmatched lengths.
pub(crate) fn poke(ws: &mut Workspace, l: &Array, r: &Array) -> AplResult<Array> {
    let pairs = paired(&ints(l)?, &ints(r)?)?;
    opened(ws)?;
    let shown = pairs
        .iter()
        .map(|(byte, at)| checked(*byte, *at))
        .collect::<AplResult<Vec<_>>>()?;
    for ((byte, at), shown) in pairs.into_iter().zip(shown) {
        let machine = opened(ws)?;
        let Some(text) = shown else {
            machine.ram.insert(at, u8::try_from(byte).unwrap_or(0));
            continue;
        };
        machine.screen.print(&apl_c80_keys::shown(&text));
        ws.output.push(Output::Bare(text));
    }
    Ok(l.clone())
}

/// What poking `byte` at `at` shows: nothing in RAM; on the screen the
/// cursor saved, moved there, the character, and the cursor put back.
///
/// # Errors
/// DOMAIN ERROR for a byte that is not one, an address the model has
/// not got, or a code the screen does not print.
fn checked(byte: i64, at: i64) -> AplResult<Option<String>> {
    let domain = || AplError::new(ErrorKind::Domain);
    let n = u8::try_from(byte).map_err(|_| domain())?;
    if RAM.contains(&at) {
        return Ok(None);
    }
    let pos = at - SCREEN.start();
    let shown = match n {
        32..=127 => glyph(n),
        128..=191 => character(byte),
        _ => None,
    };
    let to = if pos == 0 {
        Some('\u{1c}')
    } else {
        character(-pos)
    };
    match (SCREEN.contains(&at), to, shown) {
        (true, Some(to), Some(c)) => Ok(Some([SAVE, to, c, RESTORE].iter().collect())),
        _ => Err(domain()),
    }
}

/// `l # r`: Call the ROM routine at `l` with `r` in its register, as
/// the manual shows two: 51 prints the character with code `r` and
/// gives `r` back, 43 reads a key as INKEY$ does -- and as no key is
/// ever waiting while a line runs, gives 0.
///
/// # Errors
/// NONCE ERROR without `)RAM`; LENGTH ERROR for more than one address
/// or value; DOMAIN ERROR for any other address, or a value that is not
/// a byte.
pub(crate) fn call(ws: &mut Workspace, l: &Array, r: &Array) -> AplResult<Array> {
    let (at, n) = (ints(l)?, ints(r)?);
    let ([at], [n]) = (at.as_slice(), n.as_slice()) else {
        return Err(AplError::new(ErrorKind::Length));
    };
    opened(ws)?;
    let domain = || AplError::new(ErrorKind::Domain);
    let byte = u8::try_from(*n).map_err(|_| domain())?;
    match at {
        51 => {
            let c = character(*n).or_else(|| glyph(byte)).ok_or_else(domain)?;
            ws.output.push(Output::Bare(c.to_string()));
            Ok(r.clone())
        }
        43 => Ok(Array::scalar(Number::Int(0))),
        _ => Err(domain()),
    }
}
