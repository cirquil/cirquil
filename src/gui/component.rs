use egui::{Rect, Shape};

pub trait Poke {
    fn mouse_pressed(&self) {}
    fn mouse_released(&self) {}
    fn mouse_dragged(&self) {}

    fn key_typed(&self) {}
}

pub trait AsShapes {
    fn as_shapes(&self) -> Vec<Shape>;
}

pub trait Bounds {
    fn get_bounds(&self) -> Rect;
}