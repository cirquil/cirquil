mod tree;
mod cursors;
mod tool;
mod component;
mod circuit;

use eframe::emath::Rect;
use egui::{Painter, Response};

pub use tree::Tree;
use crate::editor::app::State;

pub trait Action {
    fn act(&mut self, state: &mut State, response: &Response, painter: &Painter, viewport: Rect);
}
