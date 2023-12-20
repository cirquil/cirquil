use crate::core::canvas::location::Location;
use crate::core::simulation::wire::WireIdx;

pub struct CanvasWire {
    pub segments: Vec<(Location, Location)>,
    pub wire: WireIdx
}