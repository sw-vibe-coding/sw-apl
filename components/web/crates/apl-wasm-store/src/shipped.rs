//! Library 1, baked into the bundle.

use apl_session::Shelf;

/// Library 1: the workspaces sw-apl ships, read out of the
/// repository when the bundle is built. Each is kept under its file's
/// name, which says the modes it runs in, as a library directory
/// holds it: a workspace that differs between the modes is two.
const SHIPPED: [(&str, &str); 17] = [
    (
        "BIRDS.a",
        include_str!("../../../../../ws/lib1/BIRDS.a.apl.ws"),
    ),
    (
        "BIRDS.b",
        include_str!("../../../../../ws/lib1/BIRDS.b.apl.ws"),
    ),
    (
        "BIRDS.c",
        include_str!("../../../../../ws/lib1/BIRDS.c.apl.ws"),
    ),
    (
        "BIRDS.d",
        include_str!("../../../../../ws/lib1/BIRDS.d.apl.ws"),
    ),
    (
        "EDIT.ab",
        include_str!("../../../../../ws/lib1/EDIT.ab.apl.ws"),
    ),
    (
        "EDIT.d",
        include_str!("../../../../../ws/lib1/EDIT.d.apl.ws"),
    ),
    (
        "GRAPHICS.d",
        include_str!("../../../../../ws/lib1/GRAPHICS.d.apl.ws"),
    ),
    (
        "LEARN.ab",
        include_str!("../../../../../ws/lib1/LEARN.ab.apl.ws"),
    ),
    (
        "LEARN.d",
        include_str!("../../../../../ws/lib1/LEARN.d.apl.ws"),
    ),
    (
        "LIFE.ab",
        include_str!("../../../../../ws/lib1/LIFE.ab.apl.ws"),
    ),
    (
        "LIFE.c",
        include_str!("../../../../../ws/lib1/LIFE.c.apl.ws"),
    ),
    (
        "LIFE.d",
        include_str!("../../../../../ws/lib1/LIFE.d.apl.ws"),
    ),
    (
        "RACE.ab",
        include_str!("../../../../../ws/lib1/RACE.ab.apl.ws"),
    ),
    (
        "RACE.c",
        include_str!("../../../../../ws/lib1/RACE.c.apl.ws"),
    ),
    (
        "RACE.d",
        include_str!("../../../../../ws/lib1/RACE.d.apl.ws"),
    ),
    (
        "TTTML.b",
        include_str!("../../../../../ws/lib1/TTTML.b.apl.ws"),
    ),
    (
        "TTTML.d",
        include_str!("../../../../../ws/lib1/TTTML.d.apl.ws"),
    ),
];

/// Library 1, by key.
#[must_use]
pub fn lib1() -> Shelf {
    SHIPPED
        .iter()
        .map(|(k, t)| ((*k).to_string(), (*t).to_string()))
        .collect()
}
