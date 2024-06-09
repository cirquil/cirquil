use egui::{Pos2, Rect, Shape, Vec2};

use crate::core::simulation::component::{Component, ComponentModel};

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

impl Component {
    pub fn mouse_pressed(&self, position: Pos2) {}
    pub fn mouse_released(&self, position: Pos2) {}
    pub fn mouse_clicked(&self, position: Pos2) {}
    pub fn mouse_dragged(&self, delta: Vec2) {}

    pub fn key_typed(&self) {}
    pub fn as_shapes(&self) -> Vec<Shape> {
        match &self.component {
            ComponentModel::ClockGenerator(c) => { c.as_shapes() }
            ComponentModel::AndGate(c) => { c.as_shapes() }
            ComponentModel::OrGate(c) => { c.as_shapes() }
            ComponentModel::NotGate(c) => { c.as_shapes() }
            ComponentModel::InputButton(c) => { c.as_shapes() }
            ComponentModel::Tunnel(c) => { c.as_shapes() }
        }
    }
    pub fn get_bounds(&self) -> Rect {
        match &self.component {
            ComponentModel::ClockGenerator(c) => { c.get_bounds() }
            ComponentModel::AndGate(c) => { c.get_bounds() }
            ComponentModel::OrGate(c) => { c.get_bounds() }
            ComponentModel::NotGate(c) => { c.get_bounds() }
            ComponentModel::InputButton(c) => { c.get_bounds() }
            ComponentModel::Tunnel(c) => { c.get_bounds() }
        }
    }
}