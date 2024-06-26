use egui::{Color32, Context, Pos2, Rect, Rounding, Shape, Stroke};

use crate::core::simulation::component::Tick;
use crate::core::simulation::components::clock_generator::ClockGenerator;
use crate::gui::component::{AsShapes, Bounds, Poke};

const RECT: Rect = Rect::from_min_max(Pos2::new(-20.0, -10.0), Pos2::new(0.0, 10.0));

impl Poke for ClockGenerator {
    fn mouse_clicked(&self, _: Pos2) {
        self.tick();
    }
}

impl AsShapes for ClockGenerator {
    fn as_shapes(&self, _context: &Context) -> Vec<Shape> {
        let color = match self.value.get() {
            0 => Color32::DARK_GREEN,
            1 => Color32::LIGHT_GREEN,
            _ => Color32::BLACK
        };
        vec![
            Shape::rect_filled(RECT, Rounding::ZERO, color),
            Shape::rect_stroke(RECT, Rounding::ZERO, Stroke::new(1.0, Color32::BLACK)),
        ]
    }
}

impl Bounds for ClockGenerator {
    fn get_bounds(&self) -> Rect {
        RECT
    }
}
