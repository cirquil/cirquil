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

fn dfs_wires(current: &Point, wires_map: &mut HashMap<Point, Vec<&wire::Wire>>, canvas_wire: &mut CanvasWire,
             used_locations: &mut Vec<Location>) {
    used_locations.push(Location(current.x as i16, current.y as i16));
    let wires = wires_map.remove(current).unwrap();
    for i in wires.iter() {
        let next;
        if *current == i.to {
            next = i.from;
        } else {
            next = i.to;
        }
        if wires_map.contains_key(&next) {
            canvas_wire.segments.push((Location(i.from.x as i16, i.from.y as i16),
                                       Location(i.to.x as i16, i.to.y as i16)));
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
            dfs_wires(&next, wires_map, canvas_wire, used_locations);
        }
    }
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
    let mut locations_map: HashMap<Location, WireIdx> = HashMap::new();
    let mut wire_index = 0;
    while !wires_map.is_empty() {
        let mut canvas_wire = CanvasWire { segments: Vec::new(), wire: wire_index };
        let mut used_locations: Vec<Location> = Vec::new();
        let begin = wires_map.keys().next().unwrap().clone();
        dfs_wires(&begin, &mut wires_map, &mut canvas_wire, &mut used_locations);
        wires.push(Wire {
            value: Cell::new(Default::default()),
            connected_components: Vec::new(),
        });
        canvas_wires.push(canvas_wire);
        for i in used_locations {
            locations_map.insert(i, wire_index);
        }
        wire_index += 1;
    }

    let mut canvas_components: Vec<CanvasComponent> = Vec::new();
    let mut components: Vec<Box<dyn Component>> = Vec::new();
    let mut clock_generators: Vec<usize> = Vec::new();
    let mut pins_locations_map: HashMap<Location, (ComponentIdx, PinIdx)> = HashMap::new();
    for (ci, c) in circuit.components.iter().enumerate() {
        let loc = Location(c.loc.x, c.loc.y);
        canvas_components.push(CanvasComponent { component: ci, loc });
        let comp: Box<dyn Component> = match (c.lib.as_str(), c.name.as_str()) {
            ("0", "Clock") => {
                clock_generators.push(ci);
                Box::new(ClockGenerator::create())
            }
            ("1", "OR Gate") => Box::new(OrGate::from_bit_width(1)),
            ("1", "AND Gate") => Box::new(AndGate::from_bit_width(1)),
            ("1", "NOT Gate") => Box::new(NotGate::from_bit_width(1)),
            _ => panic!("Unknown component {} from lib {}", c.name, c.lib),
        };
        for (pi, p) in comp.get_pins().iter().enumerate() {
            let location = loc + p.location;
            match locations_map.get(&location) {
                Some(wi) => {
                    comp.set_pin_wire(pi, Some(*wi));
                    wires.get_mut(*wi).unwrap().connected_components.push((ci, pi));
                }
                None => {
                    match pins_locations_map.get_mut(&location) {
                        Some((ai, ap)) => {
                            let wi = wires.len();
                            let wire = Wire {
                                value: Cell::new(Default::default()),
                                connected_components: vec![(*ai, *ap), (ci, pi)],
                            };
                            locations_map.insert(location, wi);
                            components.get(*ai).unwrap().set_pin_wire(*ap, Some(wi));
                            comp.set_pin_wire(pi, Some(wi));
                            wires.push(wire);
                        }
                        None => {
                            pins_locations_map.insert(location, (ci, pi));
                        }
                    }
                }
            }
        }
        components.push(comp);
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
    let circuit = Circuit {
        components: components,
        wires: wires,
        clock_generators: clock_generators,
    };
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