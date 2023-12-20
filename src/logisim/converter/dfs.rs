use std::collections::HashMap;
use crate::core::canvas::location::Location;

use crate::logisim::parser::location::LogisimLocation;
use crate::logisim::parser::wire::LogisimWire;

pub fn dfs_wires(current: &LogisimLocation, wires_map: &mut HashMap<LogisimLocation, Vec<&LogisimWire>>) -> Vec<(Location, Location)> {
    let mut segments: Vec<(Location, Location)> = Vec::new();
    dfs_wires_internal(current, wires_map, &mut segments);
    return segments;
}

fn dfs_wires_internal(current: &LogisimLocation, wires_map: &mut HashMap<LogisimLocation, Vec<&LogisimWire>>,
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
            dfs_wires_internal(&next, wires_map, segments);
        }
    }
}
