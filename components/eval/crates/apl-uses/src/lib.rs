//! Which modes a workspace runs in, from what its code uses. The
//! writer puts the answer on the workspace's `⍝!MODES` line.

mod uses;

pub use uses::runs_in;
