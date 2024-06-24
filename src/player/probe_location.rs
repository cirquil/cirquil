use egui::Pos2;

use crate::core::canvas::circuit::CanvasCircuit;
use crate::core::canvas::location::Location;
use crate::core::simulation::probe::CanvasProbe;
use crate::core::simulation::wire::WireIdx;

pub fn place_new_probe(interact_pos: Pos2,
                       canvas: &CanvasCircuit)
                       -> Option<(WireIdx, Location)> {
    const MARGIN: i16 = 15;
    let interact_pos = Location::from(interact_pos);

    let (wire, seg, _) = canvas.wires.iter()
        .filter_map(|wire| {
            wire.closest_segment(interact_pos, Some(MARGIN))
        })
        .min_by_key(|(_, _, dist)| *dist)?;

    let location = canvas.wires[wire].projection(seg, interact_pos);
    Some((wire, location))
}

pub fn fix_loaded_probe(probe: &mut CanvasProbe, canvas: &CanvasCircuit) {
    let wire = &canvas.wires[probe.probe.wire];
    if !wire.contains(probe.location) {
        if let Some((_, seg, _)) = wire.closest_segment(probe.location, None) {
            probe.location = wire.projection(seg, probe.location);
        }
    }
}