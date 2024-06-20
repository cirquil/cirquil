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
                    Pos2::new(0f32, 0f32),
                    Pos2::new(-9f32, 16f32),
                    Pos2::new(-30f32, 15f32)
                ],
                false,
                Color32::TRANSPARENT, Stroke::new(2f32, Color32::BLACK),
            )),
            Shape::QuadraticBezier(QuadraticBezierShape::from_points_stroke(
                [
                    Pos2::new(0f32, 0f32),
                    Pos2::new(-9f32, -16f32),
                    Pos2::new(-30f32, -15f32)
                ],
                false,
                Color32::TRANSPARENT, Stroke::new(2f32, Color32::BLACK),
            )),
            Shape::QuadraticBezier(QuadraticBezierShape::from_points_stroke(
                [
                    Pos2::new(-30f32, 15f32),
                    Pos2::new(-20f32, -0f32),
                    Pos2::new(-30f32, -15f32)
                ],
                false,
                Color32::TRANSPARENT, Stroke::new(2f32, Color32::BLACK),
            )),
        ]
    }
}

impl Bounds for OrGate {
    fn get_bounds(&self) -> Rect {
        Rect::NOTHING
    }
}
