//! The timeline's data, its scale and its layout.

use apl_timeline::{Event, Kind, Placed, Scale, Side, events, place};

/// Each event's year and estimated size, as `place` takes them.
fn sized(list: &[Event]) -> Vec<(f64, (f64, f64), Option<Side>)> {
    list.iter().map(|e| (e.year, e.size(), e.side)).collect()
}

/// How far the callouts on `side` reach from the axis.
fn reach(placed: &[Placed], side: Side) -> f64 {
    let on = placed.iter().filter(|p| p.side == side);
    on.map(|p| p.offset + p.height).fold(0.0, f64::max)
}

const DATA: &str = include_str!("../../../../../data/timeline.json");

fn note(year: f64) -> Event {
    Event {
        year,
        when: "1970".into(),
        title: "A note".into(),
        text: String::new(),
        image: None,
        tbd: None,
        href: None,
        demo: None,
        tip: None,
        side: None,
        kind: Kind::Note,
    }
}

const SCALE: Scale = Scale {
    first: 1964,
    last: 2032,
    per_year: 80.0,
    margin: 60.0,
};

/// The page's own strip, for laying out the real data.
const PAGE: Scale = Scale {
    first: 1954,
    last: 2035,
    per_year: 160.0,
    margin: 480.0,
};

#[test]
fn a_year_has_a_place_and_the_view_opens_at_1970() {
    assert!((SCALE.x(1964.0) - 60.0).abs() < 1e-9);
    assert!((SCALE.x(1970.0) - 540.0).abs() < 1e-9);
    assert!((SCALE.width() - (60.0 + 68.0 * 80.0 + 60.0)).abs() < 1e-9);
    assert!((SCALE.opening(1970.0, None) - (SCALE.x(1970.0) - 16.0)).abs() < 1e-9);
    assert!(
        (SCALE.opening(1970.0, Some(500.0)) - 484.0).abs() < 1e-9,
        "a box further left shows whole"
    );
    assert!(
        (SCALE.opening(1970.0, Some(600.0)) - 524.0).abs() < 1e-9,
        "one to the right does not move it"
    );
    assert!(
        SCALE.opening(1964.0, Some(-50.0)).abs() < 1e-9,
        "never before the strip"
    );
    assert!((Scale::fitting(2400.0, 16.0) - 150.0).abs() < 1e-9);
    assert!(
        (Scale::fitting(320.0, 16.0) - 140.0).abs() < 1e-9,
        "a phone is not crowded"
    );
}

#[test]
fn neighbors_alternate_and_a_crowd_moves_sideways() {
    let crowd = [note(1971.0), note(1971.2), note(1971.4)];
    let placed = place(&sized(&crowd), &SCALE);
    let aside = |p: &Placed| (p.left + p.width / 2.0 - p.anchor).abs() > 1.0;
    assert!(
        placed.iter().all(|p| p.offset.abs() < 1e-9),
        "none stacks: {placed:?}"
    );
    assert!(placed.iter().any(aside), "a crowd moves aside: {placed:?}");
    let sides: Vec<Side> = placed.iter().map(|p| p.side).collect();
    assert_eq!(
        sides,
        [Side::Above, Side::Below, Side::Above],
        "neighbors alternate"
    );
    let apart = place(&sized(&[note(1970.0), note(1980.0), note(1990.0)]), &SCALE);
    assert!(
        apart.iter().all(|p| p.offset.abs() < 1e-9 && !aside(p)),
        "far apart, each stays at its year, against the axis"
    );
}

#[test]
fn no_two_callouts_on_a_side_overlap() {
    let list = events(DATA).expect("data/timeline.json is a list of events");
    let placed = place(&sized(&list), &SCALE);
    for (i, a) in placed.iter().enumerate() {
        for b in &placed[i + 1..] {
            let across = a.left < b.left + b.width && b.left < a.left + a.width;
            let along = a.offset < b.offset + b.height && b.offset < a.offset + a.height;
            assert!(
                a.side != b.side || !(across && along),
                "{a:?} overlaps {b:?}"
            );
        }
    }
}

#[test]
fn the_data_is_in_order_and_every_key_event_is_pictured() {
    let list = events(DATA).expect("data/timeline.json is a list of events");
    assert!(
        list.windows(2).all(|w| w[0].year <= w[1].year),
        "in order of year"
    );
    for event in &list {
        assert_eq!(
            event.kind == Kind::Note,
            !event.pictured(),
            "{}",
            event.title
        );
        let href = event.href.as_deref().unwrap_or("x");
        assert!(
            !href.starts_with("http:"),
            "{href}: https or the site's own page"
        );
    }
}

fn pictured(year: f64) -> Event {
    Event {
        tbd: Some("a picture".into()),
        kind: Kind::Key,
        ..note(year)
    }
}

