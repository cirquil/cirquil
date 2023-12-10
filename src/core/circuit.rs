use crate::core::component::{Component, ComponentIdx};
use crate::core::pin::Direction;
use crate::core::wire::{Wire, WireIdx};

pub struct Circuit {
    pub components: Vec<Box<dyn Component>>,
    pub wires: Vec<Wire>,
    pub clock_generators: Vec<ComponentIdx>
}

impl Circuit {
    pub fn get_component(&self, idx: ComponentIdx) -> &dyn Component {
        self.components.get(idx).unwrap().as_ref()
    }

    pub fn get_wire(&self, idx: WireIdx) -> &Wire {
        self.wires.get(idx).unwrap()
    }
    pub fn propagate(&self) {
        for clock_idx in self.clock_generators.iter() {
            let clock = self.get_component(*clock_idx);
            clock.on_tick_start();
        }

        let mut first: Vec<&dyn Component> = vec![0, 1, 2].into_iter()
            .map(|idx| self.get_component(idx))
            .collect();
        // let mut first = vec![ self.get_component(2) ];
        let mut second: Vec<&dyn Component> = Vec::new();

        while !first.is_empty() {
            for component in first.iter() {
                for pin in component.get_pins() {
                    if pin.direction == Direction::INPUT {
                        let wire_idx = match pin.wire.get() {
                            None => { continue }
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

            for component in first.iter() {
                for pin in component.get_pins() {
                    if pin.direction == Direction::OUTPUT {
                        let wire_idx = match pin.wire.get() {
                            Some(v) => { v }
                            None => { continue }
                        };
                        let wire = self.get_wire(wire_idx);

                        wire.value.set(pin.value.get());

                        for (component_idx, pin_idx) in &wire.connected_components {
                            let component = self.get_component(*component_idx);
                            if component.get_pins().get(*pin_idx).unwrap().direction == Direction::INPUT {
                                second.push(component);
                            }
                        }
                    }
                }
            }

            first.clear();
            first.append(&mut second);
        }
    }
}