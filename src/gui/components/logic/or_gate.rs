use eframe::emath::{Pos2, Rect, Vec2};
use eframe::epaint::{Color32, QuadraticBezierShape, Rounding, Shape, Stroke};
use crate::core::simulation::components::logic::or_gate::OrGate;
use crate::gui::component::{AsShapes, Poke};

impl Poke for OrGate {}

impl AsShapes for OrGate {
    fn as_shapes(&self) -> Vec<Shape> {
        vec![
            Shape::QuadraticBezier(QuadraticBezierShape::from_points_stroke(
                [
                    Pos2::new(0f32, 0f32),
                    Pos2::new(-9f32, 16f32),
                    Pos2::new(-30f32, 15f32)
                ],
                false,
                Color32::WHITE, Stroke::new(2f32, Color32::BLACK)
            )),
            Shape::QuadraticBezier(QuadraticBezierShape::from_points_stroke(
                [
                    Pos2::new(0f32, 0f32),
                    Pos2::new(-9f32, -16f32),
                    Pos2::new(-30f32, -15f32)
                ],
                false,
                Color32::WHITE, Stroke::new(2f32, Color32::BLACK)
            )),
            Shape::QuadraticBezier(QuadraticBezierShape::from_points_stroke(
                [
                    Pos2::new(-30f32, 15f32),
                    Pos2::new(-20f32, -0f32),
                    Pos2::new(-30f32, -15f32)
                ],
                false,
                Color32::WHITE, Stroke::new(2f32, Color32::BLACK)
            )),
        ]
    }
}
