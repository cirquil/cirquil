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
                Rect::from_x_y_ranges(-30f32..=0f32, -10f32..=20f32),
                Rounding::ZERO, Stroke::new(2f32, Color32::BLACK),
            )
        ]
    }
}