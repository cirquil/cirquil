use egui::{Pos2, Rect, Shape, Vec2};

pub trait Poke {
    fn mouse_pressed(&self, position: Pos2) {}
    fn mouse_released(&self, position: Pos2) {}
    fn mouse_clicked(&self, position: Pos2) {}
    fn mouse_dragged(&self, delta: Vec2) {}

    fn key_typed(&self) {}
}

pub trait AsShapes {
    fn as_shapes(&self) -> Vec<Shape>;
}

pub trait Bounds {
    fn get_bounds(&self) -> Rect;
}