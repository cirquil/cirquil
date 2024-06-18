use eframe::emath::{Pos2, Rect};
use eframe::epaint::{Color32, Rounding, Shape, Stroke};
use egui::Context;

use crate::core::simulation::components::input::button::InputButton;
use crate::gui::component::{AsShapes, Bounds, Poke};

const RECT: Rect = Rect::from_min_max(Pos2::new(-20.0, -10.0), Pos2::new(0.0, 10.0));

impl Poke for InputButton {
    fn mouse_pressed(&self, _: Pos2) {
        self.state.set(1);
    }

    fn mouse_released(&self, _: Pos2) {
        self.state.set(0);
    }
}

impl AsShapes for InputButton {
    fn as_shapes(&self, context: &Context) -> Vec<Shape> {
        match self.state.get() {
            1 => {
                vec![
                    Shape::rect_filled(RECT, Rounding::ZERO, Color32::WHITE),
                    Shape::rect_stroke(RECT, Rounding::ZERO, Stroke::new(2.0, Color32::BLACK)),
                ]
            }
            _ => {
                vec![
                    Shape::convex_polygon(
                        vec![
                            Pos2::new(0f32, -10f32),
                            Pos2::new(0f32, 10f32),
                            Pos2::new(-20f32, 10f32),
                            Pos2::new(-25f32, 5f32),
                            Pos2::new(-5f32, 5f32),
                            Pos2::new(-5f32, -15f32),
                        ],
                        Color32::WHITE, Stroke::new(2.0, Color32::BLACK),
                    ),
                    Shape::line(
                        vec![
                            Pos2::new(-5f32, -15f32),
                            Pos2::new(-25f32, -15f32),
                            Pos2::new(-25f32, 5f32),
                        ],
                        Stroke::new(2.0, Color32::BLACK),
                    ),
                    Shape::line_segment(
                        [Pos2::new(-5f32, 5f32), Pos2::new(0f32, 10f32)],
                        Stroke::new(2.0, Color32::BLACK),
                    ),
                ]
            }
        }
    }
}

impl Bounds for InputButton {
    fn get_bounds(&self) -> Rect {
        Rect::from_min_max(Pos2::new(-25.0, -15.0), Pos2::new(0.0, 10.0))
    }
}
