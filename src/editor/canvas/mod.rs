pub mod grid;

use egui::{Rect, Vec2};
use crate::editor::project::EditorCircuit;

pub fn canvas_size(viewport: Rect, circuit: Option<&EditorCircuit>) -> Vec2 {
    if let Some(circuit) = circuit {
        (circuit.dimensions() + egui::vec2(100.0, 100.0)).max(viewport.size())
    } else {
        eprintln!("Failed to retrieve the picked circuit!");
        viewport.size()
    }
}
