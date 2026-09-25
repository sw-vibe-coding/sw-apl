//! Library 1, baked into the bundle.

use apl_session::Shelf;

/// Library 1: the workspaces sw-apl ships, read out of the
/// repository when the bundle is built. Each is kept under its file's
/// name, which says the modes it runs in, as a library directory
/// holds it: a workspace that differs between the modes is two.
const SHIPPED: [(&str, &str); 7] = [
    (
        "BIRDS.a-70",
        include_str!("../../../../../ws/lib1/BIRDS.a-70.apl.ws"),
    ),
    (
        "BIRDS.b-75",
        include_str!("../../../../../ws/lib1/BIRDS.b-75.apl.ws"),
    ),
    ("EDIT", include_str!("../../../../../ws/lib1/EDIT.apl.ws")),
    ("LEARN", include_str!("../../../../../ws/lib1/LEARN.apl.ws")),
    ("LIFE", include_str!("../../../../../ws/lib1/LIFE.apl.ws")),
    ("RACE", include_str!("../../../../../ws/lib1/RACE.apl.ws")),
    (
        "TTTML.b-75",
        include_str!("../../../../../ws/lib1/TTTML.b-75.apl.ws"),
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
