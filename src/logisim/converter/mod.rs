use std::cell::Cell;
use std::collections::HashMap;

use crate::core::canvas::circuit::CanvasCircuit;
use crate::core::canvas::component::CanvasComponent;
use crate::core::canvas::location::Location;
use crate::core::canvas::wire::CanvasWire;
use crate::core::simulation::circuit::Circuit;
use crate::core::simulation::component::{Component, ComponentIdx};
use crate::core::simulation::components::clock_generator::ClockGenerator;
use crate::core::simulation::components::input::button::InputButton;
use crate::core::simulation::components::logic::and_gate::AndGate;
use crate::core::simulation::components::logic::not_gate::NotGate;
use crate::core::simulation::components::logic::or_gate::OrGate;
use crate::core::simulation::components::tunnel::Tunnel;
use crate::core::simulation::pin::PinIdx;
use crate::core::simulation::wire::{Wire, WireIdx};
use crate::logisim::converter::dfs::{dfs_wires, DfsComponents};
use crate::logisim::parser::location::LogisimLocation;
use crate::logisim::parser::project::LogisimProject;
use crate::logisim::parser::wire::LogisimWire;

mod dfs;

pub fn convert_circuit(parsed_project: LogisimProject, circuit_idx: usize) -> (Circuit, CanvasCircuit) {
    let circuit = &parsed_project.circuits[circuit_idx];

    let mut wires_map: HashMap<LogisimLocation, Vec<&LogisimWire>> = HashMap::new();
    for i in circuit.wires.iter() {
        match wires_map.get_mut(&i.from) {
            None => {
                wires_map.insert(i.from, vec![i]);
            }
            Some(v) => {
                v.push(i);
            }
        }
        match wires_map.get_mut(&i.to) {
            None => {
                wires_map.insert(i.to, vec![i]);
            }
            Some(v) => {
                v.push(i);
            }
        }
    }

    let mut dfs_components = DfsComponents::new(&circuit.components);
    let mut canvas_wires: Vec<CanvasWire> = Vec::new();
    let mut wires: Vec<Wire> = Vec::new();
    let mut wire_nodes: HashMap<Location, WireIdx> = HashMap::new();
    let mut wire_index = 0;
    while !wires_map.is_empty() {
        let begin = *wires_map.keys().next().unwrap();
        let (segments, nodes) = dfs_wires(&begin, &mut wires_map, &mut dfs_components);
        for (from, to) in segments.iter() {
            wire_nodes.insert(*from, wire_index);
            wire_nodes.insert(*to, wire_index);
        }
        let canvas_wire = CanvasWire { wire: wire_index, segments, nodes };
        wires.push(Wire {
            value: Cell::new(Default::default()),
            connected_components: Vec::new(),
        });
        canvas_wires.push(canvas_wire);
        wire_index += 1;
    }

    let mut canvas_components: Vec<CanvasComponent> = Vec::new();
    let mut components: Vec<Component> = Vec::new();
    let mut clock_generators: Vec<usize> = Vec::new();
    let mut pins_no_wire: HashMap<Location, (ComponentIdx, PinIdx)> = HashMap::new();
    for (comp_i, parsed_comp) in circuit.components.iter().enumerate() {
        let loc = Location::new(parsed_comp.loc.x, parsed_comp.loc.y);
        canvas_components.push(CanvasComponent { component: comp_i, loc });
        let component: Component = match (parsed_comp.lib.as_str(), parsed_comp.name.as_str()) {
            ("0", "Clock") => {
                clock_generators.push(comp_i);
                ClockGenerator::create()
            }
            ("5", "Button") => InputButton::create(),
            ("1", "OR Gate") => OrGate::from_bit_width(1),
            ("1", "AND Gate") => AndGate::from_bit_width(1),
            ("1", "NOT Gate") => NotGate::from_bit_width(1),
            ("0", "Tunnel") => {
                Tunnel::from_name_width(
                    parsed_comp.get_param("label").unwrap(),
                    1,
                )
            }
            _ => panic!("Unknown component {} from lib {}", parsed_comp.name, parsed_comp.lib),
        };
        for (pin_i, pin) in component.get_pins().iter().enumerate() {
            let location = loc + pin.location;
            match wire_nodes.get(&location) {
                Some(wire_i) => {
                    component.set_pin_wire(pin_i, Some(*wire_i));
                    wires.get_mut(*wire_i).unwrap().connected_components.push((comp_i, pin_i));
                }
                None => {
                    match pins_no_wire.get_mut(&location) {
                        Some((another_comp, another_pin)) => {
                            let wire_i = wires.len();
                            let wire = Wire {
                                value: Cell::new(Default::default()),
                                connected_components: vec![(*another_comp, *another_pin), (comp_i, pin_i)],
                            };
                            wire_nodes.insert(location, wire_i);
                            component.set_pin_wire(pin_i, Some(wire_i));
                            components.get(*another_comp).unwrap().set_pin_wire(*another_pin, Some(wire_i));
                            wires.push(wire);
                        }
                        None => {
                            pins_no_wire.insert(location, (comp_i, pin_i));
                        }
                    }
                }
            }
        }
        components.push(component);
    }

    (Circuit { components, wires, clock_generators },
            CanvasCircuit {
                components: canvas_components,
                wires: canvas_wires,
                circuit: circuit_idx,
                appearance: (),
                pins: (),
            })
}
