use egui::{Color32, Pos2, Rect, Rounding, Shape, Stroke, Vec2};
use crate::core::simulation::components::clock_generator::ClockGenerator;
use crate::gui::component::{AsShapes, Poke};

const RECT: Rect = Rect::from_min_max(Pos2::new(-20.0, -10.0), Pos2::new(0.0, 10.0));

impl Poke for ClockGenerator {}

impl AsShapes for ClockGenerator {
    fn as_shapes(&self) -> Vec<Shape> {
        let color = match self.value.get() {
            0 => Color32::DARK_GREEN,
            1 => Color32::LIGHT_GREEN,
            _ => Color32::BLACK
        };
        vec![
            Shape::rect_filled(RECT, Rounding::ZERO, color),
            Shape::rect_stroke(RECT, Rounding::ZERO, Stroke::new(1.0, Color32::BLACK)),
        ]
    }
}
