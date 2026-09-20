mod compose;
mod display;
mod edit;

pub use display::display;
pub use edit::Move;
use std::collections::BTreeMap;

pub struct Keyboard {
    cells: Vec<String>,
    pub cursor: usize,
    pub pending: bool,
    pub literal: bool,
    pub map: BTreeMap<char, String>,
}

impl Default for Keyboard {
    fn default() -> Self {
        Self {
            cells: Vec::new(),
            cursor: 0,
            pending: false,
            literal: false,
            map: serde_json::from_str(include_str!("../../../keymap.json")).unwrap(),
        }
    }
}

impl Keyboard {
    pub fn text(&self) -> String {
        self.cells.concat()
    }
    pub fn before_cursor(&self) -> String {
        self.cells[..self.cursor].concat()
    }
    pub fn clear(&mut self) {
        self.cells.clear();
        self.cursor = 0;
        self.pending = false;
    }
}
