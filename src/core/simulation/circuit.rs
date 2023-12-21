use crate::core::simulation::component::{Component, ComponentIdx};
use crate::core::simulation::pin::Direction;
use crate::core::simulation::value::operations::assign;
use crate::core::simulation::wire::{Wire, WireIdx};

pub struct Circuit {
    pub components: Vec<Box<dyn Component>>,
    pub wires: Vec<Wire>,
    pub clock_generators: Vec<ComponentIdx>,
}

impl Circuit {
    pub fn get_component(&self, idx: ComponentIdx) -> &dyn Component {
        self.components.get(idx).unwrap().as_ref()
    }

    pub fn get_wire(&self, idx: WireIdx) -> &Wire {
        self.wires.get(idx).unwrap()
    }

    pub fn tick(&self) {
        for clock_idx in self.clock_generators.iter() {
            let clock = self.get_component(*clock_idx);
            clock.tick();
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
            self.components.iter()
                .map(|e| e.as_ref())
                .collect()
        );
    }

    pub fn propagate(&self, initial_components: Vec<&dyn Component>) {
        let mut first: Vec<&dyn Component> = initial_components.clone();
        let mut second: Vec<&dyn Component> = Vec::new();

        while !first.is_empty() {
            for component in first.iter() {
                for pin in component.get_pins() {
                    if pin.direction == Direction::INPUT {
                        let wire_idx = match pin.wire.get() {
                            None => { continue; }
                            Some(v) => { v }
                        };

                        pin.value.set(self.get_wire(wire_idx).value.get())
                    }
                }
            }

            for component in first.iter() {
                component.propagate();
            }

            // println!("{:?}", first);

            let mut dirty_wires = Vec::new();
            for component in first.iter() {
                for pin in component.get_pins() {
                    if pin.direction == Direction::OUTPUT {
                        let wire_idx = match pin.wire.get() {
                            Some(v) => { v }
                            None => { continue; }
                        };
                        let wire = self.get_wire(wire_idx);

                        dirty_wires.push(wire);
                    }
                }
            }

            for wire in dirty_wires {
                wire.value.set(Default::default());

                for (component_idx, pin_idx) in &wire.connected_components {
                    let component = self.get_component(*component_idx);
                    match component.get_pins().get(*pin_idx).unwrap().direction {
                        Direction::INPUT => {
                            second.push(component);
                        }
                        Direction::OUTPUT => {
                            let value = wire.value.get().apply_binary(self.get_component(*component_idx).get_pin_value(*pin_idx), assign);
                            wire.value.set(value);
                        }
                        Direction::INOUT => {}
                    }
                }
            }

            first.clear();
            first.append(&mut second);
        }
    }
}