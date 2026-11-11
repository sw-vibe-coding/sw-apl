//! The axis -- a line with an arrow, a tick for every year, and the
//! year written under every fifth -- and how the strip it runs along
//! scrolls: to its middle years at first, and sideways to the wheel.

use apl_timeline::{NOW, Scale};
use web_sys::{Element, WheelEvent};
use yew::prelude::*;

/// The axis's scale and how far down the strip it runs.
#[derive(Properties, PartialEq)]
pub struct Props {
    pub scale: Scale,
    pub y: f64,
}

/// The axis.
#[function_component]
pub fn Axis(props: &Props) -> Html {
    let Props { scale, y } = props;
    let (left, right) = (scale.margin / 2.0, scale.x(f64::from(NOW) + 0.8));
    let ticks = (scale.first..=NOW).map(|year| tick(scale, year, *y));
    let past = right + 24.0;
    html! {
        <>
            <div class="line" style={format!("left:{left}px;top:{y}px;width:{}px", right - left)} />
            { for ticks }
            <div class="beyond" style={format!("left:{past}px;top:{}px", y - 13.0)}>{ "And now, mine" }</div>
        </>
    }
}

/// One year's tick, and its label if it is a fifth year.
fn tick(scale: &Scale, year: i32, y: f64) -> Html {
    let x = scale.x(f64::from(year));
    let fifth = year % 5 == 0;
    let class = if fifth { "tick fifth" } else { "tick" };
    html! {
        <>
            <div {class} style={format!("left:{x}px;top:{y}px")} />
            if fifth {
                <div class="year" style={format!("left:{x}px;top:{}px", y + 12.0)}>{ year }</div>
            }
        </>
    }
}

/// Scroll the strip to `left` when it is first drawn, and again if the
/// measured drawing moves `left`, so the view opens where it should.
#[hook]
pub fn use_start_at(view: &NodeRef, left: f64) {
    let view = view.clone();
    use_effect_with(left, move |&left| {
        if let Some(strip) = view.cast::<Element>() {
            strip.scroll_to_with_x_and_y(left, 0.0);
        }
    });
}

/// A wheel turned up or down moves the strip left or right, since it
/// only scrolls one way.
pub fn sideways(view: NodeRef) -> Callback<WheelEvent> {
    Callback::from(move |turn: WheelEvent| {
        let (down, across) = (turn.delta_y(), turn.delta_x());
        if let (true, Some(strip)) = (down.abs() > across.abs(), view.cast::<Element>()) {
            strip.scroll_by_with_x_and_y(down, 0.0);
        }
    })
}
