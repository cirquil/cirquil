use std::cell::Cell;

use serde::{Deserialize, Serialize};

use crate::core::simulation::circuit::CircuitIdx;
use crate::core::simulation::component::{Component, ComponentModel, ComponentPins, ComponentProperties};
use crate::core::simulation::components::subcircuit::Subcircuit::NotInstantiated;
use crate::core::simulation::pin::Pin;
use crate::serde::project::SavedCircuit;

pub mod input_pin;
pub mod output_pin;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Subcircuit {
    Instantiated(CircuitIdx),
    NotInstantiated(String),
}

impl Subcircuit {
    pub fn from_saved_circuit(saved_circuit: &SavedCircuit, subcircuit_name: &str) -> Component {
        Component {
            pins: {
                let subcircuit_pins = saved_circuit.pins.iter()
                    .map(|pin| Pin {
                        value: Cell::new(Default::default()),
                        bit_width: 1,
                        direction: pin.direction,
                        wire: Cell::new(None),
                        location: pin.location,
                    })
                    .collect();

                ComponentPins::new(subcircuit_pins)
            },
            properties: ComponentProperties::new(vec![]),
            component: ComponentModel::Subcircuit(NotInstantiated(subcircuit_name.to_string())),
        }
    }
}
