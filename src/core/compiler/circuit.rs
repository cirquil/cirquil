use std::cell::Cell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::core::canvas::circuit::CanvasCircuit;
use crate::core::canvas::component::CanvasComponent;
use crate::core::canvas::location::Location;
use crate::core::canvas::wire::CanvasWire;
use crate::core::compiler::dfs::{dfs_wires, DfsComponents};
use crate::core::simulation::circuit::Circuit;
use crate::core::simulation::component::{Component, ComponentIdx, ComponentModel};
use crate::core::simulation::component::ComponentModel::ClockGenerator;
use crate::core::simulation::pin::Direction::{Input, Output};
use crate::core::simulation::pin::PinIdx;
use crate::core::simulation::wire::{Wire, WireIdx};
use crate::serde::project::{SavedCircuit, SavedComponent, SavedWire};

pub fn compile_circuit(saved_circuit: SavedCircuit) -> (Circuit, CanvasCircuit) {
    let mut wires_map: HashMap<Location, Vec<&SavedWire>> = HashMap::new();
    for i in saved_circuit.wires.iter() {
        match wires_map.get_mut(&i.start) {
            None => {
                wires_map.insert(i.start, vec![i]);
            }
            Some(v) => {
                v.push(i);
            }
        }
        match wires_map.get_mut(&i.end) {
            None => {
                wires_map.insert(i.end, vec![i]);
            }
            Some(v) => {
                v.push(i);
            }
        }
    }

    let mut dfs_components = DfsComponents::new(&saved_circuit.components);
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

    for (comp_i, SavedComponent { location: loc, component })
    in saved_circuit.components.into_iter().enumerate() {
        canvas_components.push(CanvasComponent { component: comp_i, loc });
        if let ClockGenerator(_) = component.component {
            clock_generators.push(comp_i);
        }

        for (pin_i, pin) in component.get_pins().iter().enumerate() {
            let location = loc + pin.location;
            match wire_nodes.entry(location) {
                Entry::Occupied(occ) => {
                    let &wire_i = occ.get();
                    component.set_pin_wire(pin_i, Some(wire_i));
                    wires.get_mut(wire_i).unwrap().connected_components.push((comp_i, pin_i));
                }
                Entry::Vacant(vac) => {
                    match pins_no_wire.get_mut(&location) {
                        Some((another_comp, another_pin)) => {
                            let wire_i = wires.len();
                            let wire = Wire {
                                value: Cell::new(Default::default()),
                                connected_components: vec![(*another_comp, *another_pin), (comp_i, pin_i)],
                            };
                            vac.insert(wire_i);
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

    let input_pins: Vec<(PinIdx, ComponentIdx)> = saved_circuit.pins.iter().enumerate()
        .filter(|(_, pin)| pin.direction == Input)
        .map(|(i, pin)| {
            let (comp_idx, _) = components.iter().enumerate()
                .filter(|(_, c)| matches!(c.component, ComponentModel::InputPin(_)))
                .find(|(_, c)| c.properties.get("label").unwrap().as_string().unwrap().get() == pin.label)
                .unwrap();

            (i, comp_idx)
        })
        .collect();

    let output_pins: Vec<(PinIdx, ComponentIdx)> = saved_circuit.pins.iter().enumerate()
        .filter(|(_, pin)| pin.direction == Output)
        .map(|(i, pin)| {
            let (comp_idx, _) = components.iter().enumerate()
                .filter(|(_, c)| matches!(c.component, ComponentModel::OutputPin(_)))
                .find(|(_, c)| c.properties.get("label").unwrap().as_string().unwrap().get() == pin.label)
                .unwrap();

            (i, comp_idx)
        })
        .collect();

    (Circuit {
        components,
        wires,
        clock_generators,
        input_pins,
        output_pins,
    },
     CanvasCircuit {
         components: canvas_components,
         wires: canvas_wires,
         appearance: (),
         pins: (),
     })
}