#[test]
fn a_crowd_of_pictures_moves_sideways_rather_than_stacking() {
    let crowd = [
        pictured(1984.0),
        pictured(1984.2),
        pictured(1984.5),
        pictured(1985.9),
    ];
    let placed = place(&sized(&crowd), &SCALE);
    assert!(
        placed.iter().all(|p| p.offset < 141.0),
        "none stands far from the axis: {placed:?}"
    );
    let center = |i: usize| placed[i].left + placed[i].width / 2.0;
    assert!(
        (center(2) - SCALE.x(1984.5)).abs() > 1.0,
        "the third moved sideways"
    );
    let below = reach(&placed, Side::Below).max(reach(&placed, Side::Above));
    assert!(
        below < 2.0 * placed[0].height,
        "so the strip is no taller than one picture a side"
    );
}

/// The middle of a callout's edge, where its leader leaves it.
fn stem(p: &Placed) -> f64 {
    p.left + p.width / 2.0
}

#[test]
fn no_leader_crosses_a_box_or_another_leader() {
    let list = events(DATA).expect("data/timeline.json is a list of events");
    for per_year in [140.0, 160.0, 220.0] {
        let scale = Scale { per_year, ..PAGE };
        let placed = place(&sized(&list), &scale);
        for (i, a) in placed.iter().enumerate() {
            for b in placed.iter().filter(|b| b.side == a.side) {
                if std::ptr::eq(a, b) {
                    continue;
                }
                let through = b.offset < a.offset && b.left < stem(a) && stem(a) < b.left + b.width;
                assert!(
                    !through,
                    "at {per_year}: {i}'s leader runs through a box: {a:?} {b:?}"
                );
                let crossed = (stem(a) - stem(b)) * (a.anchor - b.anchor) < 0.0;
                assert!(!crossed, "at {per_year}: two leaders cross: {a:?} {b:?}");
            }
        }
    }
}

#[test]
fn the_strip_is_no_taller_than_its_tallest_box_a_side() {
    let list = events(DATA).expect("data/timeline.json is a list of events");
    for (per_year, grow) in [(140.0, 1.0), (160.0, 1.0), (160.0, 1.4), (220.0, 1.0)] {
        let scale = Scale { per_year, ..PAGE };
        let mut items = sized(&list);
        for (_, (_, h), _) in &mut items {
            *h *= grow;
        }
        let tallest = items.iter().map(|(_, (_, h), _)| *h).fold(0.0, f64::max);
        let placed = place(&items, &scale);
        for side in [Side::Above, Side::Below] {
            assert!(
                reach(&placed, side) <= 1.5 * tallest + 1.0,
                "at {per_year}, {grow}: {side:?} grew past one box"
            );
        }
    }
}

#[test]
fn the_four_demos_are_named_and_say_so() {
    let list = events(DATA).expect("data/timeline.json is a list of events");
    let demos: Vec<&Event> = list.iter().filter(|e| e.demo.is_some()).collect();
    let names: Vec<&str> = demos
        .iter()
        .map(|e| &e.demo.as_deref().unwrap_or("")[..4])
        .collect();
    assert_eq!(names, ["A'70", "B'75", "C'80", "D'83"]);
    for demo in demos {
        let tip = demo.tip.as_deref().unwrap_or("");
        assert!(tip.starts_with("My live demo of "), "{tip}");
        assert!(
            demo.href.is_some() && demo.image.is_some(),
            "{}",
            demo.title
        );
    }
}

/// A crowd spreads about its years: the earliest box moves left, its
/// leader slanting right to its year, and the latest moves right.
#[test]
fn a_crowd_spreads_both_ways() {
    let shift = |p: &Placed| p.left + p.width / 2.0 - p.anchor;
    let crowd = place(
        &sized(&[note(1975.0), note(1975.2), note(1975.4), note(1975.6)]),
        &SCALE,
    );
    assert!(
        shift(&crowd[0]) < -1.0,
        "the earliest moves left: {crowd:?}"
    );
    assert!(shift(&crowd[3]) > 1.0, "the latest moves right: {crowd:?}");
}

/// Left to right, the boxes come in the order of their years, above the
/// axis and below it alike: a 1966 box never stands right of a 1970 one.
#[test]
fn the_boxes_run_in_year_order() {
    let list = events(DATA).expect("data/timeline.json is a list of events");
    for per_year in [140.0, 160.0, 220.0] {
        let placed = place(&sized(&list), &Scale { per_year, ..PAGE });
        let center = |p: &Placed| p.left + p.width / 2.0;
        for (i, a) in placed.iter().enumerate() {
            for b in &placed[i + 1..] {
                let swapped = (center(a) - center(b)) * (a.anchor - b.anchor) < 0.0;
                assert!(!swapped, "at {per_year}: out of year order: {a:?} {b:?}");
            }
        }
    }
}
