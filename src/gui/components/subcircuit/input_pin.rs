use eframe::epaint::Shape;
use egui::{Color32, Context, Rect, Rounding, Stroke};

use crate::core::simulation::components::subcircuit::input_pin::InputPin;
use crate::gui::component::AsShapes;

impl AsShapes for InputPin {
    fn as_shapes(&self, _context: &Context) -> Vec<Shape> {
        vec![
            Shape::rect_stroke(
                Rect::from_x_y_ranges(-20.0..=0.0, -10.0..=10.0),
                Rounding::ZERO, Stroke::new(2.0, Color32::GRAY),
            )
        ]
    }
}