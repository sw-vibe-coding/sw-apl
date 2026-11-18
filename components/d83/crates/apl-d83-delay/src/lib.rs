//! (D) '83's `⎕DL`, the delay APLSV had and IBM PC APL kept: wait a
//! number of seconds, and give back how long it was. ATTN ends the
//! wait early, and the run then stops at the end of its line.

mod delay;

pub use delay::delay;
