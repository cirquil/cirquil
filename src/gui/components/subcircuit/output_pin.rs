use eframe::emath::Rect;
use eframe::epaint::{Color32, Rounding, Shape, Stroke};
use egui::Context;

use crate::core::simulation::components::subcircuit::output_pin::OutputPin;
use crate::gui::component::AsShapes;

impl AsShapes for OutputPin {
    fn as_shapes(&self, _context: &Context) -> Vec<Shape> {
        vec![
            Shape::rect_stroke(
                Rect::from_x_y_ranges(0.0..=20.0, -10.0..=10.0),
                Rounding::same(15.0), Stroke::new(2.0, Color32::GRAY),
            )
        ]
    }
}