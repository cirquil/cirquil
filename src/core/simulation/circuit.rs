use serde::{Deserialize, Serialize};

use crate::core::simulation::component::{Component, ComponentIdx, ComponentModel, Tick};
use crate::core::simulation::components::subcircuit::Subcircuit;
use crate::core::simulation::pin::{Direction, PinIdx};
use crate::core::simulation::value::operations::assign;
use crate::core::simulation::wire::{Wire, WireIdx};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circuit {
    pub components: Vec<Component>,
    pub wires: Vec<Wire>,
    pub clock_generators: Vec<ComponentIdx>,
    pub input_pins: Vec<(PinIdx, ComponentIdx)>,
    pub output_pins: Vec<(PinIdx, ComponentIdx)>,
}

pub type CircuitIdx = usize;

impl Circuit {
    const ITERATIONS_TIMEOUT: u16 = 1000;
}

impl Circuit {
    pub fn get_component(&self, idx: ComponentIdx) -> &Component {
        self.components.get(idx).unwrap()
    }

    pub fn get_wire(&self, idx: WireIdx) -> &Wire {
        self.wires.get(idx).unwrap()
    }

    pub fn tick(&self) {
        for clock_idx in self.clock_generators.iter() {
            let clock = self.get_component(*clock_idx);

            if let ComponentModel::ClockGenerator(c) = &clock.component {
                c.tick()
            }
        }
    }

    pub fn propagate_ticked(&self) {
        self.propagate(
            self.clock_generators.iter()
                .map(|idx| self.get_component(*idx))
                .collect()
        )
    }

    pub fn propagate_all(&self) {
        self.propagate(
            self.components.iter().collect()
        );
    }

    pub fn propagate(&self, initial_components: Vec<&Component>) {
        let mut first: Vec<&Component> = initial_components.clone();
        let mut second: Vec<&Component> = Vec::new();

        let mut iterations = 0;
        while !first.is_empty() {
            for component in first.iter() {
                for pin in component.get_pins() {
                    if pin.direction == Direction::Input {
                        let Some(wire_idx) = pin.wire.get() else { continue; };

                        pin.value.set(self.get_wire(wire_idx).value.get())
                    }
                }
            }

            for component in first.iter() {
                if let ComponentModel::Subcircuit(Subcircuit::Instantiated(circuit)) = &component.component {
                    let mut initial_components = vec![];

                    for (component_pin, circuit_pin) in circuit.input_pins.iter() {
                        let pin_comp = circuit.components.get(*circuit_pin).unwrap();
                        if let ComponentModel::InputPin(p) = &pin_comp.component {
                            initial_components.push(pin_comp);

                            p.value.set(
                                component.pins.get_value(*component_pin)
                            );
                        }
                    }

                    circuit.propagate(initial_components);

                    for (component_pin, circuit_pin) in circuit.output_pins.iter() {
                        let pin_comp = circuit.components.get(*circuit_pin).unwrap();
                        if let ComponentModel::OutputPin(p) = &pin_comp.component {
                            component.pins.set_value(*component_pin, p.value.get());
                        }
                    }
                } else {
                    component.propagate();
                }
            }

            // println!("{:?}", first);

            let mut dirty_wires = Vec::new();
            for component in first.iter() {
                for pin in component.get_pins() {
                    if pin.direction == Direction::Output {
                        let Some(wire_idx) = pin.wire.get() else { continue; };
                        let wire = self.get_wire(wire_idx);

                        if pin.value.get() != wire.value.get() {
                            dirty_wires.push(wire);
                        }
                    }
                }
            }

            for wire in dirty_wires {
                wire.value.set(Default::default());

                for (component_idx, pin_idx) in &wire.connected_components {
                    let component = self.get_component(*component_idx);
                    match component.get_pins().get(*pin_idx).unwrap().direction {
                        Direction::Input => {
                            second.push(component);
                        }
                        Direction::Output => {
                            let value = wire.value.get().apply_binary(self.get_component(*component_idx).get_pin_value(*pin_idx), assign);
                            wire.value.set(value);
                        }
                        Direction::Inout => {}
                    }
                }
            }

            first.clear();
            first.append(&mut second);

            if iterations > Self::ITERATIONS_TIMEOUT {
                panic!("Detected oscillation, shutting down");
            }
            iterations += 1;
        }
    }
}