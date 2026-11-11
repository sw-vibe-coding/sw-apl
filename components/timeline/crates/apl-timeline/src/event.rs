//! One event on the timeline, as `data/timeline.json` writes it.

use serde::Deserialize;

use crate::layout::Side;

/// What an event is: one of sw-apl's machines or a picture to come
/// (a key event), a line of history with no picture (a note), or one
/// of the owner's other APL projects, at the end.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Key,
    Note,
    Mine,
}

/// An event: when it was, as a year with a fraction for the month,
/// and as words; what it was; and, for a key event, its picture or
/// the tile where one will go, and where it leads.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Event {
    pub year: f64,
    pub when: String,
    pub title: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub tbd: Option<String>,
    #[serde(default)]
    pub href: Option<String>,
    /// For one of the owner's live demos: its name, as A'70, and what it
    /// is, shown under the picture with the letter underscored.
    #[serde(default)]
    pub demo: Option<String>,
    /// What the picture says when the pointer is over it.
    #[serde(default)]
    pub tip: Option<String>,
    /// The side of the axis the callout must take, when the layout's own
    /// choice is the wrong one.
    #[serde(default)]
    pub side: Option<Side>,
    pub kind: Kind,
}

impl Event {
    /// Whether its callout shows a picture, or a tile where one goes.
    #[must_use]
    pub fn pictured(&self) -> bool {
        self.image.is_some() || self.tbd.is_some()
    }

    /// Its callout's width and height in pixels: room for the picture,
    /// if it has one, then a line of the hand-written date and title for
    /// every 16 characters of them and a line of the words for every 24.
    #[must_use]
    pub fn size(&self) -> (f64, f64) {
        let lines = |text: &str, per: usize| text.chars().count().div_ceil(per);
        let title = lines(&format!("{}: {}", self.when, self.title), 16);
        let said = u32::try_from(title + lines(&self.text, 24) + usize::from(self.demo.is_some()))
            .unwrap_or(u32::MAX);
        let (width, picture) = if self.pictured() {
            (310.0, 175.0)
        } else {
            (230.0, 0.0)
        };
        (width, picture + 38.0 + 31.0 * f64::from(said))
    }
}

/// The events in `json`, in the order written.
///
/// # Errors
/// When `json` is not a list of events.
pub fn events(json: &str) -> Result<Vec<Event>, serde_json::Error> {
    serde_json::from_str(json)
}
