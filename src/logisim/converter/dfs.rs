use std::collections::HashMap;
use std::ops::IndexMut;

use crate::core::canvas::location::Location;
use crate::logisim::parser::component::LogisimComponent;
use crate::logisim::parser::location::LogisimLocation;
use crate::logisim::parser::wire::LogisimWire;

pub struct DfsComponents {
    loc_to_tunnel: HashMap<LogisimLocation, Vec<usize>>,
    tunnel_to_loc: Vec<Option<Vec<LogisimLocation>>>,
}

impl DfsComponents {
    pub(crate) fn new(components: &[LogisimComponent]) -> Self {
        let mut tunnel_to_loc: Vec<Option<Vec<LogisimLocation>>> = Vec::new();
        let mut label_to_tunnel: HashMap<String, usize> = HashMap::new();
        for tun in components.iter()
            .filter(|x| x.lib == "0" && x.name == "Tunnel") {
            let label = tun.get_param("label").unwrap();
            match label_to_tunnel.get(label) {
                None => {
                    label_to_tunnel.insert(String::from(label), tunnel_to_loc.len());
                    tunnel_to_loc.push(Some(vec![tun.loc]));
                }
                Some(&idx) => {
                    tunnel_to_loc.index_mut(idx).as_mut().unwrap().push(tun.loc);
                }
            }
        };
        let mut loc_to_tunnel: HashMap<LogisimLocation, Vec<usize>> = HashMap::new();
        for (tun_i, v) in tunnel_to_loc.iter().enumerate() {
            for loc in v.as_ref().unwrap() {
                loc_to_tunnel.entry(*loc).or_default().push(tun_i);
            }
        }
        DfsComponents { loc_to_tunnel, tunnel_to_loc }
    }
}


pub fn dfs_wires(current: &LogisimLocation,
                 wires_map: &mut HashMap<LogisimLocation, Vec<&LogisimWire>>,
                 comps: &mut DfsComponents)
                 -> (Vec<(Location, Location)>, Vec<Location>) {
    let mut segments: Vec<(Location, Location)> = Vec::new();
    let mut circuit_nodes: Vec<Location> = Vec::new();
    dfs_wires_internal(current, wires_map,
                       &comps.loc_to_tunnel,
                       &mut comps.tunnel_to_loc,
                       &mut segments, &mut circuit_nodes);
    (segments, circuit_nodes)
}

fn dfs_wires_internal(current: &LogisimLocation,
                      wires_map: &mut HashMap<LogisimLocation, Vec<&LogisimWire>>,
                      loc_to_tunnel: &HashMap<LogisimLocation, Vec<usize>>,
                      tunnel_to_loc: &mut Vec<Option<Vec<LogisimLocation>>>,
                      segments: &mut Vec<(Location, Location)>,
                      circuit_nodes: &mut Vec<Location>) {
    let wires = wires_map.remove(current).unwrap();
    
    match wires.len() { 
        2 => {
            let first = if *current == wires[0].to {
                wires[0].from
            } else {
                wires[0].to
            };

            let second = if *current == wires[1].to {
                wires[1].from
            } else {
                wires[1].to
            };

            if first.x == second.x || first.y == second.y {
                circuit_nodes.push(Location::new(current.x, current.y));
            }
        }
        3.. => {
            circuit_nodes.push(Location::new(current.x, current.y));
        }
        _ => {}
    }
    
    for i in wires.iter() {
        let next = if *current == i.to {
            i.from
        } else {
            i.to
        };
        
        if wires_map.contains_key(&next) {
            segments.push((Location::new(i.from.x, i.from.y), Location::new(i.to.x, i.to.y)));
        }
    }
    for i in wires.iter() {
        let next= if *current == i.to {
            i.from
        } else {
            i.to
        };
        
        if wires_map.contains_key(&next) {
            dfs_wires_internal(&next, wires_map,
                               loc_to_tunnel,
                               tunnel_to_loc,
                               segments, circuit_nodes);
        }
    }
    match loc_to_tunnel.get(current) {
        None => {}
        Some(tunnels) => {
            for &tun in tunnels.iter() {
                tunnel_to_loc.push(None);
                match tunnel_to_loc.swap_remove(tun) {
                    None => {}
                    Some(locs) => {
                        for loc in locs.iter() {
                            if *loc != *current && wires_map.contains_key(loc) {
                                dfs_wires_internal(loc, wires_map,
                                                   loc_to_tunnel,
                                                   tunnel_to_loc,
                                                   segments, circuit_nodes);
                            }
                        }
                    }
                }
            }
        }
    }
}
