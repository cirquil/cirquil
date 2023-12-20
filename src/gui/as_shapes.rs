use egui::{Shape, Vec2};

pub trait AsShapes {
    fn as_shapes(&self, coords: Vec2) -> Vec<Shape>;
}
