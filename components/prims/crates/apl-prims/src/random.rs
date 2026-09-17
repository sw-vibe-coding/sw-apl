//! The evaluation environment and the APL\360 random link.

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

/// Advance the link and return it (in `1..MODULUS`).
fn next(env: &mut Env) -> u64 {
    env.link = env.link * MULTIPLIER % MODULUS;
    env.link
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
            let pick = n * (next(env) - 1) / (MODULUS - 1);
            Ok(Number::Int(env.io + i64::try_from(pick).unwrap_or(0)))
        })
        .collect::<AplResult<Vec<_>>>()?;
    Array::new(r.shape.clone(), Data::Num(out))
}
