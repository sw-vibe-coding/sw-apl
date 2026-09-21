//! What a running session reads, and how often.

use std::cell::{Cell, RefCell};

/// Something a session can be stopped by.
///
/// A trait rather than a flag because where the flag lives is the
/// host's business. Natively it is an atomic another thread sets. In
/// a browser it is a slot in the shared channel, because while the
/// worker is running nothing can reach it by message -- a worker
/// reads its messages only when its thread is idle, and a running
/// loop never is.
pub trait Attention {
    /// Whether a stop was asked for since this was last called, which
    /// answers it: one attention stops one run.
    fn asked(&self) -> bool;
}

thread_local! {
    /// This thread's session's flag, if it has installed one.
    static HELD: RefCell<Option<Box<dyn Attention>>> = const { RefCell::new(None) };

    /// How many strided polls since the flag was last really read.
    static SINCE: Cell<usize> = const { Cell::new(0) };
}

/// How many calls to `polled` pass between real reads of the flag.
///
/// The one place the granularity is set. A primitive's inner loop
/// calls `polled` every time round, and almost always that is an
/// increment and a compare; one call in this many reads the flag.
/// Small enough that a loop notices within microseconds, large
/// enough that the read vanishes into the arithmetic around it.
pub const STRIDE: usize = 1024;

/// Install this session's flag on this thread, replacing whatever an
/// earlier session on the same thread left. A service thread
/// outlives the session on it, and a fresh session must not be
/// stopped by an attention nobody answered.
pub fn attend(flag: Box<dyn Attention>) {
    flag.asked();
    HELD.with(|held| *held.borrow_mut() = Some(flag));
    SINCE.with(|since| since.set(0));
}

/// Whether this session has been asked to stop. For a place that
/// runs rarely -- between the lines of a defined function -- where a
/// real read every time costs nothing.
#[must_use]
pub fn asked() -> bool {
    HELD.with(|held| held.borrow().as_ref().is_some_and(|flag| flag.asked()))
}

/// Whether this session has been asked to stop, for a hot loop: it
/// really reads the flag only once in `STRIDE` calls.
///
/// This is what keeps a tight loop loose. The owner's decision was
/// that a loop which cannot be stopped is worse than one which runs
/// slower, and this is the whole of what that costs.
#[must_use]
pub fn polled() -> bool {
    let due = SINCE.with(|since| {
        let next = since.get() + 1;
        since.set(next % STRIDE);
        next == STRIDE
    });
    due && asked()
}
