use egui::{Color32, Pos2, Rect, Rounding, Shape, Stroke, Vec2};
use crate::core::simulation::components::clock_generator::ClockGenerator;
use crate::gui::component::{AsShapes, Poke};

const RECT: Rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(20.0, 20.0));

impl Poke for ClockGenerator {}

impl AsShapes for ClockGenerator {
    fn as_shapes(&self) -> Vec<Shape> {
        vec![
            Shape::rect_filled(RECT, Rounding::ZERO, Color32::LIGHT_GRAY),
            Shape::rect_stroke(RECT, Rounding::ZERO, Stroke::new(1.0, Color32::BLACK)),
        ]
    }
}
