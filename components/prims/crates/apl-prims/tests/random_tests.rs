//! Roll with the workspace random link.

use apl_prims::{Env, apply_monadic, roll};
use apl_value::{Array, Data, ErrorKind, Number};

fn nums(a: &Array) -> Vec<i64> {
    let Data::Num(v) = &a.data else { panic!() };
    v.iter()
        .map(|n| match n {
            Number::Int(i) => *i,
            Number::Float(_) => panic!("float"),
        })
        .collect()
}

#[test]
fn the_link_starts_at_16807_and_roll_is_reproducible() {
    let mut env = Env::default();
    assert_eq!((env.io, env.link), (1, 16807));
    let first = nums(&roll(&Array::vector(vec![Number::Int(6); 10]), &mut env).unwrap());
    assert!(first.iter().all(|&x| (1..=6).contains(&x)), "{first:?}");
    let again = nums(
        &roll(
            &Array::vector(vec![Number::Int(6); 10]),
            &mut Env::default(),
        )
        .unwrap(),
    );
    assert_eq!(first, again);
    assert_ne!(env.link, 16807);
}

#[test]
fn roll_honours_the_origin_and_rejects_bad_arguments() {
    let mut env = Env { io: 0, link: 16807 };
    let r = nums(&roll(&Array::vector(vec![Number::Int(1); 5]), &mut env).unwrap());
    assert_eq!(r, [0; 5]);
    let mut env = Env::default();
    assert_eq!(
        nums(&roll(&Array::scalar(Number::Int(1)), &mut env).unwrap()),
        [1]
    );
    assert_eq!(
        roll(&Array::scalar(Number::Int(0)), &mut env)
            .unwrap_err()
            .kind,
        ErrorKind::Domain
    );
    assert_eq!(
        roll(&Array::scalar(Number::Float(2.5)), &mut env)
            .unwrap_err()
            .kind,
        ErrorKind::Domain
    );
    let shaped = Array::new(vec![2, 2], Data::Num(vec![Number::Int(3); 4])).unwrap();
    assert_eq!(roll(&shaped, &mut env).unwrap().shape, vec![2, 2]);
}

#[test]
fn query_dispatches_to_roll_through_the_env() {
    let mut env = Env::default();
    let r = apply_monadic('?', &Array::scalar(Number::Int(100)), None, &mut env).unwrap();
    assert!(matches!(nums(&r)[0], 1..=100));
}

#[test]
fn deal_picks_distinct_indexes_reproducibly() {
    use apl_prims::deal;
    let mut env = Env::default();
    let hand = nums(
        &deal(
            &Array::scalar(Number::Int(5)),
            &Array::scalar(Number::Int(52)),
            &mut env,
        )
        .unwrap(),
    );
    assert_eq!(hand.len(), 5);
    let mut sorted = hand.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), 5, "distinct");
    assert!(hand.iter().all(|&x| (1..=52).contains(&x)));
    let again = nums(
        &deal(
            &Array::scalar(Number::Int(5)),
            &Array::scalar(Number::Int(52)),
            &mut Env::default(),
        )
        .unwrap(),
    );
    assert_eq!(hand, again);
    let mut all = nums(
        &deal(
            &Array::scalar(Number::Int(5)),
            &Array::scalar(Number::Int(5)),
            &mut env,
        )
        .unwrap(),
    );
    all.sort_unstable();
    assert_eq!(all, [1, 2, 3, 4, 5]);
    let mut env = Env { io: 0, link: 16807 };
    let mut all = nums(
        &deal(
            &Array::scalar(Number::Int(3)),
            &Array::scalar(Number::Int(3)),
            &mut env,
        )
        .unwrap(),
    );
    all.sort_unstable();
    assert_eq!(all, [0, 1, 2]);
    assert_eq!(
        nums(
            &deal(
                &Array::scalar(Number::Int(0)),
                &Array::scalar(Number::Int(3)),
                &mut env
            )
            .unwrap()
        ),
        []
    );
    assert_eq!(
        deal(
            &Array::scalar(Number::Int(4)),
            &Array::scalar(Number::Int(3)),
            &mut env
        )
        .unwrap_err()
        .kind,
        ErrorKind::Domain
    );
    // A left argument of more than one element is RANK ERROR. This once
    // asserted it of a one-element vector, which APL\360 takes as its
    // scalar -- the shuffle, A[(⍴A)?⍴A], depends on that.
    assert_eq!(
        deal(
            &Array::vector(vec![Number::Int(1), Number::Int(2)]),
            &Array::scalar(Number::Int(3)),
            &mut env
        )
        .unwrap_err()
        .kind,
        ErrorKind::Rank
    );
}
