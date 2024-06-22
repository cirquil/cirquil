use eframe::emath::{Pos2, Rect};
use eframe::epaint::{Color32, Shape, Stroke};
use egui::Context;

use crate::core::simulation::components::logic::not_gate::NotGate;
use crate::gui::component::{AsShapes, Bounds};

impl AsShapes for NotGate {
    fn as_shapes(&self, _context: &Context) -> Vec<Shape> {
        vec![
            Shape::convex_polygon(vec![
                Pos2::new(-6.0, 0.0),
                Pos2::new(-20.0, -8.0),
                Pos2::new(-20.0, 8.0),
            ], Color32::WHITE, Stroke::new(2.0, Color32::BLACK)),
            Shape::circle_stroke(Pos2::new(-3.0, 0.0), 3.0, Stroke::new(2.0, Color32::BLACK)),
            // Shape::rect_filled(RECT, Rounding::ZERO, Color32::RED),
            // Shape::rect_stroke(RECT, Rounding::ZERO, Stroke::new(1.0, Color32::BLACK)),
        ]
    }
}

impl Bounds for NotGate {
    fn get_bounds(&self) -> Rect {
        Rect::NOTHING
    }
}
