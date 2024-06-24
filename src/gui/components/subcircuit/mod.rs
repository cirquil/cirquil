use eframe::emath::Rect;
use eframe::epaint::{Color32, Rounding, Shape, Stroke};
use egui::Context;

use crate::core::simulation::components::subcircuit::Subcircuit;
use crate::gui::component::AsShapes;

pub mod input_pin;
pub mod output_pin;

impl AsShapes for Subcircuit {
    fn as_shapes(&self, _context: &Context) -> Vec<Shape> {
        vec![
            Shape::rect_stroke(
                Rect::from_x_y_ranges(-30.0..=0.0, -20.0..=40.0),
                Rounding::ZERO, Stroke::new(2.0, Color32::BLACK),
            )
        ]
    }
}