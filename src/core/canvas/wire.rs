use crate::core::canvas::location::Location;
use crate::core::simulation::wire::WireIdx;

pub struct CanvasWire {
    pub wire: WireIdx,
    pub segments: Vec<(Location, Location)>,
    pub nodes: Vec<Location>,
}