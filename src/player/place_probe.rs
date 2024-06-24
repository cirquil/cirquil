use egui::Pos2;

use crate::core::canvas::circuit::CanvasCircuit;
use crate::core::canvas::location::Location;
use crate::core::simulation::wire::WireIdx;

pub fn place_new_probe(interact_pos: Pos2,
                       canvas: &CanvasCircuit)
                       -> Option<(WireIdx, Location)> {
    const MARGIN: i16 = 15;
    let interact_pos = Location::from(interact_pos);

    let (wire, seg, _) = canvas.wires.iter()
        .filter_map(|wire| {
            wire.contains(interact_pos, MARGIN)
        })
        .min_by_key(|(_, _, dist)| *dist)?;

    let location = canvas.wires[wire].projection(seg, interact_pos);
    Some((wire, location))
}