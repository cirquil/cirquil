use eframe::emath::{Align2, Pos2, Rect};
use eframe::epaint::{Color32, FontId, Shape, Stroke};
use egui::Context;

use crate::core::simulation::components::tunnel::Tunnel;
use crate::gui::component::{AsShapes, Bounds};

const RECT: Rect = Rect::from_min_max(Pos2::new(-20.0, -10.0), Pos2::new(0.0, 10.0));
const LABEL_PAD: Pos2 = Pos2::new(10.0, 0.0);

impl AsShapes for Tunnel {
    fn as_shapes(&self, context: &Context) -> Vec<Shape> {
        let label = context.fonts(|fonts|
            Shape::text(fonts, LABEL_PAD, Align2::LEFT_CENTER, &self.name, FontId::monospace(11.0), Color32::BLACK)
        );
        let rect = label.visual_bounding_rect();

        let polygon = Shape::convex_polygon(
            vec![
                Pos2::ZERO,
                Pos2::new(7.5, -10.0),
                Pos2::new(rect.right() + 7.5, -10.0),
                Pos2::new(rect.right() + 7.5, 10.0),
                Pos2::new(7.5, 10.0),
            ],
            Color32::WHITE,
            Stroke::new(1.5, Color32::BLACK),
        );

        vec![polygon, label]
    }
}

impl Bounds for Tunnel {
    fn get_bounds(&self) -> Rect {
        RECT
    }
}
