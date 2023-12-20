use egui::{Shape, Vec2};

pub trait Poke {
    fn mouse_pressed(&self) {}
    fn mouse_released(&self) {}
    fn mouse_dragged(&self) {}

    fn key_typed(&self) {}
}

pub trait AsShapes {
    fn as_shapes(&self, coords: Vec2) -> Vec<Shape>;
}
