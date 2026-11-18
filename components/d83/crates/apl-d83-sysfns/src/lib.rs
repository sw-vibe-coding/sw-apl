//! (D) '83's shared-variable system functions, `⎕SVO`, `⎕SVR`,
//! `⎕SVC` and `⎕SVQ`, as IBM PC APL's manual (May 1983, chapter 9)
//! gives them: what each takes and gives back. The sharing itself is
//! `apl-d83-shares`', kept on the workspace. See
//! docs/ibm-pc-apl-plan.md.

mod call;
mod control;
mod offer;

pub use call::{Class, dyadic, is_function, monadic};
