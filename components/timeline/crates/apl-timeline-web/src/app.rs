//! The strip: as wide as its years, scrolled at first so that 1970 --
//! or the first live demo's callout, A'70, if it reaches further left --
//! is at the left edge of the view; the earlier years are off to the
//! left and the later ones to the right.
//!
//! The axis runs across the middle: the strip is as tall above it as
//! below, whichever side reaches further, and is drawn smaller, to no
//! less than 90 per cent, when that is taller than the view: the text
//! matters more than how much of the strip shows at once.
//!
//! It is drawn twice. The first time each box is as tall as `Event::size`
//! guesses, or taller if its words need it; then every box is measured,
//! and the second drawing lays the strip out with the heights the
//! browser's own fonts actually take, so no words are ever cut off.

use apl_timeline::{Event, Placed, Scale, Side, events, place};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, HtmlElement};
use yew::prelude::*;

use crate::axis::{Axis, sideways, use_start_at};
use crate::callout::Callout;

/// The events, built in, so the page fetches nothing for them.
const DATA: &str = include_str!("../../../../../data/timeline.json");

/// The strip's years and margin; its pixels to a year fit the view. The
/// margin is wide, so callouts that fan out left of 1954 stay on the page.
const STRIP: Scale = Scale {
    first: 1954,
    last: 2035,
    per_year: 80.0,
    margin: 480.0,
};

/// Callouts start this far from the axis line, on either side.
const BAND: f64 = 52.0;

/// The timeline.
#[function_component]
pub fn App() -> Html {
    let list = use_memo((), |()| events(DATA).unwrap_or_default());
    let (scale, room) = strip();
    let (view, track) = (use_node_ref(), use_node_ref());
    let placed = place(&use_sizes(&list, &track), &scale);
    let axis = reach(&placed, Side::Above).max(reach(&placed, Side::Below)) + BAND;
    let height = 2.0 * axis + 16.0;
    let zoom = (room / height).clamp(0.9, 1.0);
    let first = list.iter().position(|e| e.demo.is_some());
    use_start_at(
        &view,
        scale.opening(1970.0, first.map(|i| placed[i].left)) * zoom,
    );
    let callouts = list.iter().zip(&placed).enumerate().map(|(tone, (event, placed))| {
        html! { <Callout event={event.clone()} placed={*placed} {axis} band={BAND} x={scale.x(event.year)} {tone} /> }
    });
    html! {
        <div class="strip" ref={view.clone()} tabindex="0" onwheel={sideways(view)}>
            <div class="track" ref={track} style={format!("width:{}px;height:{height}px;zoom:{zoom}", scale.width())}>
                <Axis {scale} y={axis} />
                { for callouts }
            </div>
        </div>
    }
}

/// The strip, 1954 to well past 2026 with eleven years to a view, from 1970 as it opens, and
/// the height left for the strip below the header
/// and above the footer: a laptop's when they cannot be read. A strip
/// taller than that is drawn smaller to fit it.
fn strip() -> (Scale, f64) {
    let window = web_sys::window();
    let read = |side: fn(&web_sys::Window) -> Result<JsValue, JsValue>, or: f64| {
        let value = window.as_ref().and_then(|w| side(w).ok());
        value.and_then(|v| v.as_f64()).unwrap_or(or)
    };
    let document = window.as_ref().and_then(web_sys::Window::document);
    let mount = document.and_then(|d| d.get_element_by_id("timeline"));
    let mount = mount.and_then(|e| e.dyn_into::<HtmlElement>().ok());
    let top = mount.map_or(96.0, |e| f64::from(e.offset_top()));
    let wide = read(web_sys::Window::inner_width, 1280.0);
    let room = read(web_sys::Window::inner_height, 800.0) - top - 72.0;
    let per_year = Scale::fitting(wide, 11.0);
    (Scale { per_year, ..STRIP }, room)
}

/// Each event's year and box size: estimated, until the first drawing
/// has been measured in `track`, and then as measured.
#[hook]
fn use_sizes(list: &[Event], track: &NodeRef) -> Vec<(f64, (f64, f64), Option<Side>)> {
    let measured = use_state(|| None::<Vec<f64>>);
    let (track, set) = (track.clone(), measured.clone());
    use_effect_with((), move |()| {
        let found = track
            .cast::<Element>()
            .map(|t| t.query_selector_all(".callout"));
        if let Some(Ok(boxes)) = found {
            let each =
                (0..boxes.length()).filter_map(|i| boxes.item(i)?.dyn_into::<HtmlElement>().ok());
            set.set(Some(each.map(|e| f64::from(e.offset_height())).collect()));
        }
    });
    let known = measured.as_ref();
    let height = |i: usize, guess: f64| known.and_then(|m| m.get(i).copied()).unwrap_or(guess);
    let size = |(i, e): (usize, &Event)| (e.year, (e.size().0, height(i, e.size().1)), e.side);
    list.iter().enumerate().map(size).collect()
}

/// How far the callouts on `side` reach from the axis, which is where
/// the axis goes: below everything above it.
fn reach(placed: &[Placed], side: Side) -> f64 {
    let on = placed.iter().filter(|p| p.side == side);
    on.map(|p| p.offset + p.height).fold(0.0, f64::max)
}
