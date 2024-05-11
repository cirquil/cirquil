use std::default::Default;

use egui::{Color32, Pos2, Rect, Rounding, Shape, Stroke};

use crate::core::simulation::components::tunnel::Tunnel;
use crate::gui::component::{AsShapes, Bounds, Poke};

const RECT: Rect = Rect::from_min_max(Pos2::new(-20.0, -10.0), Pos2::new(0.0, 10.0));

impl Poke for Tunnel {
    fn mouse_clicked(&self, _: Pos2) {}
}

impl AsShapes for Tunnel {
    fn as_shapes(&self) -> Vec<Shape> {
        vec![
            Shape::rect_filled(RECT, Rounding::ZERO, Color32::YELLOW),
            Shape::rect_stroke(RECT, Rounding::ZERO, Stroke::new(1.0, Color32::BLACK)),
        ]
    }
}

impl Bounds for Tunnel {
    fn get_bounds(&self) -> Rect {
        RECT
    }
}
