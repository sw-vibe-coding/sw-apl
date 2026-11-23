//! `⎕FC`, the format control picture format reads (APL Reference,
//! May 1983, pages 8-18 and 8-19): the system variable only (D) has.

mod control;

pub use control::{assign, control, read};
