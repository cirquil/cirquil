use eframe::emath::{Align2, Pos2, Rect};
use eframe::epaint::{Color32, FontId, Rounding, Shape, Stroke};
use egui::Context;

use crate::core::simulation::components::tunnel::Tunnel;
use crate::gui::component::{AsShapes, Bounds};

const RECT: Rect = Rect::from_min_max(Pos2::new(-20.0, -10.0), Pos2::new(0.0, 10.0));

impl AsShapes for Tunnel {
    fn as_shapes(&self, context: &Context) -> Vec<Shape> {
        vec![
            Shape::rect_filled(RECT, Rounding::ZERO, Color32::YELLOW),
            Shape::rect_stroke(RECT, Rounding::ZERO, Stroke::new(1.0, Color32::BLACK)),
            context.fonts(|fonts| Shape::text(fonts, (RECT.max - RECT.min).to_pos2(), Align2::CENTER_CENTER, &self.name, FontId::monospace(10.0) ,Color32::RED))
        ]
    }
}

impl Bounds for Tunnel {
    fn get_bounds(&self) -> Rect {
        RECT
    }
}
