use eframe::emath::{Pos2, Rect, Vec2};
use eframe::epaint::{Color32, Rounding, Shape, Stroke};
use crate::core::simulation::components::logic::or_gate::OrGate;
use crate::gui::component::{AsShapes, Poke};

const RECT: Rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(20.0, 20.0));

impl Poke for OrGate {}

impl AsShapes for OrGate {
    fn as_shapes(&self, coords: Vec2) -> Vec<Shape> {
        let mut normalized_rect = RECT;
        normalized_rect.min += coords;
        normalized_rect.max += coords;

        vec![
            Shape::rect_filled(normalized_rect, Rounding::ZERO, Color32::BLUE),
            Shape::rect_stroke(normalized_rect, Rounding::ZERO, Stroke::new(1.0, Color32::BLACK)),
        ]
    }
}
