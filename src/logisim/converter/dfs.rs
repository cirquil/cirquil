use std::collections::HashMap;
use crate::core::canvas::location::Location;

use crate::logisim::parser::location::LogisimLocation;
use crate::logisim::parser::wire::LogisimWire;

pub fn dfs_wires(current: &LogisimLocation, wires_map: &mut HashMap<LogisimLocation, Vec<&LogisimWire>>)
                 -> (Vec<(Location, Location)>, Vec<Location>) {
    let mut segments: Vec<(Location, Location)> = Vec::new();
    let mut circuit_nodes: Vec<Location> = Vec::new();
    dfs_wires_internal(current, wires_map, &mut segments, &mut circuit_nodes);
    return (segments, circuit_nodes);
}

fn dfs_wires_internal(current: &LogisimLocation, wires_map: &mut HashMap<LogisimLocation, Vec<&LogisimWire>>,
                      segments: &mut Vec<(Location, Location)>, circuit_nodes: &mut Vec<Location>) {
    let wires = wires_map.remove(current).unwrap();
    if wires.len() > 2 {
        circuit_nodes.push(Location::new(current.x, current.y));
    } else if wires.len() == 2 {
        let first;
        if *current == wires[0].to {
            first = wires[0].from;
        } else {
            first = wires[0].to;
        }
        let second;
        if *current == wires[1].to {
            second = wires[1].from;
        } else {
            second = wires[1].to;
        }
        if first.x == second.x || first.y == second.y {
            circuit_nodes.push(Location::new(current.x, current.y));
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
            segments.push((Location::new(i.from.x, i.from.y), Location::new(i.to.x, i.to.y)));
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
            dfs_wires_internal(&next, wires_map, segments, circuit_nodes);
        }
    }
}
