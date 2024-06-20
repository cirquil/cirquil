use eframe::emath::Rect;
use eframe::epaint::{Color32, Rounding, Shape, Stroke};
use egui::Context;

use crate::core::simulation::components::subcircuit::output_pin::OutputPin;
use crate::gui::component::AsShapes;

impl AsShapes for OutputPin {
    fn as_shapes(&self, context: &Context) -> Vec<Shape> {
        vec![
            Shape::rect_stroke(
                Rect::from_x_y_ranges(0f32..=20f32, -10f32..=10f32),
                Rounding::same(15f32), Stroke::new(2f32, Color32::GRAY),
            )
        ]
    }
}