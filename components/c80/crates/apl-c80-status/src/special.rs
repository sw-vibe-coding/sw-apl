//! The codes APL80 obeys rather than prints (the manual's "The # and s
//! Operators").

use apl_value::{AplError, AplResult, ErrorKind};
use apl_workspace::{Env, Workspace};

/// Obey code `n` if it is one APL80 gives an action, and say so: 5 and
/// 9, a printer's hard copy on and off, do nothing with none
/// connected, as the manual says; 6 randomizes the random link from
/// the clock; 11 starts it again; 18 sets the clock `s0` reads back to
/// nothing.
///
/// # Errors
/// NONCE ERROR for zone printing, 3, 7, 15 and 255, and for stopping
/// and starting the clock, 16 and 17, which (C) does not do.
pub(crate) fn obey(ws: &mut Workspace, n: i64) -> AplResult<bool> {
    match n {
        5 | 9 => {}
        6 => {
            let now = (ws.clock)().now.rem_euclid(2_147_483_646);
            ws.saved.env.link = u64::try_from(now).unwrap_or(0) + 1;
        }
        11 => ws.saved.env.link = Env::default().link,
        18 => ws.signed_on = (ws.clock)().now,
        3 | 7 | 15 | 16 | 17 | 255 => return Err(AplError::new(ErrorKind::Nonce)),
        _ => return Ok(false),
    }
    Ok(true)
}
