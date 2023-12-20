use egui::{Color32, Pos2, Rect, Rounding, Shape, Stroke, Vec2};
use egui::text::Fonts;
use crate::core::components::clock_generator::ClockGenerator;
use crate::gui::as_shapes::AsShapes;

const RECT: Rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(20.0, 20.0));

impl AsShapes for ClockGenerator {
    fn as_shapes(&self, coords: Vec2) -> Vec<Shape> {
        let mut normalized_rect = RECT;
        normalized_rect.min += coords;
        normalized_rect.max += coords;

        vec![
            Shape::rect_filled(normalized_rect, Rounding::ZERO, Color32::LIGHT_GRAY),
            Shape::rect_stroke(normalized_rect, Rounding::ZERO, Stroke::new(1.0, Color32::BLACK)),
        ]
    }
}
