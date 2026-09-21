//! The attention flag: who can set it, who sees it, and how often a
//! run notices.
//!
//! The reason this is not one global: a service holds sixteen
//! sessions on sixteen threads, and an attention meant for one of
//! them must not stop the other fifteen. Each session installs its
//! own flag on its own thread, so "this session's" is what a running
//! primitive reads and there is nothing to look up.

use std::sync::{Arc, Barrier};
use std::thread;

use apl_attn::{Flag, STRIDE, asked, attend, polled};

/// A session: install a flag on this thread and hand it back, as a
/// host does when it starts one.
fn session() -> Flag {
    let flag = Flag::default();
    attend(Box::new(flag.clone()));
    flag
}

#[test]
fn a_thread_with_no_flag_is_never_asked_to_stop() {
    assert!(!asked());
}

#[test]
fn asking_is_seen_by_the_session_asked() {
    let flag = session();
    assert!(!asked());
    flag.ask();
    assert!(asked());
}

#[test]
fn and_is_cleared_by_seeing_it() {
    let flag = session();
    flag.ask();
    assert!(asked());
    assert!(!asked(), "an attention is answered once");
}

#[test]
fn a_flag_may_be_set_from_another_thread() {
    let flag = session();
    let far = flag.clone();
    thread::spawn(move || far.ask())
        .join()
        .expect("the asking thread");
    assert!(asked());
}

#[test]
fn one_session_is_not_stopped_by_another_s_attention() {
    // Two sessions on two threads, as a service holds them. Each
    // installs its flag when it starts -- before anything can ask it,
    // which is why the ask here waits until both have -- and then only
    // the one asked may notice.
    let (one, two) = (Flag::default(), Flag::default());
    let started = Arc::new(Barrier::new(3));
    let checked = Arc::new(Barrier::new(3));
    let run = |flag: Flag| {
        let (started, checked) = (Arc::clone(&started), Arc::clone(&checked));
        thread::spawn(move || {
            attend(Box::new(flag));
            started.wait();
            checked.wait();
            asked()
        })
    };
    let (first, second) = (run(one.clone()), run(two));
    started.wait();
    one.ask();
    checked.wait();
    assert!(
        first.join().expect("the first session"),
        "the one asked did not stop"
    );
    assert!(
        !second.join().expect("the second session"),
        "the one not asked stopped"
    );
}

#[test]
fn polled_notices_within_one_stride() {
    let flag = session();
    flag.ask();
    // A hot loop calls `polled`, which looks only every STRIDE times
    // so the cost is an increment rather than a load. It must still
    // notice, and within one stride.
    let mut noticed = None;
    for i in 0..STRIDE * 2 {
        if polled() {
            noticed = Some(i);
            break;
        }
    }
    let at = noticed.expect("a strided poll never noticed");
    assert!(at < STRIDE, "noticed only after {at} of {STRIDE}");
}

#[test]
fn polled_says_nothing_when_nothing_was_asked() {
    let _flag = session();
    for _ in 0..STRIDE * 4 {
        assert!(!polled());
    }
}

#[test]
fn a_fresh_session_on_a_reused_thread_does_not_inherit_an_old_attention() {
    // A service thread outlives the session on it. The next session
    // installs its own flag, and an attention nobody answered must
    // not stop it before it starts.
    let stale = Flag::default();
    attend(Box::new(stale.clone()));
    stale.ask();
    let _fresh = session();
    assert!(
        !asked(),
        "the new session inherited the old one's attention"
    );
}
