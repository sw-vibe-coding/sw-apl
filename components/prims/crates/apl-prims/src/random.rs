//! The evaluation environment and the APL\360 random link.
//!
//! The generator is Lehmer's, as APL\360's was and as Roger Hui's
//! "Roll" describes it: the link is multiplied by 7^5 modulo the prime
//! 2^31-1, starting from 7^5. A link is mapped onto `0..N` as
//! `⌊N×(link-1)÷(P-1)`. Hui gives J's as `⌊N×link÷P`; the two agree
//! almost always for small N and differ by one at large N, and nothing
//! shows J's was APL\360's, so this one is kept -- changing it would
//! move every transcript that rolls. `docs/language.md` records it.

use std::collections::HashMap;

use apl_prims_mixed::non_negative_int;
use apl_prims_scalar::numbers;
use apl_value::{AplError, AplResult, Array, Data, ErrorKind, Number};

/// Workspace state the primitives read or advance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Env {
    /// Index origin (`)ORIGIN`), 0 or 1.
    pub io: i64,
    /// The random link: a Lehmer generator state, saved with the
    /// workspace so `?` sequences reproduce.
    pub link: u64,
}

/// The multiplier and the initial link of a clear workspace.
const MULTIPLIER: u64 = 16807;
/// The Mersenne prime 2^31 - 1.
const MODULUS: u64 = 2_147_483_647;

impl Default for Env {
    fn default() -> Self {
        Env {
            io: 1,
            link: MULTIPLIER,
        }
    }
}

/// Advance the link, and map it onto `0..n`.
///
/// In 128 bits: `n × (link-1)` overflows 64 once `n` passes about
/// 8.6×10^9, and a release build wraps rather than failing, so `?1E15`
/// came back as a ten-digit number. The quotient is below `n`, so it
/// always fits where `n` did.
///
/// There are only 2^31-2 distinct links, so for `n` past that not every
/// value in `0..n` can come up, however this is computed. That is not
/// refused: sw-apl does not know what limit APL\360 set, if any, and a
/// DOMAIN ERROR would claim one.
fn next(env: &mut Env, n: u64) -> u64 {
    env.link = env.link * MULTIPLIER % MODULUS;
    let pick = u128::from(n) * u128::from(env.link - 1) / u128::from(MODULUS - 1);
    u64::try_from(pick).unwrap_or(0)
}

/// `?r`: for each positive integer `n`, a random index from the
/// first `n` in the index origin.
///
/// # Errors
/// DOMAIN ERROR unless every element is a positive integer.
pub fn roll(r: &Array, env: &mut Env) -> AplResult<Array> {
    let out = numbers(r)?
        .iter()
        .map(|&n| {
            let n = u64::try_from(non_negative_int(n)?).unwrap_or(0);
            if n == 0 {
                return Err(AplError::new(ErrorKind::Domain));
            }
            Ok(Number::Int(
                env.io + i64::try_from(next(env, n)).unwrap_or(0),
            ))
        })
        .collect::<AplResult<Vec<_>>>()?;
    Array::new(r.shape.clone(), Data::Num(out))
}

/// `l?r`: `l` distinct random indexes from the first `r`, in the
/// index origin, driven by the link: a partial Fisher-Yates shuffle.
///
/// Sparse. The pool is every index in `0..r`, but only the positions a
/// swap has touched are held, and an untouched position holds its own
/// index. So the memory is the hand's, not the pool's: `1?1E9` built a
/// billion indexes to pick one, eight gigabytes and past a browser's
/// ceiling, and escaped the workspace quota doing it. The swaps are the
/// same swaps in the same order, so every deal is the one it was.
///
/// Each argument is a scalar or a one-element vector, as the index
/// generator's is: `A[(⍴A)?⍴A]`, the shuffle, deals from a shape.
///
/// # Errors
/// RANK ERROR unless each is a scalar or a one-element vector; DOMAIN
/// ERROR unless both are non-negative integers with `l` at most `r`.
pub fn deal(l: &Array, r: &Array, env: &mut Env) -> AplResult<Array> {
    let single = |a: &Array| a.shape.len() <= 1 && a.shape.iter().product::<usize>() == 1;
    if !single(l) || !single(r) {
        return Err(AplError::new(ErrorKind::Rank));
    }
    let count = usize::try_from(non_negative_int(numbers(l)?[0])?).unwrap_or(usize::MAX);
    let total = usize::try_from(non_negative_int(numbers(r)?[0])?).unwrap_or(usize::MAX);
    if count > total {
        return Err(AplError::new(ErrorKind::Domain));
    }
    // A swap at step `i` touches positions `i` and `j >= i`, and no
    // later step touches `i` again, so what lands at `i` is dealt card
    // `i` and can be taken there and then.
    let mut moved: HashMap<usize, usize> = HashMap::new();
    let mut picks = Vec::with_capacity(count);
    for i in 0..count {
        let span = u64::try_from(total - i).unwrap_or(1);
        let j = i + usize::try_from(next(env, span)).unwrap_or(0);
        let (at_i, at_j) = (*moved.get(&i).unwrap_or(&i), *moved.get(&j).unwrap_or(&j));
        moved.insert(j, at_i);
        picks.push(Number::Int(env.io + i64::try_from(at_j).unwrap_or(0)));
    }
    Ok(Array::vector(picks))
}
