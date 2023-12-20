use eframe::emath::{Pos2, Rect, Vec2};
use eframe::epaint::{Color32, Rounding, Shape, Stroke};
use crate::core::simulation::components::logic::and_gate::AndGate;
use crate::gui::component::{AsShapes, Poke};

const RECT: Rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(20.0, 20.0));

impl Poke for AndGate {}

impl AsShapes for AndGate {
    fn as_shapes(&self) -> Vec<Shape> {
        vec![
            Shape::rect_filled(RECT, Rounding::ZERO, Color32::GREEN),
            Shape::rect_stroke(RECT, Rounding::ZERO, Stroke::new(1.0, Color32::BLACK)),
        ]
    }
}
