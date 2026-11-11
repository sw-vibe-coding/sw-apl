//! The APL timeline: its events, the scale that puts a year at a
//! place across the strip, and where each callout goes. Nothing here
//! needs a browser, so all of it is tested natively; the Yew crate
//! beside this one only draws what it is given.

mod event;
mod layout;
mod scale;

pub use event::{Event, Kind, events};
pub use layout::{Placed, Side, place};
pub use scale::{NOW, Scale};
