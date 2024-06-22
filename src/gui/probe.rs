use eframe::emath::Align2;
use eframe::epaint::{FontId, Shape};
use egui::{Color32, Context, Pos2, Stroke};

use crate::core::simulation::probe::Probe;
use crate::gui::component::AsShapes;

impl AsShapes for Probe {
    fn as_shapes(&self, context: &Context) -> Vec<Shape> {
        let label = context.fonts(|fonts|
            Shape::text(fonts, Pos2::new(-5.0, -30.0), Align2::LEFT_CENTER, &self.name, FontId::monospace(11.0), Color32::BLACK)
        );

        vec![
            label,
            Shape::convex_polygon(
                vec![
                    Pos2::new(0.0, 0.0),
                    Pos2::new(10.0, -20.0),
                    Pos2::new(20.0, -10.0),
                ],
                Color32::GRAY,
                Stroke::new(1.0, Color32::BLACK),
            ),
        ]
    }
}