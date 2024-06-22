use eframe::emath::{Pos2, Rect};
use eframe::epaint::{Color32, QuadraticBezierShape, Shape, Stroke};
use egui::Context;

use crate::core::simulation::components::logic::or_gate::OrGate;
use crate::gui::component::{AsShapes, Bounds};

impl AsShapes for OrGate {
    fn as_shapes(&self, _context: &Context) -> Vec<Shape> {
        vec![
            Shape::QuadraticBezier(QuadraticBezierShape::from_points_stroke(
                [
                    Pos2::new(0.0, 0.0),
                    Pos2::new(-9.0, 16.0),
                    Pos2::new(-30.0, 15.0)
                ],
                false,
                Color32::TRANSPARENT, Stroke::new(2.0, Color32::BLACK),
            )),
            Shape::QuadraticBezier(QuadraticBezierShape::from_points_stroke(
                [
                    Pos2::new(0.0, 0.0),
                    Pos2::new(-9.0, -16.0),
                    Pos2::new(-30.0, -15.0)
                ],
                false,
                Color32::TRANSPARENT, Stroke::new(2.0, Color32::BLACK),
            )),
            Shape::QuadraticBezier(QuadraticBezierShape::from_points_stroke(
                [
                    Pos2::new(-30.0, 15.0),
                    Pos2::new(-20.0, -0.0),
                    Pos2::new(-30.0, -15.0)
                ],
                false,
                Color32::TRANSPARENT, Stroke::new(2.0, Color32::BLACK),
            )),
        ]
    }
}

impl Bounds for OrGate {
    fn get_bounds(&self) -> Rect {
        Rect::NOTHING
    }
}
