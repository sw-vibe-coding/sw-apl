//! A callout: its box, the leader from the box to its year on the
//! axis -- straight down, or bent when the box has moved sideways --
//! and the dot where the leader meets the axis.

use apl_timeline::{Event, Placed, Side};
use yew::prelude::*;

/// The event, where its box goes, the axis's height down the strip,
/// the band either side of it, and the event's year across.
#[derive(Properties, PartialEq)]
pub struct Props {
    pub event: Event,
    pub placed: Placed,
    pub axis: f64,
    pub band: f64,
    pub x: f64,
    /// Which of the four leader colors, so neighboring leaders differ.
    pub tone: usize,
}

/// The callout.
#[function_component]
pub fn Callout(props: &Props) -> Html {
    let (axis, x, p) = (props.axis, props.x, props.placed);
    let (top, edge) = top_and_edge(&p, axis, props.band);
    let class = format!("callout {:?}", props.event.kind).to_lowercase();
    html! {
        <>
            { leader((p.left + p.width / 2.0, edge), (x, axis), (props.band, props.tone)) }
            <div class="dot" style={format!("left:{x}px;top:{axis}px")} />
            <div {class} style={format!("left:{}px;top:{top}px;width:{}px", p.left, p.width)}>
                { body(&props.event) }
            </div>
        </>
    }
}

/// Where the box's top is, and its edge nearest the axis.
fn top_and_edge(placed: &Placed, axis: f64, band: f64) -> (f64, f64) {
    let near = band + placed.offset;
    match placed.side {
        Side::Above => (axis - near - placed.height, axis - near),
        Side::Below => (axis + near, axis + near),
    }
}

/// The leader from `from`, the middle of the box's edge nearest the
/// axis, to `to`, the year on the axis: straight down from the box to
/// half the band short of the axis, then on the slant to the year. Its
/// color is one of four, by `tone`.
fn leader(from: (f64, f64), to: (f64, f64), (band, tone): (f64, usize)) -> Html {
    let ((bx, by), (yx, ay)) = (from, to);
    let elbow = if by < ay {
        ay - band / 2.0
    } else {
        ay + band / 2.0
    };
    let (x0, y0) = (bx.min(yx) - 2.0, by.min(ay));
    let (w, h) = ((bx - yx).abs() + 4.0, (by - ay).abs());
    let points = format!(
        "{},{} {},{} {},{}",
        bx - x0,
        by - y0,
        bx - x0,
        elbow - y0,
        yx - x0,
        ay - y0
    );
    html! {
        <svg class={format!("leader tone-{}", tone % 4)} style={format!("left:{x0}px;top:{y0}px")} width={w.to_string()} height={h.to_string()}>
            <polyline {points} />
        </svg>
    }
}

/// What the box holds: the picture or its tile, with the tip it shows
/// when the pointer is over it; for a live demo, its name with the
/// letter underscored; then when and what, then the words. All of it is
/// a link when the event leads somewhere.
fn body(event: &Event) -> Html {
    let picture = match (&event.image, &event.tbd) {
        (Some(src), _) => html! { <img src={src.clone()} alt={event.title.clone()} /> },
        (None, Some(what)) => html! { <div class="tbd">{ format!("Image TBD: {what}") }</div> },
        (None, None) => html! {},
    };
    let demo = event.demo.as_deref().map(|name| name.split_at(1));
    let inner = html! {
        <>
            <div class="shot" data-tip={event.tip.clone()}>{ picture }</div>
            if let Some((letter, rest)) = demo { <p class="demo"><u>{ letter }</u>{ rest }</p> }
            <p class="when">{ format!("{}: {}", event.when, event.title) }</p>
            if !event.text.is_empty() { <p class="text">{ &event.text }</p> }
        </>
    };
    match &event.href {
        Some(href) => html! { <a href={href.clone()}>{ inner }</a> },
        None => inner,
    }
}
