use std::cell::Cell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::core::simulation::circuit::Circuit;
use crate::core::simulation::component::{Behaviour, Component, ComponentModel, ComponentPins, ComponentProperties};
use crate::core::simulation::components::subcircuit::Subcircuit::NotInstantiated;
use crate::core::simulation::pin::Pin;
use crate::core::uuid::make_uuid;
use crate::serde::project::SavedCircuit;

pub mod input_pin;
pub mod output_pin;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Subcircuit {
    Instantiated(Rc<Circuit>),
    NotInstantiated(String),
}

impl Behaviour for Subcircuit {
    fn propagate(&self, pins: &ComponentPins, _properties: &ComponentProperties) {
        debug_assert!(matches!(self, Subcircuit::Instantiated(_)));

        if let Subcircuit::Instantiated(circuit) = self {
            let mut initial_components = vec![];

            for (component_pin, circuit_pin) in circuit.input_pins.iter() {
                let pin_comp = circuit.components.get(*circuit_pin).unwrap();
                if let ComponentModel::InputPin(p) = &pin_comp.model {
                    initial_components.push(pin_comp);

                    p.value.set(
                        pins.get_value(*component_pin)
                    );
                }
            }

            circuit.propagate(initial_components);

            for (component_pin, circuit_pin) in circuit.output_pins.iter() {
                let pin_comp = circuit.components.get(*circuit_pin).unwrap();
                if let ComponentModel::OutputPin(p) = &pin_comp.model {
                    pins.set_value(*component_pin, p.value.get());
                }
            }
        }
    }
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
            model: ComponentModel::Subcircuit(NotInstantiated(subcircuit_name.to_string())),
            uuid: make_uuid(),
        }
    }
}
