use std::cell::Cell;
use std::collections::HashMap;
use crate::core::canvas::{CanvasComponent, CanvasWire};
use crate::core::circuit::Circuit;
use crate::core::component::{Component, ComponentIdx};
use crate::core::components::clock_generator::ClockGenerator;
use crate::core::components::logic::and_gate::AndGate;
use crate::core::components::logic::not_gate::NotGate;
use crate::core::components::logic::or_gate::OrGate;
use crate::core::location::Location;
use crate::core::pin::PinIdx;
use crate::core::wire::{Wire, WireIdx};
use crate::logisim::converter::circuit::wire;
use crate::logisim::converter::circuit::point::Point;
use crate::logisim::converter::parse_logisim;

fn dfs_wires(current: &Point, wires_map: &mut HashMap<Point, Vec<&wire::Wire>>) -> Vec<(Location, Location)> {
    fn dfs_wires(current: &Point, wires_map: &mut HashMap<Point, Vec<&wire::Wire>>,
                 segments: &mut Vec<(Location, Location)>) {
        let wires = wires_map.remove(current).unwrap();
        for i in wires.iter() {
            let next;
            if *current == i.to {
                next = i.from;
            } else {
                next = i.to;
            }
            if wires_map.contains_key(&next) {
                segments.push((Location(i.from.x, i.from.y), Location(i.to.x, i.to.y)));
            }
        }
        for i in wires.iter() {
            let next;
            if *current == i.to {
                next = i.from;
            } else {
                next = i.to;
            }
            if wires_map.contains_key(&next) {
                dfs_wires(&next, wires_map, segments);
            }
        }
    }

    let mut segments: Vec<(Location, Location)> = Vec::new();
    dfs_wires(current, wires_map, &mut segments);
    return segments
}

pub fn test_canvas() {
    let parsed = parse_logisim("test.circ");
    let circuit = &parsed.circuits[0];

    let mut wires_map: HashMap<Point, Vec<&wire::Wire>> = HashMap::new();
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

    let mut canvas_wires: Vec<CanvasWire> = Vec::new();
    let mut wires: Vec<Wire> = Vec::new();
    let mut wire_nodes: HashMap<Location, WireIdx> = HashMap::new();
    let mut wire_index = 0;
    while !wires_map.is_empty() {
        let begin = wires_map.keys().next().unwrap().clone();
        let segments = dfs_wires(&begin, &mut wires_map);
        for (from, to) in segments.iter() {
            wire_nodes.insert(*from, wire_index);
            wire_nodes.insert(*to, wire_index);
        }
        let canvas_wire = CanvasWire { segments, wire: wire_index };
        wires.push(Wire {
            value: Cell::new(Default::default()),
            connected_components: Vec::new(),
        });
        canvas_wires.push(canvas_wire);
        wire_index += 1;
    }

    let mut canvas_components: Vec<CanvasComponent> = Vec::new();
    let mut components: Vec<Box<dyn Component>> = Vec::new();
    let mut clock_generators: Vec<usize> = Vec::new();
    let mut pins_no_wire: HashMap<Location, (ComponentIdx, PinIdx)> = HashMap::new();
    for (comp_i, parsed_comp) in circuit.components.iter().enumerate() {
        let loc = Location(parsed_comp.loc.x, parsed_comp.loc.y);
        canvas_components.push(CanvasComponent { component: comp_i, loc });
        let component: Box<dyn Component> = match (parsed_comp.lib.as_str(), parsed_comp.name.as_str()) {
            ("0", "Clock") => {
                clock_generators.push(comp_i);
                Box::new(ClockGenerator::create())
            }
            ("1", "OR Gate") => Box::new(OrGate::from_bit_width(1)),
            ("1", "AND Gate") => Box::new(AndGate::from_bit_width(1)),
            ("1", "NOT Gate") => Box::new(NotGate::from_bit_width(1)),
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


    // Model
    let or = OrGate::from_bit_width(1);
    let clock = ClockGenerator::create();

    let wire = Wire {
        value: Cell::new(Default::default()),
        connected_components: vec![
            (0, 0), (1, 0),
        ],
    };

    let display_wire = Wire {
        value: Cell::new(Default::default()),
        connected_components: vec![
            (1, 2)
        ],
    };

    clock.set_pin_wire(0, Some(0));

    or.set_pin_wire(0, Some(0));
    or.set_pin_wire(2, Some(1));


    println!("Hardcoded");
    let circuit = Circuit {
        components: vec![Box::new(clock), Box::new(or)],
        wires: vec![wire, display_wire],
        clock_generators: vec![0],
    };
    println!("{:?} {:?}", circuit.components, circuit.wires);
    circuit.propagate();
    println!("{:?} {:?}", circuit.components, circuit.wires);

    println!("From file");
    let circuit = Circuit { components, wires, clock_generators };
    println!("{:?} {:?}", circuit.components, circuit.wires);
    circuit.propagate();
    println!("{:?} {:?}", circuit.components, circuit.wires);
    circuit.propagate();
    println!("{:?} {:?}", circuit.components, circuit.wires);
    circuit.propagate();
    println!("{:?} {:?}", circuit.components, circuit.wires);
    circuit.propagate();
    println!("{:?} {:?}", circuit.components, circuit.wires);
    circuit.propagate();
    println!("{:?} {:?}", circuit.components, circuit.wires);
    circuit.propagate();
    println!("{:?} {:?}", circuit.components, circuit.wires);
    circuit.propagate();
    println!("{:?} {:?}", circuit.components, circuit.wires);
    circuit.propagate();
    println!("{:?} {:?}", circuit.components, circuit.wires);
}