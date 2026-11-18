//! A flag another thread can set, which is what a native host uses,
//! and a wait that a stop cuts short.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::attention::{Attention, HELD, KEPT};

/// An attention flag that may be set from any thread and read by the
/// session that installed it. Cloning shares it: the session keeps
/// one and whoever may interrupt it -- a signal handler, the thread
/// listening on a connection -- keeps another.
#[derive(Debug, Clone, Default)]
pub struct Flag(Arc<AtomicBool>);

impl Flag {
    /// Ask the session holding this flag to stop. Safe from a signal
    /// handler: it only stores.
    pub fn ask(&self) {
        self.0.store(true, Ordering::Relaxed);
    }
}

impl Attention for Flag {
    fn asked(&self) -> bool {
        self.0.swap(false, Ordering::Relaxed)
    }
}

/// Wait `seconds`, or until this session is asked to stop, and say
/// how long it waited: `⎕DL`. A stop that ends the wait is kept for
/// the run, which stops at the end of its line. A thread with no
/// session's flag -- a test, a transcript -- does not wait: it
/// reports the wait it was asked for, so what it prints is the same
/// every time.
#[must_use]
pub fn delay(seconds: f64) -> f64 {
    let seconds = seconds.max(0.0);
    if HELD.with(|held| held.borrow().is_none()) {
        return seconds;
    }
    let start = now();
    while now() - start < seconds {
        if crate::asked() {
            KEPT.with(|kept| kept.set(true));
            break;
        }
        #[cfg(not(target_arch = "wasm32"))]
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    now() - start
}

/// Seconds on a clock that only goes forward. A browser's worker has
/// no `Instant`, and asks the page's clock instead.
fn now() -> f64 {
    #[cfg(target_arch = "wasm32")]
    return js_sys::Date::now() / 1000.0;
    #[cfg(not(target_arch = "wasm32"))]
    {
        static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
        START
            .get_or_init(std::time::Instant::now)
            .elapsed()
            .as_secs_f64()
    }
}
