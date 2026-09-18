//! The limit a workspace is held against.
//!
//! The quota belongs to the session, not to the workspace: like the
//! console and the clock it is not written by `)SAVE`, so a workspace
//! saved under a large quota need not fit under a small one. That is
//! what happened on a real APL\360, and it is the reason `)COPY` of a
//! few names mattered as much as `)LOAD` of the lot.

use apl_value::{AplError, AplResult, ErrorKind};

/// The workspace size a session starts with, in bytes. Large enough
/// that ordinary work never meets it, small enough that a runaway
/// expression stops rather than taking the machine down with it.
pub const DEFAULT: usize = 1_048_576;

/// Bytes still free: the quota less what is held. A quota lowered
/// below what is already held reports nothing free rather than
/// going negative.
#[must_use]
pub fn free(quota: usize, used: usize) -> usize {
    quota.saturating_sub(used)
}

/// Whether `want` more bytes fit, once the `freed` bytes of whatever
/// is being replaced are given back.
///
/// # Errors
/// WS FULL when they do not.
pub fn room(quota: usize, used: usize, want: usize, freed: usize) -> AplResult<()> {
    if used.saturating_sub(freed) + want > quota {
        return Err(AplError::new(ErrorKind::WsFull));
    }
    Ok(())
}
