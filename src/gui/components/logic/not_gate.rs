use eframe::emath::{Pos2, Rect};
use eframe::epaint::{Color32, Shape, Stroke};
use egui::Context;

use crate::core::simulation::components::logic::not_gate::NotGate;
use crate::gui::component::{AsShapes, Bounds};

impl AsShapes for NotGate {
    fn as_shapes(&self, context: &Context) -> Vec<Shape> {
        vec![
            Shape::convex_polygon(vec![
                Pos2::new(-6f32, 0f32),
                Pos2::new(-20f32, -8f32),
                Pos2::new(-20f32, 8f32),
            ], Color32::WHITE, Stroke::new(2f32, Color32::BLACK)),
            Shape::circle_stroke(Pos2::new(-3f32, 0f32), 3f32, Stroke::new(2.0, Color32::BLACK)),
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
