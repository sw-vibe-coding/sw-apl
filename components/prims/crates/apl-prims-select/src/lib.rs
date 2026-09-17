//! Selection and rearrangement: take, drop, reverse, rotate,
//! transpose. Each builds its result by mapping every result
//! position to a source position (or a fill element).

mod gather;
mod rotate;
mod take_drop;

pub use gather::axis_index;
pub use rotate::{reverse, rotate, transpose};
pub use take_drop::{drop, take};
