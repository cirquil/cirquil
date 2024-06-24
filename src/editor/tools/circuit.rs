use eframe::emath::Rect;
use egui::{Painter, Response};
use crate::core::simulation::component::{Component, ComponentModel};
use crate::core::simulation::components::subcircuit::Subcircuit;
use crate::core::uuid::make_uuid;
use crate::editor::app::State;
use crate::editor::canvas::grid::nearest_grid_anchor;
use crate::editor::project::CircuitId;
use crate::editor::tools::Action;

pub struct Circuit {
    id: CircuitId,
}

impl Circuit {
    pub fn new(name: impl ToString) -> Self {
        Self {
            id: name.to_string(),
        }
    }
}

impl Action for Circuit {
    fn act(&mut self, state: &mut State, response: &Response, _painter: &Painter, _viewport: Rect) {
        if !response.clicked() || state.project.is_picked(&self.id) {
            return;
        }

        if state.project.picked_circuit().is_none() {
            return eprintln!("Failed to retrieve the picked circuit!");
        }

        let mut pointer = response.interact_pointer_pos().unwrap();
        pointer = nearest_grid_anchor(pointer - response.rect.min.to_vec2());

        let component = {
            let Some(subcircuit) = state.project.get_circuit(&self.id) else {
                return eprintln!("Failed to retrieve the picked subcircuit!");
            };

            Component {
                pins: subcircuit.pins.clone(),
                properties: Default::default(),
                model: ComponentModel::Subcircuit(Subcircuit::NotInstantiated(self.id.clone())),
                uuid: make_uuid(),
            }
        };

        state.project.picked_circuit().unwrap().add_component(component, pointer.to_vec2());
    }
}
