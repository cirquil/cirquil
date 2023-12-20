use eframe::emath::{Pos2, Rect, Vec2};
use eframe::epaint::{Color32, Rounding, Shape, Stroke};
use crate::core::components::logic::not_gate::NotGate;
use crate::gui::AsShapes;

const RECT: Rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(20.0, 20.0));

impl AsShapes for NotGate {
    fn as_shapes(&self, coords: Vec2) -> Vec<Shape> {
        let mut normalized_rect = RECT;
        normalized_rect.min += coords;
        normalized_rect.max += coords;

        vec![
            Shape::rect_filled(normalized_rect, Rounding::ZERO, Color32::RED),
            Shape::rect_stroke(normalized_rect, Rounding::ZERO, Stroke::new(1.0, Color32::BLACK)),
        ]
    }
}
