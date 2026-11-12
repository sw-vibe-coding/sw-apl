//! The TRS-80's screen as the page holds it, in (C) '80.

use apl_c80_screen::Screen;
use wasm_bindgen::prelude::wasm_bindgen;

/// A 64-by-16 screen for one page: what is printed to it obeys the
/// terminal codes (C) sends for the TRS-80's (`apl-c80-screen`).
#[wasm_bindgen]
#[derive(Default)]
pub struct Trs80(Screen);

#[wasm_bindgen]
impl Trs80 {
    /// A blank screen, the cursor at the top left.
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new() -> Trs80 {
        Trs80::default()
    }

    /// Print `text` to it.
    pub fn print(&mut self, text: &str) {
        self.0.print(text);
    }

    /// The screen as the page draws it, as JSON: its 16 lines, and the
    /// line and column of the cursor.
    #[must_use]
    pub fn shown(&self) -> String {
        let (row, col) = self.0.cursor();
        serde_json::json!({ "rows": self.0.rows(), "row": row, "col": col }).to_string()
    }
}
