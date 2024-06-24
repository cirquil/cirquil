use eframe::emath::Rect;
use egui::{Painter, Response};
use crate::core::simulation::component::Component;
use crate::editor::app::State;
use crate::editor::canvas::grid::nearest_grid_anchor;
use crate::editor::tools::Action;

pub struct ComponentFactory {
    factory: Box<dyn FnMut() -> Component>,
}

impl ComponentFactory {
    pub fn new(factory: Box<dyn FnMut() -> Component>) -> Self {
        Self {
            factory
        }
    }
}

impl Action for ComponentFactory {
    fn act(&mut self, state: &mut State, response: &Response, _painter: &Painter, _viewport: Rect) {
        if !response.clicked() {
            return;
        }
        
        let Some(circuit) = state.project.picked_circuit() else {
            return eprintln!("Failed to retrieve the picked circuit!");
        };
        
        let mut pointer = response.interact_pointer_pos().unwrap();
        pointer = nearest_grid_anchor(pointer - response.rect.min.to_vec2());
        
        circuit.add_component((self.factory)(), pointer.to_vec2())
    }
}
