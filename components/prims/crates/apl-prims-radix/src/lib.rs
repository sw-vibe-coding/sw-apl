//! Encode (`l⊤r`) and decode (`l⊥r`) in mixed radix.

mod place;
mod radix;

pub use radix::{decode, encode};
