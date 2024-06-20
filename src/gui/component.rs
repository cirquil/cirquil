use egui::{Context, Pos2, Rect, Shape, Vec2};

use crate::core::simulation::component::{Component, ComponentModel};

pub trait Poke {
    fn mouse_pressed(&self, _position: Pos2) {}
    fn mouse_released(&self, _position: Pos2) {}
    fn mouse_clicked(&self, _position: Pos2) {}
    fn mouse_dragged(&self, _delta: Vec2) {}

    fn key_typed(&self) {}
}

pub trait AsShapes {
    fn as_shapes(&self, context: &Context) -> Vec<Shape>;
}

pub trait Bounds {
    fn get_bounds(&self) -> Rect;
}

impl Component {
    pub fn mouse_pressed(&self, position: Pos2) {
        match &self.model {
            ComponentModel::ClockGenerator(c) => { c.mouse_pressed(position) }
            ComponentModel::InputButton(c) => { c.mouse_pressed(position) }
            _ => {}
        }
    }
    pub fn mouse_released(&self, position: Pos2) {
        match &self.model {
            ComponentModel::ClockGenerator(c) => { c.mouse_released(position) }
            ComponentModel::InputButton(c) => { c.mouse_released(position) }
            _ => {}
        }
    }
    pub fn mouse_clicked(&self, position: Pos2) {
        match &self.model {
            ComponentModel::ClockGenerator(c) => { c.mouse_clicked(position) }
            ComponentModel::InputButton(c) => { c.mouse_clicked(position) }
            _ => {}
        }
    }
    pub fn mouse_dragged(&self, delta: Vec2) {
        match &self.model {
            ComponentModel::ClockGenerator(c) => { c.mouse_dragged(delta) }
            ComponentModel::InputButton(c) => { c.mouse_dragged(delta) }
            _ => {}
        }
    }

    pub fn key_typed(&self) {
        match &self.model {
            ComponentModel::ClockGenerator(c) => { c.key_typed() }
            ComponentModel::InputButton(c) => { c.key_typed() }
            _ => {}
        }
    }
    pub fn as_shapes(&self, context: &Context) -> Vec<Shape> {
        match &self.model {
            ComponentModel::ClockGenerator(c) => { c.as_shapes(context) }
            ComponentModel::AndGate(c) => { c.as_shapes(context) }
            ComponentModel::OrGate(c) => { c.as_shapes(context) }
            ComponentModel::NotGate(c) => { c.as_shapes(context) }
            ComponentModel::InputButton(c) => { c.as_shapes(context) }
            ComponentModel::Tunnel(c) => { c.as_shapes(context) }

            ComponentModel::InputPin(c) => { c.as_shapes(context) }
            ComponentModel::OutputPin(c) => { c.as_shapes(context) }
            ComponentModel::Subcircuit(c) => { c.as_shapes(context) }
        }
    }
    pub fn get_bounds(&self) -> Rect {
        match &self.model {
            ComponentModel::ClockGenerator(c) => { c.get_bounds() }
            ComponentModel::AndGate(c) => { c.get_bounds() }
            ComponentModel::OrGate(c) => { c.get_bounds() }
            ComponentModel::NotGate(c) => { c.get_bounds() }
            ComponentModel::InputButton(c) => { c.get_bounds() }
            ComponentModel::Tunnel(c) => { c.get_bounds() }

            ComponentModel::InputPin(_) => { Rect::ZERO }
            ComponentModel::OutputPin(_) => { Rect::ZERO }
            ComponentModel::Subcircuit(_) => { Rect::ZERO }
        }
    }
}