use eframe::emath::{Pos2, Rect};
use eframe::epaint::{Color32, QuadraticBezierShape, Shape, Stroke};
use egui::Context;

use crate::core::simulation::components::logic::and_gate::AndGate;
use crate::gui::component::{AsShapes, Bounds};

impl AsShapes for AndGate {
    fn as_shapes(&self, _context: &Context) -> Vec<Shape> {
        vec![
            Shape::QuadraticBezier(QuadraticBezierShape::from_points_stroke(
                [
                    Pos2::new(0.0, 0.0),
                    Pos2::new(-1.0, 14.0),
                    Pos2::new(-13.0, 15.0)
                ],
                false,
                Color32::WHITE, Stroke::new(2.0, Color32::BLACK),
            )),
            Shape::QuadraticBezier(QuadraticBezierShape::from_points_stroke(
                [
                    Pos2::new(0.0, 0.0),
                    Pos2::new(-1.0, -14.0),
                    Pos2::new(-13.0, -15.0)
                ],
                false,
                Color32::WHITE, Stroke::new(2.0, Color32::BLACK),
            )),
            Shape::line(vec![
                Pos2::new(-13.0, -15.0),
                Pos2::new(-30.0, -15.0),
                Pos2::new(-30.0, 15.0),
                Pos2::new(-13.0, 15.0),
            ], Stroke::new(2.0, Color32::BLACK)),
            // Shape::rect_filled(RECT, Rounding::ZERO, Color32::GREEN),
            // Shape::rect_stroke(RECT, Rounding::ZERO, Stroke::new(1.0, Color32::BLACK)),
        ]
    }
}

impl Bounds for AndGate {
    fn get_bounds(&self) -> Rect {
        Rect::NOTHING
    }
}
