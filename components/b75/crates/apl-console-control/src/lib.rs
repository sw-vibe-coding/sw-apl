//! Console control, `⎕CC`: the IBM 5110's own system function, for
//! its national character sets, screen, audible alarm, keyboard case,
//! scrolling and printer tab (IBM 5110 APL Reference Manual, Chapter
//! 5). (B) '75 has it and (A) '70 does not. sw-apl has none of those
//! devices, so `⎕CC` checks its arguments and answers as the 5110
//! would, and does nothing else. See docs/mode-b.md.

mod control;

pub use control::{control, national};
