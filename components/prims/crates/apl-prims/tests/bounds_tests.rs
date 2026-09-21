//! Roll and deal at the edges: arguments large enough that the
//! arithmetic overflowed, and deals whose memory was proportional to
//! the pool rather than to the hand.
//!
//! Found comparing sw-apl with Roger Hui's "Roll", which confirmed the
//! generator -- Lehmer, 7^5, 2^31-1, first link 7^5 -- and led to
//! these, which it is not about.

use std::time::Instant;

use apl_prims::{Env, deal, roll};
use apl_value::{Array, Data, Number};

const MODULUS: u64 = 2_147_483_647;
const MULTIPLIER: u64 = 16807;

fn nums(a: &Array) -> Vec<i64> {
    let Data::Num(v) = &a.data else {
        panic!("not numbers")
    };
    v.iter()
        .map(|n| match n {
            Number::Int(i) => *i,
            Number::Float(_) => panic!("float"),
        })
        .collect()
}

fn scalar(n: i64) -> Array {
    Array::scalar(Number::Int(n))
}

#[test]
fn a_large_roll_is_computed_exactly_not_wrapped() {
    // After )CLEAR the link is 7^5. These three rolls are the ones that
    // exposed the overflow: the third wrapped in u64 and came back as
    // 7,753,573,564. The values here are the exact floors, computed
    // with integers of unbounded size.
    let mut env = Env::default();
    assert_eq!(
        nums(&roll(&scalar(1_000_000_000), &mut env).unwrap()),
        [131_537_788]
    );
    assert_eq!(
        nums(&roll(&scalar(10_000_000_000), &mut env).unwrap()),
        [7_556_053_221]
    );
    assert_eq!(
        nums(&roll(&scalar(1_000_000_000_000_000), &mut env).unwrap()),
        [458_650_131_671_364]
    );
}

#[test]
fn a_large_roll_stays_in_range() {
    let mut env = Env::default();
    let n = 9_000_000_000_000_000_i64;
    for _ in 0..200 {
        let got = nums(&roll(&scalar(n), &mut env).unwrap())[0];
        assert!((1..=n).contains(&got), "{got} is not in 1..{n}");
    }
}

/// The deal sw-apl had before, kept as the oracle: a dense partial
/// Fisher-Yates over every index in the pool. A deal that is not
/// identical to this would change every transcript that deals.
fn dense(count: usize, total: usize, link: u64) -> (Vec<i64>, u64) {
    let mut link = link;
    let mut pool: Vec<usize> = (0..total).collect();
    for i in 0..count {
        link = link * MULTIPLIER % MODULUS;
        let span = u128::try_from(total - i).expect("fits");
        let offset = span * u128::from(link - 1) / u128::from(MODULUS - 1);
        pool.swap(i, i + usize::try_from(offset).expect("fits"));
    }
    let hand = pool[..count]
        .iter()
        .map(|&i| 1 + i64::try_from(i).expect("fits"))
        .collect();
    (hand, link)
}

#[test]
fn deal_gives_exactly_what_the_dense_deal_gave() {
    for &(count, total) in &[
        (0, 0),
        (0, 5),
        (1, 1),
        (5, 52),
        (52, 52),
        (13, 100),
        (99, 100),
        (7, 1000),
    ] {
        for start in [MULTIPLIER, 1, 12_345, MODULUS - 1, 987_654_321] {
            let mut env = Env {
                link: start,
                ..Env::default()
            };
            let got = nums(&deal(&scalar(count), &scalar(total), &mut env).unwrap());
            let count = usize::try_from(count).expect("fits");
            let (want, link) = dense(count, usize::try_from(total).expect("fits"), start);
            assert_eq!(got, want, "{count}?{total} from link {start}");
            assert_eq!(
                env.link, link,
                "{count}?{total} left the link somewhere else"
            );
        }
    }
}

#[test]
fn dealing_one_from_a_billion_does_not_build_the_billion() {
    // The dense deal allocated every index in the pool to pick one:
    // 1?1E9 wanted about 8 GB, past a browser's 4 GB ceiling. It must
    // now take the time and memory of the hand, not of the pool.
    let started = Instant::now();
    let hand = nums(&deal(&scalar(1), &scalar(1_000_000_000), &mut Env::default()).unwrap());
    assert_eq!(hand.len(), 1);
    assert!((1..=1_000_000_000).contains(&hand[0]));
    assert!(
        started.elapsed().as_millis() < 500,
        "took {:?}",
        started.elapsed()
    );
}
